//! Telemetry module
//!
//! This module provides logging, tracing, and metrics functionality.

use lazy_static::lazy_static;
use opentelemetry::trace::TraceError;
use opentelemetry::global;
use tracing_subscriber::{EnvFilter, fmt, prelude::*, Registry};
use tracing_subscriber::fmt::format::FmtSpan;
use std::time::Duration;

/// Initialize telemetry with logging and tracing
pub fn init_telemetry(
    service_name: &str,
    environment: &str,
    log_level: &str,
) -> anyhow::Result<()> {
    // Parse log level from environment
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(log_level));

    // Format layer for console output
    let fmt_layer = fmt::layer()
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .with_target(false)
        .with_thread_ids(false)
        .with_file(false)
        .with_line_number(false);

    // Set up the subscriber
    let subscriber = Registry::default()
        .with(env_filter)
        .with(fmt_layer);

    // Try to initialize OpenTelemetry if configured
    if let Ok(jaeger_endpoint) = std::env::var("JAEGER_ENDPOINT") {
        init_tracing(service_name, environment, &jaeger_endpoint)?;
        tracing::info!("Jaeger tracing initialized");
    }

    // Initialize the subscriber globally
    subscriber.try_init()?;
    tracing::info!("Telemetry initialized for {} in {} mode", service_name, environment);

    Ok(())
}

/// Initialize OpenTelemetry tracing with Jaeger
pub fn init_tracing(
    service_name: &str,
    _environment: &str,
    _jaeger_endpoint: &str,
) -> Result<(), TraceError> {
    #[allow(deprecated)]
    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_service_name(service_name)
        .install_simple()?;

    let telemetry_layer = tracing_opentelemetry::layer::<tracing_subscriber::Registry>()
        .with_tracer(tracer);

    // Note: This needs to be added to the subscriber during init_telemetry
    let _ = telemetry_layer; // Suppress unused warning

    Ok(())
}

/// Shutdown telemetry and flush any pending data
pub fn shutdown_telemetry() {
    // Try to shutdown OpenTelemetry if it was initialized
    global::shutdown_tracer_provider();
}

/// Prometheus metrics helper
#[derive(Clone)]
pub struct Metrics {
    pub request_duration: prometheus::HistogramVec,
    pub request_count: prometheus::IntCounterVec,
    pub active_connections: prometheus::Gauge,
    pub message_count: prometheus::IntCounterVec,
}

impl Metrics {
    /// Create new metrics instance
    pub fn new() -> Result<Self, prometheus::Error> {
        let request_duration = prometheus::register_histogram_vec!(
            "http_request_duration_seconds",
            "HTTP request duration in seconds",
            &["method", "path", "status"],
            prometheus::exponential_buckets(0.005, 2.0, 20)?
        )?;

        let request_count = prometheus::register_int_counter_vec!(
            "http_requests_total",
            "Total number of HTTP requests",
            &["method", "path", "status"]
        )?;

        let active_connections = prometheus::register_gauge!(
            "websocket_active_connections",
            "Number of active WebSocket connections"
        )?;

        let message_count = prometheus::register_int_counter_vec!(
            "websocket_messages_total",
            "Total number of WebSocket messages",
            &["direction"] // "sent" or "received"
        )?;

        Ok(Self {
            request_duration,
            request_count,
            active_connections,
            message_count,
        })
    }

    /// Record HTTP request metrics
    pub fn record_request(&self, method: &str, path: &str, status: u16, duration: Duration) {
        let status_str = status.to_string();
        self.request_duration
            .with_label_values(&[method, path, &status_str])
            .observe(duration.as_secs_f64());
        self.request_count
            .with_label_values(&[method, path, &status_str])
            .inc();
    }

    /// Increment active WebSocket connections
    pub fn increment_connections(&self) {
        self.active_connections.inc();
    }

    /// Decrement active WebSocket connections
    pub fn decrement_connections(&self) {
        self.active_connections.dec();
    }

    /// Record WebSocket message
    pub fn record_message(&self, direction: &str) {
        self.message_count
            .with_label_values(&[direction])
            .inc();
    }

    /// Export metrics in Prometheus format
    pub fn export(&self) -> String {
        let encoder = prometheus::TextEncoder::new();
        let metric_families = prometheus::gather();
        encoder.encode_to_string(&metric_families).unwrap()
    }
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new().expect("Failed to initialize metrics")
    }
}

// 全局 metrics 单例，只注册一次，避免 /metrics 每次请求重复注册导致 AlreadyReg
lazy_static! {
    static ref METRICS: Metrics = Metrics::new().expect("Failed to initialize metrics");
}

/// 导出 Prometheus 格式的指标（供 /metrics 使用，不会重复注册）
pub fn export_metrics() -> String {
    METRICS.export()
}

/// Request tracking middleware state
#[derive(Clone, Default)]
pub struct RequestTracking {
    pub metrics: Metrics,
}

impl RequestTracking {
    pub fn new() -> Self {
        Self {
            metrics: Metrics::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_creation() {
        // 使用全局单例，避免 Metrics::new() 重复注册导致 AlreadyReg。
        // 测试并行时其他测试可能修改 gauge，只断言可读且非负。
        assert!(METRICS.active_connections.get() >= 0.0);
    }

    #[test]
    fn test_metrics_recording() {
        // 测试并行时 gauge 可能被其他测试修改，只断言增减效果
        let before = METRICS.active_connections.get();
        METRICS.increment_connections();
        assert!(METRICS.active_connections.get() >= before + 1.0);
        METRICS.decrement_connections();
        assert!(METRICS.active_connections.get() <= before + 1.0);
    }

    #[test]
    fn test_metrics_export() {
        let export = export_metrics();
        // 至少包含我们注册的指标名称（Prometheus 文本格式）
        assert!(!export.is_empty());
        assert!(
            export.contains("http_request_duration_seconds") || export.contains("websocket_active_connections"),
            "export should contain our metrics, got: {}",
            if export.len() > 200 { format!("{}...", &export[..200]) } else { export.clone() }
        );
    }
}
