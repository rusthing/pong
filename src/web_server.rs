use crate::targets::Targets;
use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{App, HttpResponse, HttpServer, Responder, get};
use log::debug;
use prometheus::{Encoder, IntGaugeVec, Registry, TextEncoder, opts};

#[get("/metrics")]
async fn metrics(targets: Data<Targets>) -> impl Responder {
    debug!("receive http request: GET:/metrics");

    let encoder = TextEncoder::new();

    let registry = Registry::new();
    // 定义带标签的GaugeVec
    let http_requests =
        IntGaugeVec::new(opts!("pong_host_online", "Is the host online?"), &["host"]).unwrap();

    // 注册到注册表
    registry.register(Box::new(http_requests.clone())).unwrap();

    let statuses = targets.get_all();
    for (host, status) in statuses {
        // 更新不同标签的Gauge值
        http_requests
            .with_label_values(&[host])
            .set(status.is_online as i64);
    }

    // 收集指标数据
    let metric_families = registry.gather();

    // 编码为 Prometheus 文本格式
    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer).unwrap();

    // 返回响应
    HttpResponse::Ok().content_type("text/plain").body(buffer)
}
#[get("/health")]
async fn health() -> impl Responder {
    "Ok"
}

pub struct WebServer {
    server: Server,
}

impl WebServer {
    pub fn new(port: u16, targets: Targets) -> Self {
        let data = Data::new(targets);
        let server = HttpServer::new(move || {
            App::new()
                .app_data(data.clone())
                .service(metrics)
                .service(health)
        })
        .bind(("127.0.0.1", port))
        .unwrap()
        .bind(("::1", port))
        .unwrap()
        .run();
        Self { server }
    }

    pub async fn run(self) {
        self.server.await.expect("服务器启动失败");
    }
}
