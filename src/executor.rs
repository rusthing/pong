/// 执行器
pub trait Executor {
    /// 获取执行器的名称
    fn get_name(&self) -> String;

    /// 执行任务
    fn exec(&self) -> Result<(), String>;
}
