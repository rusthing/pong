use crate::duration_serde;
use log::info;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;
use std::time::Duration;
use std::{env, fs};
use strum_macros::Display;

/// 全局配置
pub static CONFIG: OnceLock<Config> = OnceLock::new();

/// 配置文件结构
#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
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
    /// tcp
    #[serde(rename = "tcp")]
    TCP,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskConfig {
    /// 任务类型
    pub task_type: TaskType,
    /// 目标
    pub target: String,
}

impl Config {
    /// 创建新的配置实例
    ///
    /// 该函数用于初始化应用程序配置，支持通过配置文件路径和端口参数来定制配置。
    /// 如果未提供配置文件路径，将尝试在可执行文件同目录下查找与包名同名的YAML配置文件。
    /// 如果提供了端口参数，将覆盖配置文件中的端口设置。
    ///
    /// # 参数
    /// * `path` - 可选的配置文件路径，如果为None则使用当前程序所在的目录
    /// * `port` - 可选的端口号，如果提供将覆盖配置文件中的端口设置
    ///
    /// # 返回值
    /// 返回解析后的Config实例
    ///
    /// # Panics
    /// 当配置文件读取失败或解析失败时会触发panic
    pub fn new(path: Option<String>, port: Option<u16>) -> Self {
        let path = path.unwrap_or_else(|| {
            // 如果未指定配置文件路径
            let mut exe_file_path = env::current_exe().expect("获取可执行文件路径失败");
            let config_file_name = concat!(env!("CARGO_PKG_NAME"), ".yml");
            exe_file_path.pop(); // 移除可执行文件名
            exe_file_path
                .join(&config_file_name)
                .to_string_lossy()
                .to_string()
        });

        let content = fs::read_to_string(path).expect("读取配置文件失败，可能是配置文件不存在");

        // 解析配置文件
        let mut config: Config = serde_yaml::from_str(content.as_str()).expect("解析配置文件失败");

        // 如果命令行指定了端口，则使用命令行指定的端口
        if port.is_some() {
            config.web_server.port = port;
        }

        info!("检查配置是否符合规范");
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
