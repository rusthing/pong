use log::{error, trace};
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PingError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub struct TcpPing {}
impl TcpPing {
    pub fn ping(socket_addr: &SocketAddr, timeout: Duration) -> Result<(), PingError> {
        trace!("ping {} ....", socket_addr);
        // 连接成功后断开
        TcpStream::connect_timeout(socket_addr, timeout)?.shutdown(std::net::Shutdown::Both)?;
        trace!("ping {} success", socket_addr);
        Ok(())
    }
}
