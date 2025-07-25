use crate::Executor;
use dns_lookup::lookup_host;
use std::net::IpAddr;

#[derive(Copy, Clone)]
pub struct IcmpExecutor {
    ip_addr: IpAddr,
}

impl IcmpExecutor {
    /// # 构造函数
    /// ## 参数
    /// * `host` - 一个字符串切片，表示要解析的主机名或 IP 地址
    pub fn new(host: String) -> Result<Self, String> {
        // 解析主机的字符串成IP地址
        let _ip_addr = Self::parse_host(&host)?;
        Ok(Self { ip_addr: _ip_addr })
    }

    /// # 解析主机的字符串成IP地址
    /// ## 参数
    /// * `host` - 一个字符串切片，表示要解析的主机名或 IP 地址
    /// ## 返回值
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
        self.ip_addr.to_string()
    }

    fn exec(&self) -> Result<(), String> {
        ping::new(self.ip_addr)
            .send()
            .map_err(|e| format!("Ping失败: {e}"))?;
        Ok(())
    }
}
