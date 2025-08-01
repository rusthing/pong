use crate::executor::Executor;
use dns_lookup::lookup_host;
use log::error;
use std::net::IpAddr;
use std::time::Duration;

#[derive(Copy, Clone)]
pub struct IcmpExecutor {
    ip_addr: IpAddr,
    timeout: Duration,
}

impl IcmpExecutor {
    /// 构造函数
    /// # 参数
    /// * `host` - 一个字符串切片，表示要解析的主机名或 IP 地址
    /// * `timeout` - 一个 `Duration`，表示超时时间
    pub fn new(host: String, timeout: Duration) -> Self {
        // 解析主机的字符串成IP地址
        let ip_addr = Self::parse_host(&host).unwrap();
        Self { ip_addr, timeout }
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
        addrs.pop().ok_or_else(|| format!("无法解析主机名: {host}"))
    }
}

impl Executor for IcmpExecutor {
    fn get_name(&self) -> String {
        String::from("ICMP")
    }

    fn exec(&self) -> Result<(), String> {
        ping::new(self.ip_addr)
            .timeout(self.timeout)
            .send()
            .map_err(|e| {
                let msg = format!("Ping失败: {e}");
                error!("{}", msg);
                msg
            })?;
        Ok(())
    }
}
