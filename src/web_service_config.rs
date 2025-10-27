use crate::metrics::prometheus_metrics::PrometheusMetrics;
use crate::scheduler::Scheduler;
use crate::settings::settings::SETTINGS;
use crate::targets::Targets;
use actix_web::web::Data;
use actix_web::{get, web, HttpResponse, Responder};
use log::debug;
use prometheus::{Encoder, TextEncoder};
use tokio::time::Instant;

/**
 * 获取指标
 */
#[get("/metrics")]
async fn metrics(
    targets: Data<Targets>,
    prometheus_metrics: Data<PrometheusMetrics>,
) -> impl Responder {
    debug!("接收到Http请求: GET:/metrics");

    let start_time = Instant::now();
    let statuses = targets.get_all();
    let elapsed = start_time.elapsed().as_millis();
    debug!("获取所有目标状态耗时: {}ms", elapsed);
    for (host, status) in statuses {
        // 更新不同标签的Gauge值
        prometheus_metrics.update_metric(host, status.elapsed);
    }

    // 收集指标数据
    let metric_families = prometheus_metrics.gather();

    // 编码为 Prometheus 文本格式
    let mut buffer = vec![];
    let encoder = TextEncoder::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();

    // 返回响应
    HttpResponse::Ok().content_type("text/plain").body(buffer)
}

#[get("/health")]
async fn health() -> impl Responder {
    "Ok"
}

/// # 配置WebService
pub fn web_service_config(cfg: &mut web::ServiceConfig) {
    debug!("创建PrometheusMetrics...");
    let prometheus_metrics = PrometheusMetrics::new();

    debug!("创建任务调度器...");
    let targets = Targets::new();
    Scheduler::new(targets.clone_tx()).start(SETTINGS.get().unwrap().clone().pong.task_groups);

    let targets_data = Data::new(targets);
    let prometheus_metrics_data = Data::new(prometheus_metrics);

    cfg.app_data(targets_data.clone())
        .app_data(prometheus_metrics_data.clone())
        .service(metrics) // 获取指标
        .service(health); // 健康检查
}
