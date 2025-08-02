use chrono::Local;
use clap::Parser;
use env_logger::Builder;
use log::{debug, info};
use pong::config::Config;
use pong::icmp_ping::IcmpPing;
use pong::prometheus_metrics::PrometheusMetrics;
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

    // IcmpPing::new()
    //     .ping("::1".parse().unwrap(), std::time::Duration::from_secs(1))
    //     .expect("ping error");
    // IcmpPing::new()
    //     .ping(
    //         "127.0.0.1".parse().unwrap(),
    //         std::time::Duration::from_secs(1),
    //     )
    //     .expect("ping error");
    // IcmpPing::new()
    //     .ping(
    //         "192.168.1.1".parse().unwrap(),
    //         std::time::Duration::from_secs(1),
    //     )
    //     .expect("ping error");
    // IcmpPing::new()
    //     .ping(
    //         "157.148.69.186".parse().unwrap(),
    //         std::time::Duration::from_secs(1),
    //     )
    //     .expect("ping error");
    // IcmpPing::new()
    //     .ping(
    //         "199.16.158.9".parse().unwrap(),
    //         std::time::Duration::from_secs(1),
    //     )
    //     .expect("ping error");
    // IcmpPing::new()
    //     .ping(
    //         "2001::1".parse().unwrap(),
    //         std::time::Duration::from_secs(1),
    //     )
    //     .expect("ping error");

    info!("程序正在启动……");

    debug!("解析命令行参数...");
    let args = Args::parse();

    debug!("加载配置文件...");
    let config = Config::new(args.config_file, args.port);

    debug!("创建PrometheusMetrics...");
    let prometheus_metrics = PrometheusMetrics::new();

    debug!("创建任务调度器...");
    let targets = Targets::new();
    Scheduler::new(targets.clone_tx()).start(config.task_groups);

    info!("创建Web服务器({:?})并运行...", config.web_server);
    WebServer::new(config.web_server, targets, prometheus_metrics)
        .run()
        .await;

    info!("退出程序");
    Ok(())
}
