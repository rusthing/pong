use crate::duration_serde;
use log::info;
use serde::{Deserialize, Serialize};
use std::fs;
use std::time::Duration;
use strum_macros::Display;

/// 配置文件结构
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// Web服务器的端口号
    pub port: Option<u16>,
    /// 任务列表
    #[serde(rename = "taskGroups")]
    pub task_groups: Vec<TaskGroupConfig>,
}

/// 任务类型
#[derive(Debug, Serialize, Deserialize, Display, Clone)]
pub enum TaskType {
    /// ping
    #[serde(rename = "icmp")]
    Icmp,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskGroupConfig {
    /// 任务组执行间隔
    #[serde(with = "duration_serde", default = "interval_default")]
    pub interval: Option<Duration>,
    /// 超时时间
    #[serde(with = "duration_serde", default = "timeout_default")]
    pub timeout: Option<Duration>,
    /// 任务列表
    pub tasks: Vec<TaskConfig>,
}

fn interval_default() -> Option<Duration> {
    Some(Duration::from_secs(3)) // 默认 3 秒
}
fn timeout_default() -> Option<Duration> {
    Some(Duration::from_secs(5)) // 默认 5 秒
}

/// 任务属性
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskConfig {
    /// 任务类型
    #[serde(rename = "taskType")]
    pub task_type: TaskType,
    /// 目标
    pub target: String,
}

impl Config {
    pub fn new(path: Option<String>, port: Option<u16>) -> Self {
        // 读取配置文件内容
        let content = if path.is_some() {
            fs::read_to_string(path.unwrap()).expect("读取配置文件失败")
        } else {
            let path = String::from("pong.yml");
            fs::read_to_string(path).expect("读取配置文件失败")
        };

        // 解析配置文件
        let mut config: Config = serde_yaml::from_str(content.as_str()).expect("解析配置文件失败");
        // 如果命令行指定了端口，则使用命令行指定的端口
        if port.is_some() {
            config.port = port;
        }
        // 如果配置文件中没有指定端口，则使用默认端口6780
        else if config.port.is_none() {
            config.port = Some(6780);
        }

        info!("检查配置文件的配置是否符合规范");
        if config.task_groups.is_empty() {
            panic!("配置文件中没有配置任务组");
        }

        for task_group in config.task_groups.iter_mut() {
            if task_group.tasks.is_empty() {
                panic!("配置文件中任务组没有配置任务");
            }
        }

        config
    }
}
