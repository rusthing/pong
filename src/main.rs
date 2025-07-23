use chrono::Local;
use env_logger::Builder;
use log::{debug, info, trace};
use pong::Executor;
use pong::icmp::IcmpExecutor;
use std::io::Write;
use std::time::Instant;
use tokio::signal;
use tokio_cron_scheduler::{Job, JobScheduler};

#[tokio::main]
async fn main() {
    // 初始化日志实现库
    Builder::from_default_env()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                format!("{:<5}", record.level()),
                record.args()
            )
        })
        .init();

    info!("程序正在启动……");
    debug!("创建Icmp执行器");
    let _host = "www.google.com";
    let _host = "www.baidu.com";
    let executor: IcmpExecutor =
        IcmpExecutor::new(String::from(_host)).expect("Icmp执行器创建失败");

    let cron = "0/2 * * * * *";

    debug!("创建任务调度器");
    let mut scheduler = JobScheduler::new().await.expect("任务调度器创建失败");

    // 添加异步任务（每 cron 周期执行一次）
    scheduler
        .add(
            Job::new_async(cron, move |_uuid, _| {
                Box::pin(async move {
                    let start_time = Instant::now();
                    let executor_name = executor.get_name();
                    trace!("任务执行中: {}", executor_name);
                    executor
                        .exec()
                        .expect(&format!("任务执行失败: {}", executor_name));
                    let elapsed = start_time.elapsed().as_millis();
                    info!("Ping --> {} --> Pong in {} ms", executor_name, elapsed);
                })
            })
            .unwrap(),
        )
        .await
        .expect("任务添加失败");

    debug!("启动任务调度器");
    scheduler.start().await.expect("任务调度器启动失败");

    info!("程序已启动，按 Ctrl+C 退出...");
    // 等待 Ctrl+C 信号
    signal::ctrl_c().await.expect("监听 Ctrl+C 失败");

    debug!("监听到 Ctrl+C，关闭任务调度器...");

    // 优雅关闭调度器
    scheduler.shutdown().await.expect("任务调度器关闭失败");
    debug!("任务调度器已关闭");
    info!("退出程序");
}
