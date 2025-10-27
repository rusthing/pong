use crate::executor::Executor;
use crate::settings::pong_settings::{TaskGroupSettings, TaskType};
use crate::targets::TargetStatus;
use crate::task::http::http_executor::HttpExecutor;
use crate::task::icmp::icmp_executor::IcmpExecutor;
use crate::task::tcp::tcp_executor::TcpExecutor;
use log::{debug, error, info, trace};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::sync::mpsc::Sender;
use std::sync::Arc;
use tokio::time::{sleep, Instant};

/// 代表一个可执行的任务单元
#[derive(Clone)]
struct Task {
    /// 任务类型，目前支持 ICMP / TCP /HTTP
    task_type: TaskType,
    /// 目标地址，可以是 IP 地址或域名
    target: String,
    /// 用于发送目标状态更新的通道发送端
    target_status_tx: Sender<TargetStatus>,
    /// 执行器实例，根据任务类型确定具体的执行方式
    executor: Arc<dyn Executor + Send + Sync>,
}
/// 为 Task 结构体实现 Debug trait，用于调试时打印任务信息
impl Debug for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Task")
            .field("task_type", &self.task_type)
            .field("target", &self.target)
            .field("executor", &self.executor.get_name())
            .finish()
    }
}

/// 任务调度器，负责管理和执行不同类型的任务组
///
/// Scheduler 负责接收任务组配置，将其转换为可执行的任务，并按照指定的时间间隔
/// 循环执行这些任务。它支持多种任务类型，如 ICMP ping 和 TCP 连接测试。
///
/// 每个任务执行后会通过通道发送目标状态更新，以便其他组件可以监听和处理结果。
pub struct Scheduler {
    /// 目标状态的消息发送通道
    target_status_tx: Sender<TargetStatus>,
}

impl Scheduler {
    pub fn new(target_status_tx: Sender<TargetStatus>) -> Self {
        Self { target_status_tx }
    }

    pub fn start(&self, task_groups: Vec<TaskGroupSettings>) {
        debug!("启动任务调度器...");
        for task_group in task_groups.into_iter() {
            info!("添加任务组: {:?}", task_group);
            // 将配置中的任务转成要执行的任务
            let tasks: Arc<Vec<Task>> = Arc::new(
                task_group
                    .tasks
                    .iter()
                    .map(|task| Task {
                        task_type: task.task_type.clone(),
                        target: task.target.clone(),
                        target_status_tx: self.target_status_tx.clone(),
                        executor: match task.task_type {
                            TaskType::ICMP => Arc::new(IcmpExecutor::new(
                                task.target.clone(),
                                task_group.timeout.unwrap(),
                            )),
                            TaskType::TCP => Arc::new(TcpExecutor::new(
                                task.target.clone(),
                                task_group.timeout.unwrap(),
                            )),
                            TaskType::HTTP => Arc::new(HttpExecutor::new(
                                task.target.clone(),
                                task_group.timeout.unwrap(),
                            )),
                        },
                    })
                    .collect(),
            );
            let duration = task_group.interval.unwrap();
            let tasks_clone = Arc::clone(&tasks); // 克隆 Arc 以供异步任务使用
            tokio::spawn(async move {
                loop {
                    // let mut handles = vec![];
                    for task in tasks_clone.iter() {
                        // handles.push(tokio::spawn(Self::exec_task(task.clone())));
                        Self::exec_task(task.clone()).await;
                    }

                    // for handle in handles {
                    //     handle.await.expect("任务执行失败");
                    // }

                    sleep(duration).await;
                }
            });
        }
    }

    async fn exec_task(task: Task) {
        let start_time = Instant::now();
        let task_desc = format!("{:?}", task);
        let executor_name = task.executor.get_name().clone();
        trace!("开始执行任务: {}:{}", executor_name, task_desc);
        let elapsed = match task.executor.exec().await {
            Ok(_) => {
                let elapsed = start_time.elapsed().as_millis() as i64;
                info!(
                    "Ping {} --> {} --> Pong in {} ms",
                    executor_name, task.target, elapsed
                );
                elapsed
            }
            Err(e) => {
                error!(
                    "Ping {} --> {} --> Failed {}",
                    executor_name, task.target, e
                );
                -1
            }
        };

        let target_status = TargetStatus {
            task_type: task.task_type.clone(),
            target: task.target.clone(),
            elapsed,
        };
        trace!("更新目标状态: {:?}", target_status);
        task.target_status_tx.send(target_status).unwrap();
    }
}
