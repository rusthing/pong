use crate::executor::Executor;
use crate::ping_error::PingError;
use crate::task::http::http_ping::HttpPing;
use async_trait::async_trait;
use log::trace;
use std::time::Duration;

#[derive(Clone)]
pub struct HttpExecutor {
    urn: String,
    http_ping: HttpPing,
    timeout: Duration,
}

impl HttpExecutor {
    /// 构造函数
    /// # 参数
    /// * `urn` - 要请求的URN地址，格式为 `<method>:<url>`，例如: `GET:http://127.0.0.1:8080`
    /// * `timeout` - 一个 `Duration`，表示超时时间
    pub fn new(urn: String, timeout: Duration) -> Self {
        let http_ping = HttpPing::new(urn.clone());

        Self {
            http_ping,
            urn,
            timeout,
        }
    }
}

#[async_trait]
impl Executor for HttpExecutor {
    fn get_name(&self) -> String {
        String::from("HTTP")
    }

    async fn exec(&self) -> Result<(), PingError> {
        trace!("开始执行 HTTP 任务: ping {}", self.urn);
        self.http_ping.ping(self.timeout).await
    }
}
