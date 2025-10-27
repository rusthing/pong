use thiserror::Error;

#[derive(Debug, Error)]
pub enum PingError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Timeout")]
    Timeout,
    #[error("Invalid reply: {0}")]
    InvalidReply(String),
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),
}
