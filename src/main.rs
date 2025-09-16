use chrono::Local;
use clap::Parser;
use env_logger::Builder;
use log::{debug, info};
use pong::config::{Config, CONFIG};
use pong::prometheus_metrics::PrometheusMetrics;
use pong::scheduler::Scheduler;
use pong::targets::Targets;
use pong::web_server::WebServer;
use std::io::Write;

/// 网络监控工具
///
/// SUMMARY: 这是一个用于网络监控的工具，可以监控各种网络目标并提供指标收集功能
///
#[derive(Parser, Debug)]
// 命令行参数使用定义
// version: 命令行添加 -V/--version参数可以查看版本信息
// about: --help命令第一行显示文档注释的内容
// long_about = None: 只显示文档注释的第一行(包括about的和arg的)
#[command(
    author = env!("CARGO_PKG_AUTHORS"),
    version,
    about,
    help_template = "{name} v{version} - {about}\n\nAUTHOR: {author}\n\nUSAGE: {usage}\n\nOPTIONS:\n{options}"
)]
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
        // 自定义日志格式
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

    debug!("解析命令行参数...");
    let args = Args::parse();

    debug!("加载配置文件...");
    let config = Config::new(args.config_file, args.port);
    CONFIG.set(config).expect("无法设置全局配置");

    debug!("创建PrometheusMetrics...");
    let prometheus_metrics = PrometheusMetrics::new();

    debug!("创建任务调度器...");
    let targets = Targets::new();
    Scheduler::new(targets.clone_tx()).start(CONFIG.get().unwrap().clone().task_groups);

    WebServer::new(targets, prometheus_metrics).run().await;

    info!("退出程序");
    Ok(())
}
