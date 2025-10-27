use crate::ping_error::PingError;
use log::trace;
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

#[derive(Clone)]
pub struct TcpPing {
    socket_addr: SocketAddr,
}
impl TcpPing {
    pub fn new(socket_addr: SocketAddr) -> Self {
        TcpPing { socket_addr }
    }

    pub fn ping(&self, timeout: Duration) -> Result<(), PingError> {
        trace!("ping {} ....", self.socket_addr);
        // 连接成功后断开
        TcpStream::connect_timeout(&self.socket_addr, timeout)?
            .shutdown(std::net::Shutdown::Both)?;
        trace!("ping {} success", self.socket_addr);
        Ok(())
    }
}
