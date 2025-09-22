use crate::executor::Executor;
use crate::icmp_ping::IcmpPing;
use crate::tcp_utils::parse_host;
use log::{error, trace};
use std::net::IpAddr;
use std::time::Duration;

#[derive(Clone)]
pub struct IcmpExecutor {
    ip_addr: IpAddr,
    timeout: Duration,
    icmp_ping: IcmpPing,
}

impl IcmpExecutor {
    /// 构造函数
    /// # 参数
    /// * `host` - 一个字符串切片，表示要解析的主机名或 IP 地址
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

impl Executor for IcmpExecutor {
    fn get_name(&self) -> String {
        String::from("ICMP")
    }

    fn exec(&self) -> Result<(), String> {
        trace!("开始执行 ICMP 任务: ping {}", self.ip_addr);
        self.icmp_ping
            .ping(self.ip_addr, self.timeout)
            .map_err(|e| {
                let msg = format!("ping {} fail: {e}", self.ip_addr);
                error!("{}", msg);
                msg
            })
    }
}
