use crate::cst::{
    HOST_ONLINE_PROMETHEUS_METRIC_DESC, HOST_ONLINE_PROMETHEUS_METRIC_LABEL_NAME,
    HOST_ONLINE_PROMETHEUS_METRIC_NAME,
};
use prometheus::proto::MetricFamily;
use prometheus::{IntGaugeVec, Registry, opts};

pub struct PrometheusMetrics {
    registry: Registry,
    host_online_gauges: IntGaugeVec,
}

impl PrometheusMetrics {
    /// 构造函数
    pub fn new() -> Self {
        // 定义带标签的GaugeVec
        let host_online_gauges = IntGaugeVec::new(
            opts!(
                HOST_ONLINE_PROMETHEUS_METRIC_NAME,
                HOST_ONLINE_PROMETHEUS_METRIC_DESC
            ),
            &[HOST_ONLINE_PROMETHEUS_METRIC_LABEL_NAME],
        )
        .unwrap();
        // 创建注册中心
        let registry = Registry::new();
        // 注册到注册表
        registry
            .register(Box::new(host_online_gauges.clone()))
            .unwrap();

        Self {
            registry,
            host_online_gauges,
        }
    }

    /// 更新指标
    pub fn update_metric(&self, host: String, is_online: bool) {
        self.host_online_gauges
            .with_label_values(&[host])
            .set(is_online as i64);
    }

    /// 获取指标集
    pub fn gather(&self) -> Vec<MetricFamily> {
        self.registry.gather()
    }
}
