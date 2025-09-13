use crate::executor::Executor;
use crate::icmp_ping::IcmpPing;
use dns_lookup::lookup_host;
use log::{error, info, trace};
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
        let ip_addr = Self::parse_host(&host).unwrap();
        Self {
            ip_addr,
            timeout,
            icmp_ping: IcmpPing::new(),
        }
    }

    /// 解析主机的字符串成IP地址
    /// # 参数
    /// * `host` - 一个字符串切片，表示要解析的主机名或 IP 地址
    /// # 返回值
    /// 如果解析成功，则返回一个 `IpAddr`; 如果解析失败，则返回一个包含错误信息的 `String`
    /// ```
    fn parse_host(host: &str) -> Result<IpAddr, String> {
        // 尝试直接解析为 IP 地址
        if let Ok(ip_addr) = host.parse::<IpAddr>() {
            return Ok(ip_addr);
        }

        // 不是 IP，尝试 DNS 解析
        let mut addrs = lookup_host(host).map_err(|e| format!("DNS解析失败: {e}"))?;
        let ip_addr = addrs.next().ok_or_else(|| format!("无法解析主机名: {host}"))?;
        info!("解析主机名: {host} -> {}", ip_addr.to_string());
        Ok(ip_addr)
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
