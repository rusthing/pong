use pong::Executor;
use pong::icmp::IcmpExecutor;
use std::time::Instant;

fn main() {
    let _host = "www.google.com";
    let _host = "www.baidu.com";
    let executor: IcmpExecutor =
        IcmpExecutor::new(String::from(_host)).expect("Icmp执行器创建失败");
    let start_time = Instant::now();
    executor.exec().expect("Icmp执行器执行失败");
    let elapsed = start_time.elapsed().as_millis();
    println!("执行完成，耗时: {} ms", elapsed);
}
