use crate::executor::Executor;
use crate::ping_error::PingError;
use crate::task::icmp::icmp_ping::IcmpPing;
use async_trait::async_trait;
use log::trace;
use std::net::IpAddr;
use std::time::Duration;
use wheel_rs::dns_utils::parse_host;

#[derive(Clone)]
pub struct IcmpExecutor {
    ip_addr: IpAddr,
    timeout: Duration,
    icmp_ping: IcmpPing,
}

impl IcmpExecutor {
    /// 构造函数
    /// # 参数
    /// * `host` - 要ping的主机名或 IP 地址
    /// * `timeout` - 一个 `Duration`，表示超时时间
    pub fn new(host: String, timeout: Duration) -> Self {
        // 解析主机的字符串成IP地址
        let ip_addr = parse_host(&host).unwrap();
        Self {
            ip_addr,
            timeout,
            icmp_ping: IcmpPing::new(),
        }
    }
}

#[async_trait]
impl Executor for IcmpExecutor {
    fn get_name(&self) -> String {
        String::from("ICMP")
    }

    async fn exec(&self) -> Result<(), PingError> {
        trace!("开始执行 ICMP 任务: ping {}", self.ip_addr);
        self.icmp_ping.ping(self.ip_addr, self.timeout)
    }
}
