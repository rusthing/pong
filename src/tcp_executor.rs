use crate::executor::Executor;
use crate::tcp_ping::TcpPing;
use log::{error, trace};
use std::net::SocketAddr;
use std::time::Duration;

#[derive(Clone)]
pub struct TcpExecutor {
    socket_addr: SocketAddr,
    timeout: Duration,
}

impl TcpExecutor {
    /// 构造函数
    /// # 参数
    /// * `socket_addr` - 一个字符串切片，表示要解析的主机名或 IP 地址
    /// * `timeout` - 一个 `Duration`，表示超时时间
    pub fn new(socket_addr: String, timeout: Duration) -> Self {
        // 解析主机的字符串成IP地址
        let socket_addr = socket_addr.parse::<SocketAddr>().unwrap();

        Self {
            socket_addr,
            timeout,
        }
    }
}

impl Executor for TcpExecutor {
    fn get_name(&self) -> String {
        String::from("TCP")
    }

    fn exec(&self) -> Result<(), String> {
        trace!("开始执行 ICMP 任务: ping {}", self.socket_addr);
        TcpPing::ping(&self.socket_addr, self.timeout).map_err(|e| {
            let msg = format!("ping {} fail: {e}", self.socket_addr);
            error!("{}", msg);
            msg
        })
    }
}
