use crate::cst::{
    HOST_ACTIVITY_PROMETHEUS_METRIC_DESC, HOST_ACTIVITY_PROMETHEUS_METRIC_LABEL_NAME,
    HOST_ACTIVITY_PROMETHEUS_METRIC_NAME,
};
use prometheus::proto::MetricFamily;
use prometheus::{opts, IntGaugeVec, Registry};

pub struct PrometheusMetrics {
    registry: Registry,
    host_activity_gauges: IntGaugeVec,
}

impl PrometheusMetrics {
    /// 构造函数
    pub fn new() -> Self {
        // 定义带标签的GaugeVec
        let host_activity_gauges = IntGaugeVec::new(
            opts!(
                HOST_ACTIVITY_PROMETHEUS_METRIC_NAME,
                HOST_ACTIVITY_PROMETHEUS_METRIC_DESC
            ),
            &[HOST_ACTIVITY_PROMETHEUS_METRIC_LABEL_NAME],
        )
        .unwrap();
        // 创建注册中心
        let registry = Registry::new();
        // 注册到注册表
        registry
            .register(Box::new(host_activity_gauges.clone()))
            .unwrap();

        Self {
            registry,
            host_activity_gauges,
        }
    }

    /// 更新指标
    /// # 参数
    /// `host` - 主机名
    /// `elapsed` - 耗时
    pub fn update_metric(&self, host: String, elapsed: u128) {
        self.host_activity_gauges
            .with_label_values(&[host])
            .set(elapsed as i64);
    }

    /// 获取指标集
    pub fn gather(&self) -> Vec<MetricFamily> {
        self.registry.gather()
    }
}
