use crate::config::WebServerConfig;
use crate::prometheus_metrics::PrometheusMetrics;
use crate::targets::Targets;
use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use log::debug;
use prometheus::{Encoder, TextEncoder};
use tokio::time::Instant;

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
        prometheus_metrics.update_metric(host, status.is_online);
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

pub struct WebServer {
    pub server: Server,
}

impl WebServer {
    pub fn new(
        web_server_config: WebServerConfig,
        targets: Targets,
        prometheus_metrics: PrometheusMetrics,
    ) -> Self {
        let targets_data = Data::new(targets);
        let prometheus_metrics_data = Data::new(prometheus_metrics);
        let mut server = HttpServer::new(move || {
            App::new()
                .app_data(targets_data.clone())
                .app_data(prometheus_metrics_data.clone())
                .service(metrics)
                .service(health)
        });

        for bind in web_server_config.bind {
            let port = web_server_config.port.unwrap();
            server = server.bind((bind, port)).unwrap();
        }

        Self {
            server: server.run(),
        }
    }

    pub async fn run(self) {
        self.server.await.expect("服务器启动失败");
    }
}
