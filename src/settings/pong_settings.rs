use serde::{Deserialize, Serialize};
use std::time::Duration;
use strum_macros::Display;
use wheel_rs::serde::duration_option_serde;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct PongSettings {
    /// 任务列表
    pub task_groups: Vec<TaskGroupSettings>,
}

/// 任务分组配置，定义了一组相关任务的执行参数
///
/// 该结构体用于配置任务组的基本属性，包括执行间隔、超时时间和任务列表。
/// 所有字段都支持序列化和反序列化，便于从配置文件中读取和保存。
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskGroupSettings {
    /// 任务组执行间隔
    #[serde(with = "duration_option_serde", default = "interval_default")]
    pub interval: Option<Duration>,
    /// 超时时间
    #[serde(with = "duration_option_serde", default = "timeout_default")]
    pub timeout: Option<Duration>,
    /// 任务列表
    pub tasks: Vec<TaskSettings>,
}

fn interval_default() -> Option<Duration> {
    Some(Duration::from_secs(3)) // 默认 3 秒
}
fn timeout_default() -> Option<Duration> {
    Some(Duration::from_secs(5)) // 默认 5 秒
}

/// 任务类型枚举，定义了支持的任务类型
///
/// 该枚举包含了三种任务类型：
/// - ICMP: 用于网络连通性测试
/// - TCP: 用于TCP端口连通性测试
/// - HTTP: 用于HTTP服务可用性测试
#[derive(Debug, Serialize, Deserialize, Display, Clone)]
pub enum TaskType {
    /// icmp
    #[serde(rename = "icmp")]
    ICMP,
    /// tcp
    #[serde(rename = "tcp")]
    TCP,
    /// http
    #[serde(rename = "http")]
    HTTP,
}

/// 任务属性
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct TaskSettings {
    /// 任务类型
    pub task_type: TaskType,
    /// 目标
    pub target: String,
}
