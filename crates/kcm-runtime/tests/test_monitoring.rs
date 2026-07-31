use kcm_runtime::health::*;
use kcm_runtime::logging::*;
use kcm_runtime::metrics::*;
use std::sync::Arc;

#[test]
fn test_metrics_record_query() {
    let metrics = Metrics::new();
    metrics.record_query(100, true);
    metrics.record_query(200, true);
    metrics.record_query(50, false);
    let snap = metrics.snapshot();
    assert_eq!(snap.queries_total, 3);
    assert_eq!(snap.queries_failed, 1);
    assert!((snap.avg_query_latency_ms - 116.67).abs() < 1.0);
}

#[test]
fn test_metrics_record_insert() {
    let metrics = Metrics::new();
    metrics.record_insert(true);
    metrics.record_insert(true);
    metrics.record_insert(false);
    let snap = metrics.snapshot();
    assert_eq!(snap.inserts_total, 3);
    assert_eq!(snap.inserts_failed, 1);
}

#[test]
fn test_metrics_cache_ratio() {
    let metrics = Metrics::new();
    for _ in 0..7 {
        metrics.record_cache_hit();
    }
    for _ in 0..3 {
        metrics.record_cache_miss();
    }
    let ratio = metrics.get_cache_hit_ratio();
    assert!((ratio - 0.7).abs() < 0.001);
}

#[test]
fn test_metrics_empty() {
    let metrics = Metrics::new();
    assert_eq!(metrics.get_avg_query_latency_ms(), 0.0);
    assert_eq!(metrics.get_cache_hit_ratio(), 0.0);
    assert_eq!(metrics.get_insert_error_rate(), 0.0);
}

#[test]
fn test_metrics_snapshot_json() {
    let metrics = Metrics::new();
    metrics.record_query(10, true);
    let json = metrics.snapshot().to_json();
    assert!(json.contains("queries_total"));
    assert!(json.contains("1"));
}

#[test]
fn test_health_check_healthy() {
    let metrics = Arc::new(Metrics::new());
    for _ in 0..10 {
        metrics.record_insert(true);
    }
    metrics.record_query(10, true);
    for _ in 0..10 {
        metrics.record_cache_hit();
    }
    let hc = HealthCheck::new(metrics);
    assert_eq!(hc.check(), HealthStatus::Healthy);
}

#[test]
fn test_health_check_unhealthy() {
    let metrics = Arc::new(Metrics::new());
    for _ in 0..10 {
        metrics.record_insert(false);
    }
    let hc = HealthCheck::with_error_threshold(HealthCheck::new(metrics), 0.01);
    assert_eq!(hc.check(), HealthStatus::Unhealthy);
}

#[test]
fn test_health_check_detailed() {
    let metrics = Arc::new(Metrics::new());
    metrics.record_query(10, true);
    metrics.record_insert(true);
    for _ in 0..5 {
        metrics.record_cache_hit();
    }
    let hc = HealthCheck::new(metrics);
    let report = hc.check_detailed();
    assert_eq!(report.status, HealthStatus::Healthy);
    assert!(report.total_queries > 0);
    let json = report.to_json();
    assert!(json.contains("healthy"));
}

#[test]
fn test_health_check_degraded_high_latency() {
    let metrics = Arc::new(Metrics::new());
    metrics.record_query(500, true);
    let hc = HealthCheck::with_latency_threshold_ms(HealthCheck::new(metrics), 10.0);
    assert_eq!(hc.check(), HealthStatus::Degraded);
}

#[test]
fn test_logging_level() {
    set_log_level(LogLevel::Warn);
    assert_eq!(get_log_level(), LogLevel::Warn);
    set_log_level(LogLevel::Debug);
    assert_eq!(get_log_level(), LogLevel::Debug);
}

#[test]
fn test_scoped_timer() {
    let duration_ms = Arc::new(std::sync::atomic::AtomicU64::new(0));
    {
        let _timer = ScopedTimer::new(duration_ms.clone());
    }
    let elapsed = duration_ms.load(std::sync::atomic::Ordering::Relaxed);
    assert!(
        elapsed < 10_000,
        "Timer should measure a reasonable duration"
    );
}
