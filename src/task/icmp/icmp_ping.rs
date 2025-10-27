use crate::ping_error::PingError;
use log::trace;
use socket2::{Domain, Protocol, SockAddr, Socket, Type};
use std::mem::MaybeUninit;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::sync::atomic::{AtomicU16, Ordering};
use std::time::Duration;

/// ICMPv4 头部长度
const ICMP_V4_HEADER_LENGTH: usize = 20;

/// 计算 ICMP 校验和（RFC 1071）
/// 按 16 位分组累加，处理奇数长度数据，最后取反
fn checksum_v4(buf: &[u8]) -> u16 {
    let mut sum: u32 = 0;
    for chunk in buf.chunks(2) {
        let word = u16::from_be_bytes([chunk[0], chunk.get(1).copied().unwrap_or(0)]);
        sum = sum.wrapping_add(word as u32);
    }
    while (sum >> 16) != 0 {
        sum = (sum & 0xffff) + (sum >> 16);
    }
    !(sum as u16)
}

/// 构造 ICMPv4 Echo Request 包
fn build_icmp_v4_echo(id: u16, seq: u16) -> Vec<u8> {
    let mut buf = vec![0; 8];
    buf[0] = 8; // type = Echo Request
    buf[1] = 0; // code = 0
    buf[4..6].copy_from_slice(&id.to_be_bytes());
    buf[6..8].copy_from_slice(&seq.to_be_bytes());
    let checksum = checksum_v4(&buf);
    buf[2..4].copy_from_slice(&checksum.to_be_bytes());
    buf
}

/// 构造 ICMPv6 Echo Request 包
fn build_icmp_v6_echo(id: u16, seq: u16) -> Vec<u8> {
    let mut buf = vec![0; 8];
    buf[0] = 128; // type = Echo Request
    buf[1] = 0; // code = 0
    buf[4..6].copy_from_slice(&id.to_be_bytes());
    buf[6..8].copy_from_slice(&seq.to_be_bytes());
    // ICMPv6 的校验和计算需包含伪头部（IPv6源/目的地址等），但操作系统内核通常自动处理，此处省略
    buf
}

pub struct IcmpPing {
    id: u16,
    seq: AtomicU16,
}

// 手动实现 Clone（可选）
impl Clone for IcmpPing {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            seq: AtomicU16::new(self.seq.load(Ordering::Relaxed)),
        }
    }
}

impl IcmpPing {
    pub fn new() -> Self {
        Self {
            // XXX u16范围是 0~65,535，PID 最大值默认为 32,767(通过 /proc/sys/kernel/pid_max 可调整)
            id: std::process::id() as u16,
            seq: 0.into(),
        }
    }

    pub fn ping(&self, dst_ip: IpAddr, timeout: Duration) -> Result<(), PingError> {
        trace!("ping {} ....", dst_ip);
        let (domain, protocol, src_ip) = match dst_ip {
            IpAddr::V4(_) => (
                Domain::IPV4,
                Protocol::ICMPV4,
                IpAddr::V4(Ipv4Addr::UNSPECIFIED),
            ),
            IpAddr::V6(_) => (
                Domain::IPV6,
                Protocol::ICMPV6,
                IpAddr::V6(Ipv6Addr::UNSPECIFIED),
            ),
        };

        // 创建原始套接字
        let sock = Socket::new(domain, Type::RAW, Some(protocol))?;
        sock.set_read_timeout(Some(timeout))?;
        sock.set_write_timeout(Some(timeout))?;

        // 绑定到本地 0.0.0.0 / :: 让内核选源地址
        let src_addr = SockAddr::from(SocketAddr::new(src_ip, 0));
        sock.bind(&src_addr)?;

        let dst_addr = SockAddr::from(SocketAddr::new(dst_ip, 0));
        // 序列号使用 u16，在达到最大值后会自然回绕，这符合 ICMP 协议规范
        let seq = self.seq.fetch_add(1, Ordering::Relaxed).wrapping_add(1);

        // 构造并发送
        let packet = match dst_ip {
            IpAddr::V4(_) => build_icmp_v4_echo(self.id, seq),
            IpAddr::V6(_) => build_icmp_v6_echo(self.id, seq),
        };
        sock.send_to(&packet, &dst_addr)?;

        // 接收
        let mut buf: [MaybeUninit<u8>; 1024] = unsafe { MaybeUninit::uninit().assume_init() };
        let (len, _) = sock.recv_from(&mut buf)?;

        // 校验回包
        let reply = unsafe { std::slice::from_raw_parts(buf.as_ptr() as *const u8, len) };
        match dst_ip {
            IpAddr::V4(_) => {
                if reply.len() != 28 {
                    return Err(PingError::InvalidReply(format!(
                        "ICMPv4 reply length expect 28 bytes but {} bytes",
                        reply.len()
                    )));
                }
                if &reply[(ICMP_V4_HEADER_LENGTH + 4)..(ICMP_V4_HEADER_LENGTH + 8)] != &packet[4..8]
                {
                    return Err(PingError::InvalidReply(format!(
                        "ICMPv4 reply data is not correct! send-{:?} received-{:?}",
                        packet, reply
                    )));
                }
            }
            IpAddr::V6(_) => {
                if reply.len() != 8 {
                    return Err(PingError::InvalidReply(format!(
                        "ICMPv6 reply length expect 8 bytes but {} bytes",
                        reply.len(),
                    )));
                }
                if &reply[4..8] != &packet[4..8] {
                    return Err(PingError::InvalidReply(format!(
                        "ICMPv6 reply data is not correct! send-{:?} received-{:?}",
                        packet, reply
                    )));
                }
            }
        }

        trace!("ping {} success", dst_ip);
        Ok(())
    }
}
