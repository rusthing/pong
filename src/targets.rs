use crate::config::TaskType;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

/// 目标状态
#[derive(Serialize, Clone, Debug)]
pub struct TargetStatus {
    /// 任务类型
    pub task_type: TaskType,
    /// 目标
    pub target: String,
    /// 耗时
    pub elapsed: i64,
}

/// 目标管理
pub struct Targets {
    /// 发送通道
    tx: mpsc::Sender<TargetStatus>,
    /// 目标状态
    statuses: Arc<Mutex<HashMap<String, TargetStatus>>>,
}

/// 目标管理
impl Targets {
    /// 构造函数
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel::<TargetStatus>();
        let statuses = Arc::new(Mutex::new(HashMap::<String, TargetStatus>::new()));
        let statuses_clone = statuses.clone();
        thread::spawn(move || {
            loop {
                {
                    let new_status = rx.recv().unwrap();
                    let key = Self::calc_key(&new_status.task_type, &new_status.target);
                    let mut statuses = statuses_clone.lock().unwrap();
                    let old_status = statuses.get(&key);
                    if old_status.is_none() || old_status.unwrap().elapsed != new_status.elapsed {
                        statuses.insert(key, new_status);
                    }
                }
                sleep(Duration::from_secs(1)); // 注意要放在作用域外，先释放锁
            }
        });
        Self { tx, statuses }
    }

    pub fn calc_key(task_type: &TaskType, target: &str) -> String {
        format!("{} {}", task_type, target)
    }

    /// 克隆发送通道
    pub fn clone_tx(&self) -> mpsc::Sender<TargetStatus> {
        self.tx.clone()
    }

    /// 添加获取所有目标状态的方法
    pub fn get_all(&self) -> HashMap<String, TargetStatus> {
        self.statuses.lock().unwrap().clone()
    }
}
