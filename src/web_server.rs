use crate::targets::Targets;
use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{App, HttpResponse, HttpServer, Responder, get};

#[get("/metrics")]
async fn metrics(targets: Data<Targets>) -> impl Responder {
    HttpResponse::Ok().json(targets.get_all())
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
