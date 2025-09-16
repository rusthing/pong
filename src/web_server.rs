use crate::config::CONFIG;
use crate::prometheus_metrics::PrometheusMetrics;
use crate::targets::Targets;
use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use log::{debug, info};
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

pub struct WebServer {
    pub server: Server,
}

impl WebServer {
    pub fn new(targets: Targets, prometheus_metrics: PrometheusMetrics) -> Self {
        let web_server_config = CONFIG.get().unwrap().web_server.clone();
        info!("创建Web服务器({:?})并运行...", web_server_config);

        let targets_data = Data::new(targets);
        let prometheus_metrics_data = Data::new(prometheus_metrics);
        let port = web_server_config.port.unwrap();
        let mut server = HttpServer::new(move || {
            App::new()
                .app_data(targets_data.clone())
                .app_data(prometheus_metrics_data.clone())
                .service(metrics)
                .service(health)
        });

        for bind in web_server_config.bind {
            server = server.bind((bind, port)).unwrap();
        }

        let server = server.run();

        Self { server }
    }

    pub async fn run(self) {
        self.server.await.expect("服务器启动失败");
    }
}
