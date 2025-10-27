use crate::executor::Executor;
use crate::ping_error::PingError;
use crate::task::tcp::tcp_ping::TcpPing;
use async_trait::async_trait;
use log::trace;
use std::net::SocketAddr;
use std::time::Duration;
use wheel_rs::dns_utils::parse_host_port;

#[derive(Clone)]
pub struct TcpExecutor {
    ip_addr: String,
    port: u16,
    tcp_ping: TcpPing,
    timeout: Duration,
}

impl TcpExecutor {
    /// # 构造函数
    /// ## 参数
    /// * `host_port` - 要ping的主机名及端口号
    /// * `timeout` - 一个 `Duration`，表示超时时间
    pub fn new(host_port: String, timeout: Duration) -> Self {
        // 解析主机的字符串成IP地址和端口号
        let (ip_addr, port) = parse_host_port(&host_port).unwrap();
        // 创建SocketAddr对象
        let socket_addr = SocketAddr::new(ip_addr, port);

        let tcp_ping = TcpPing::new(socket_addr);

        Self {
            ip_addr: ip_addr.to_string(),
            port,
            tcp_ping,
            timeout,
        }
    }
}

#[async_trait]
impl Executor for TcpExecutor {
    fn get_name(&self) -> String {
        String::from("TCP")
    }

    async fn exec(&self) -> Result<(), PingError> {
        trace!(
            "开始执行 TCP 任务: ping {}",
            format!("{}:{}", self.ip_addr, self.port)
        );
        self.tcp_ping.ping(self.timeout)
    }
}
