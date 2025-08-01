use crate::duration_serde;
use log::info;
use serde::{Deserialize, Serialize};
use std::fs;
use std::time::Duration;
use strum_macros::Display;

/// 配置文件结构
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// Web服务器
    #[serde(default = "web_server_default")]
    pub web_server: WebServerConfig,

    /// 任务列表
    pub task_groups: Vec<TaskGroupConfig>,
}

fn web_server_default() -> WebServerConfig {
    WebServerConfig {
        bind: bind_default(),
        port: port_default(),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebServerConfig {
    /// 绑定的IP地址
    #[serde(default = "bind_default")]
    pub bind: Vec<String>,
    /// Web服务器的端口号
    #[serde(default = "port_default")]
    pub port: Option<u16>,
}

fn bind_default() -> Vec<String> {
    vec![String::from("0.0.0.0")]
}

fn port_default() -> Option<u16> {
    Some(6780)
}

/// 任务类型
#[derive(Debug, Serialize, Deserialize, Display, Clone)]
pub enum TaskType {
    /// icmp
    #[serde(rename = "icmp")]
    ICMP,
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
            config.web_server.port = port;
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
