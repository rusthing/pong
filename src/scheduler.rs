use crate::config::{TaskGroupConfig, TaskType};
use crate::executor::Executor;
use crate::icmp_executor::IcmpExecutor;
use crate::targets::TargetStatus;
use log::{debug, info, trace};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use std::sync::mpsc::Sender;
use tokio::time::{Instant, sleep};

#[derive(Clone)]
struct Task {
    task_type: TaskType,
    target: String,
    target_status_tx: Sender<TargetStatus>,
    executor: Arc<dyn Executor + Send + Sync>,
}

impl Debug for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Task")
            .field("task_type", &self.task_type)
            .field("target", &self.target)
            .field("executor", &self.executor.get_name())
            .finish()
    }
}

pub struct Scheduler {
    /// 目标状态的消息发送通道
    target_status_tx: Sender<TargetStatus>,
}

impl Scheduler {
    pub fn new(target_status_tx: Sender<TargetStatus>) -> Self {
        Self { target_status_tx }
    }

    pub fn start(&self, task_groups: Vec<TaskGroupConfig>) {
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
                        executor: Arc::new(IcmpExecutor::new(
                            task.target.clone(),
                            task_group.timeout.unwrap(),
                        )),
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
                        Self::exec_task(task.clone());
                    }

                    // for handle in handles {
                    //     handle.await.expect("任务执行失败");
                    // }

                    sleep(duration).await;
                }
            });
        }
    }

    fn exec_task(task: Task) {
        let start_time = Instant::now();
        let task_desc = format!("{:?}", task);
        let executor_name = task.executor.get_name().clone();
        trace!("开始执行任务: {}:{}", executor_name, task_desc);
        let elapsed = match task.executor.exec() {
            Ok(_) => {
                let elapsed = start_time.elapsed().as_millis() as i64;
                info!(
                    "Ping {} --> {} --> Pong in {} ms",
                    executor_name, task.target, elapsed
                );
                elapsed
            }
            Err(_) => -1,
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
