pub mod icmp;

pub trait ExecutorConfig {}

pub trait Executor {
    fn exec(&self) -> Result<(), String>;
}
