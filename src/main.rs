use actix_web::{App, HttpServer, Responder, get, web};
use chrono::Local;
use clap::Parser;
use env_logger::Builder;
use log::{debug, info, trace};
use pong::Executor;
use pong::config::Config;
use pong::icmp::IcmpExecutor;
use std::io::Write;
use std::time::Instant;
use tokio::signal;
use tokio_cron_scheduler::{Job, JobScheduler};

fn ping(executor: &dyn Executor) {
    let start_time = Instant::now();
    let executor_name = executor.get_name();
    trace!("任务执行中: {}", executor_name);
    executor
        .exec()
        .expect(&format!("任务执行失败: {}", executor_name));
    let elapsed = start_time.elapsed().as_millis();
    info!("Ping --> {} --> Pong in {} ms", executor_name, elapsed);
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 配置文件的路径
    #[arg(short, long)]
    config_file: Option<String>,

    /// Web服务器的端口号
    #[arg(short, long)]
    port: Option<u16>,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
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

    debug!("解析命令行参数");
    let args = Args::parse();

    debug!("加载配置文件");
    let config: Config = Config::from_file(args.config_file, args.port);

    debug!("开始执行任务: {}", config.port.is_some());
    debug!("创建Icmp执行器");
    let _host = "www.google.com";
    let _host = "www.baidu.com";
    let executor: IcmpExecutor =
        IcmpExecutor::new(String::from(_host)).expect("Icmp执行器创建失败");

    // let cron = "0/2 * * * * *";

    // debug!("创建任务调度器");
    // let mut scheduler = JobScheduler::new().await.expect("任务调度器创建失败");
    //
    // // 添加异步任务（每 cron 周期执行一次）
    // scheduler
    //     .add(
    //         Job::new_async(cron, move |_uuid, _| {
    //             Box::pin(async move {
    //                 let start_time = Instant::now();
    //                 let executor_name = executor.get_name();
    //                 trace!("任务执行中: {}", executor_name);
    //                 executor
    //                     .exec()
    //                     .expect(&format!("任务执行失败: {}", executor_name));
    //                 let elapsed = start_time.elapsed().as_millis();
    //                 info!("Ping --> {} --> Pong in {} ms", executor_name, elapsed);
    //             })
    //         })
    //         .unwrap(),
    //     )
    //     .await
    //     .expect("任务添加失败");
    //
    // debug!("启动任务调度器");
    // scheduler.start().await.expect("任务调度器启动失败");
    //
    // info!("程序已启动，按 Ctrl+C 退出...");
    // // 等待 Ctrl+C 信号
    // signal::ctrl_c().await.expect("监听 Ctrl+C 失败");
    //
    // debug!("监听到 Ctrl+C，关闭任务调度器...");
    //
    // // 优雅关闭调度器
    // scheduler.shutdown().await.expect("任务调度器关闭失败");
    // debug!("任务调度器已关闭");

    HttpServer::new(|| App::new().service(greet))
        .bind(("127.0.0.1", config.port.unwrap()))?
        .bind(("::1", config.port.unwrap()))?
        .run()
        .await

    // info!("退出程序");
}
