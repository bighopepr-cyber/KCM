use crate::metrics::Metrics;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

impl HealthStatus {
    pub fn to_json(&self) -> &str {
        match self {
            HealthStatus::Healthy => r#"{"status":"healthy"}"#,
            HealthStatus::Degraded => r#"{"status":"degraded"}"#,
            HealthStatus::Unhealthy => r#"{"status":"unhealthy"}"#,
        }
    }
}

pub struct HealthCheck {
    metrics: Arc<Metrics>,
    error_threshold: f64,
    cache_hit_threshold: f64,
    latency_threshold_ms: f64,
}

impl HealthCheck {
    pub fn new(metrics: Arc<Metrics>) -> Self {
        HealthCheck {
            metrics,
            error_threshold: 0.05,
            cache_hit_threshold: 0.5,
            latency_threshold_ms: 100.0,
        }
    }

    pub fn with_error_threshold(mut self, threshold: f64) -> Self {
        self.error_threshold = threshold;
        self
    }

    pub fn with_latency_threshold_ms(mut self, threshold: f64) -> Self {
        self.latency_threshold_ms = threshold;
        self
    }

    pub fn with_cache_hit_threshold(mut self, threshold: f64) -> Self {
        self.cache_hit_threshold = threshold;
        self
    }

    pub fn check(&self) -> HealthStatus {
        let snap = self.metrics.snapshot();

        let insert_error_rate = if snap.inserts_total > 0 {
            snap.inserts_failed as f64 / snap.inserts_total as f64
        } else {
            0.0
        };
        let query_error_rate = if snap.queries_total > 0 {
            snap.queries_failed as f64 / snap.queries_total as f64
        } else {
            0.0
        };

        if insert_error_rate > self.error_threshold || query_error_rate > self.error_threshold {
            return HealthStatus::Unhealthy;
        }

        if snap.avg_query_latency_ms > self.latency_threshold_ms {
            return HealthStatus::Degraded;
        }

        if snap.queries_total > 0 && snap.cache_hit_ratio < self.cache_hit_threshold {
            return HealthStatus::Degraded;
        }

        HealthStatus::Healthy
    }

    pub fn check_detailed(&self) -> HealthReport {
        let status = self.check();
        let snap = self.metrics.snapshot();

        HealthReport {
            status,
            avg_query_latency_ms: snap.avg_query_latency_ms,
            cache_hit_ratio: snap.cache_hit_ratio,
            insert_error_rate: if snap.inserts_total > 0 {
                snap.inserts_failed as f64 / snap.inserts_total as f64
            } else {
                0.0
            },
            query_error_rate: if snap.queries_total > 0 {
                snap.queries_failed as f64 / snap.queries_total as f64
            } else {
                0.0
            },
            total_queries: snap.queries_total,
            total_inserts: snap.inserts_total,
        }
    }
}

#[derive(Debug, Clone)]
pub struct HealthReport {
    pub status: HealthStatus,
    pub avg_query_latency_ms: f64,
    pub cache_hit_ratio: f64,
    pub insert_error_rate: f64,
    pub query_error_rate: f64,
    pub total_queries: u64,
    pub total_inserts: u64,
}

impl HealthReport {
    pub fn to_json(&self) -> String {
        format!(
            r#"{{"status":"{}","avg_query_latency_ms":{:.2},"cache_hit_ratio":{:.4},"insert_error_rate":{:.4},"query_error_rate":{:.4},"total_queries":{},"total_inserts":{}}}"#,
            match self.status {
                HealthStatus::Healthy => "healthy",
                HealthStatus::Degraded => "degraded",
                HealthStatus::Unhealthy => "unhealthy",
            },
            self.avg_query_latency_ms,
            self.cache_hit_ratio,
            self.insert_error_rate,
            self.query_error_rate,
            self.total_queries,
            self.total_inserts,
        )
    }
}
