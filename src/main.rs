use actix_web::{App, HttpServer, Responder, get, web};
use chrono::Local;
use clap::Parser;
use env_logger::Builder;
use log::{debug, info, trace};
use pong::config::Config;
use pong::executor::Executor;
use pong::icmp::IcmpExecutor;
use pong::scheduler::Scheduler;
use std::io::Write;
use std::time::Instant;
use tokio_cron_scheduler::{Job, JobScheduler};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

/// 命令行参数
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
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(7);

    // println!("Before calling closure: {list:?}");    // 错误！
    borrows_mutably();
    list.push(8);
    println!("After calling closure: {list:?}");

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
    let config: Config = Config::new(args.config_file, args.port);

    let scheduler = Scheduler::new().await;
    scheduler.start(config.tasks).await;

    HttpServer::new(|| App::new().service(greet))
        .bind(("127.0.0.1", config.port.unwrap()))?
        .bind(("::1", config.port.unwrap()))?
        .run()
        .await
        .expect("服务器启动失败");

    info!("退出程序");
    Ok(())
}
