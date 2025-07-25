use serde::{Deserialize, Serialize};
use std::fs;

/// 配置文件结构
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// Web服务器的端口号
    pub port: Option<u16>,
    /// 任务列表
    pub tasks: Vec<TaskProperty>,
}

/// 任务类型
#[derive(Debug, Serialize, Deserialize)]
pub enum TaskType {
    /// ping
    Ping,
    /// http
    Http,
}

/// 任务属性
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskProperty {
    /// 任务类型
    pub task_type: TaskType,
    /// 目标
    pub target: String,
    /// 定时cron表达式
    pub cron: String,
}

impl Config {
    pub fn from_file(path: Option<String>, port: Option<u16>) -> Self {
        let content = if path.is_some() {
            fs::read_to_string(path.unwrap()).expect("读取配置文件失败")
        } else {
            let path = String::from("pong.yml");
            fs::read_to_string(path).expect("读取配置文件失败")
        };

        let mut config: Config = serde_yaml::from_str(content.as_str()).expect("解析配置文件失败");
        if port.is_some() {
            config.port = port;
        }
        if config.port.is_none() {
            config.port = Some(6780);
        }
        config
    }
}
