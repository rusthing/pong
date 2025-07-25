use crate::config::TaskConfig;
use crate::executor::Executor;
use crate::icmp::IcmpExecutor;
use log::{debug, info, trace};
use std::sync::Arc;
use tokio::time::Instant;
use tokio_cron_scheduler::{Job, JobScheduler};

pub struct Scheduler {
    scheduler: JobScheduler,
}

impl Scheduler {
    pub async fn new() -> Self {
        debug!("创建任务调度器");
        let scheduler = JobScheduler::new().await.expect("任务调度器创建失败");
        Scheduler { scheduler }
    }

    pub async fn start(&self, tasks: Vec<TaskConfig>) {
        debug!("启动任务调度器");
        for task in tasks.iter() {
            debug!("创建Icmp执行器");
            // let executor: Arc<dyn Executor + Send + Sync> =
            //     Arc::new(IcmpExecutor::new(task.target.clone()).expect("Icmp执行器创建失败"));
            //
            // debug!("添加任务: {:?}", task);
            // self.scheduler
            //     .add(
            //         Job::new_async(task.cron.clone(), move |_uuid, _| {
            //             Box::pin(async move {
            //                 let start_time = Instant::now();
            //                 let executor_name = executor.get_name();
            //                 trace!("开始执行任务: {}", executor_name);
            //                 // executor
            //                 //     .exec()
            //                 //     .expect(&format!("任务执行失败: {}", executor_name));
            //                 let elapsed = start_time.elapsed().as_millis();
            //                 info!("Ping --> {} --> Pong in {} ms", executor_name, elapsed);
            //             })
            //         })
            //         .unwrap(),
            //     )
            //     .await
            //     .expect("任务添加失败");
        }

        // debug!("启动任务调度器");
        self.scheduler.start().await.expect("任务调度器启动失败");
    }

    pub async fn stop(&mut self) {
        debug!("停止任务调度器");
        self.scheduler.shutdown().await.expect("任务调度器停止失败");
    }
}
