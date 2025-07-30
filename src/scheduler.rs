use crate::config::TaskConfig;
use crate::executor::Executor;
use crate::icmp::IcmpExecutor;
use log::{debug, info, trace};
use std::sync::Arc;
use tokio::time::Instant;
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};

pub struct Scheduler {
    scheduler: JobScheduler,
}

impl Scheduler {
    pub async fn new() -> Result<Self, JobSchedulerError> {
        debug!("创建任务调度器");
        let scheduler = JobScheduler::new().await?;
        Ok(Self { scheduler })
    }

    pub async fn start(&self, tasks: Vec<TaskConfig>) -> Result<(), JobSchedulerError> {
        debug!("启动任务调度器");
        for task in tasks.iter() {
            let task_target = task.target.clone();
            let task_desc = format!("{:?}", task);
            info!("添加任务: {}", task_desc);
            debug!("创建Icmp执行器");
            let executor: Arc<dyn Executor + Send + Sync> = Arc::new(
                IcmpExecutor::new(task.target.clone(), task.timeout.unwrap())
                    .expect("Icmp执行器创建失败"),
            );

            self.scheduler
                .add(Job::new_async(task.cron.clone(), move |_uuid, _| {
                    let task_target = task_target.clone();
                    let task_desc = task_desc.clone();
                    let executor_name = executor.get_name().clone();
                    let executor = executor.clone();
                    Box::pin({
                        async move {
                            let start_time = Instant::now();
                            trace!("开始执行任务: {}:{}", executor_name, task_desc);
                            executor
                                .exec()
                                .expect(&format!("任务执行失败: {}:{}", executor_name, task_desc));
                            let elapsed = start_time.elapsed().as_millis();
                            info!(
                                "Ping {} --> {} --> Pong in {} ms",
                                executor_name, task_target, elapsed
                            );
                        }
                    })
                })?)
                .await?;
        }

        debug!("启动任务调度器");
        self.scheduler.start().await
    }

    pub async fn stop(&mut self) {
        debug!("停止任务调度器");
        self.scheduler.shutdown().await.expect("任务调度器停止失败");
    }
}
