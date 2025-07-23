pub mod icmp;

pub trait ExecutorConfig {}

pub trait Executor {
    fn get_name(&self) -> String;

    fn exec(&self) -> Result<(), String>;
}
