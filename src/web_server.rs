use crate::targets::Targets;
use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{get, App, HttpServer, Responder};

#[get("/hello")]
async fn greet(targets: Data<Targets>) -> impl Responder {
    format!("Hello {:?}!", targets.get_all())
}

pub struct WebServer {
    server: Server,
}

impl WebServer {
    pub fn new(port: u16, targets: Targets) -> Self {
        let data = Data::new(targets);
        let server = HttpServer::new(move || App::new().app_data(data.clone()).service(greet))
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
