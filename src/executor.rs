use crate::ping_error::PingError;
use async_trait::async_trait;

/// 执行器
#[async_trait]
pub trait Executor {
    /// 获取执行器的名称
    fn get_name(&self) -> String;

    /// 执行任务
    async fn exec(&self) -> Result<(), PingError>;
}
