use chrono::Local;
use clap::Parser;
use env_logger::Builder;
use log::{debug, info};
use pong::config::Config;
use pong::scheduler::Scheduler;
use pong::targets::Targets;
use pong::web_server::WebServer;
use std::io::Write;

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
    let config = Config::new(args.config_file, args.port);

    debug!("创建任务调度器");
    let targets = Targets::new();
    Scheduler::new(targets.clone_tx())
        .await
        .expect("创建任务调度器失败")
        .start(config.tasks)
        .await
        .expect("启动任务调度器失败");

    WebServer::new(config.port.unwrap(), targets).run().await;

    info!("退出程序");
    Ok(())
}
