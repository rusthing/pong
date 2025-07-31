use crate::config::TaskConfig;
use crate::executor::Executor;
use crate::icmp_executor::IcmpExecutor;
use crate::targets::TargetStatus;
use log::{debug, info, trace};
use std::sync::{Arc, mpsc};
use tokio::time::Instant;
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};

pub struct Scheduler {
    scheduler: JobScheduler,
    tx: mpsc::Sender<TargetStatus>,
}

impl Scheduler {
    pub async fn new(tx: mpsc::Sender<TargetStatus>) -> Result<Self, JobSchedulerError> {
        let scheduler = JobScheduler::new().await?;
        Ok(Self { scheduler, tx })
    }

    pub async fn start(&self, tasks: Vec<TaskConfig>) -> Result<(), JobSchedulerError> {
        debug!("启动任务调度器...");
        for task in tasks.iter() {
            let task_desc = format!("{:?}", task);
            info!("添加任务: {}", task_desc);
            let task_target = task.target.clone();
            let task_type = task.task_type.clone();
            debug!("创建Icmp执行器...");
            let executor: Arc<dyn Executor + Send + Sync> = Arc::new(
                IcmpExecutor::new(task.target.clone(), task.timeout.unwrap())
                    .expect("Icmp执行器创建失败"),
            );

            let tx = self.tx.clone();

            self.scheduler
                .add(Job::new(task.cron.clone(), move |_uuid, _schedule| {
                    let start_time = Instant::now();
                    let task_target = task_target.clone();
                    let task_type = task_type.clone();
                    let task_desc = task_desc.clone();
                    let executor_name = executor.get_name().clone();
                    // let executor = executor.clone();
                    let tx = tx.clone();
                    trace!("开始执行任务: {}:{}", executor_name, task_desc);

                    tx.send(TargetStatus {
                        task_type,
                        target: task_target.clone(),
                        is_online: match executor.exec() {
                            Ok(_result) => true,
                            Err(_err) => false,
                        },
                    })
                    .expect("发送消息异常");

                    let elapsed = start_time.elapsed().as_millis();
                    info!(
                        "Ping {} --> {} --> Pong in {} ms",
                        executor_name, task_target, elapsed
                    );

                    // Box::pin({
                    //     async move {
                    //         let start_time = Instant::now();
                    //         trace!("开始执行任务: {}:{}", executor_name, task_desc);
                    //
                    //         tx.send(TargetStatus {
                    //             task_type,
                    //             target: task_target.clone(),
                    //             is_online: match executor.exec() {
                    //                 Ok(_result) => true,
                    //                 Err(_err) => false,
                    //             },
                    //         })
                    //         .expect("发送消息异常");
                    //
                    //         let elapsed = start_time.elapsed().as_millis();
                    //         info!(
                    //             "Ping {} --> {} --> Pong in {} ms",
                    //             executor_name, task_target, elapsed
                    //         );
                    //     }
                    // })
                })?)
                .await?;
        }

        debug!("启动任务调度器...");
        self.scheduler.start().await
    }

    pub async fn stop(&mut self) {
        debug!("停止任务调度器...");
        self.scheduler.shutdown().await.expect("任务调度器停止失败");
    }
}
