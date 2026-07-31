use std::collections::HashMap;
use std::time::Instant;

pub struct TestMetrics {
    pub total: u64,
    pub passed: u64,
    pub failed: u64,
    pub skipped: u64,
    pub execution_time_secs: f64,
}

impl TestMetrics {
    pub fn pass_rate(&self) -> f64 {
        if self.total == 0 {
            1.0
        } else {
            self.passed as f64 / self.total as f64
        }
    }

    pub fn to_report(&self) -> String {
        format!(
            "Tests: {} total, {} passed, {} failed, {} skipped\nPass Rate: {:.2}%\nExecution Time: {:.2}s",
            self.total,
            self.passed,
            self.failed,
            self.skipped,
            self.pass_rate() * 100.0,
            self.execution_time_secs,
        )
    }
}

pub struct PerformanceMetrics {
    pub metrics: HashMap<String, f64>,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl PerformanceMetrics {
    pub fn new() -> Self {
        PerformanceMetrics {
            metrics: HashMap::new(),
        }
    }

    pub fn record(&mut self, name: &str, value: f64) {
        self.metrics.insert(name.to_string(), value);
    }

    pub fn get(&self, name: &str) -> Option<f64> {
        self.metrics.get(name).copied()
    }

    pub fn to_report(&self) -> String {
        let mut lines = vec!["Performance Metrics:".to_string()];
        let mut sorted: Vec<_> = self.metrics.iter().collect();
        sorted.sort_by_key(|(k, _)| k.to_owned());
        for (name, value) in sorted {
            lines.push(format!("  {}: {:.2}", name, value));
        }
        lines.join("\n")
    }
}

pub struct MetricsCollector {
    test_metrics: HashMap<String, TestMetrics>,
    perf_metrics: HashMap<String, PerformanceMetrics>,
    start_time: Instant,
}

impl MetricsCollector {
    pub fn new() -> Self {
        MetricsCollector {
            test_metrics: HashMap::new(),
            perf_metrics: HashMap::new(),
            start_time: Instant::now(),
        }
    }

    pub fn record_test_suite(&mut self, name: &str, metrics: TestMetrics) {
        self.test_metrics.insert(name.to_string(), metrics);
    }

    pub fn record_performance(&mut self, name: &str, metrics: PerformanceMetrics) {
        self.perf_metrics.insert(name.to_string(), metrics);
    }

    pub fn total_tests(&self) -> (u64, u64, u64) {
        let mut total = 0u64;
        let mut passed = 0u64;
        let mut failed = 0u64;
        for m in self.test_metrics.values() {
            total += m.total;
            passed += m.passed;
            failed += m.failed;
        }
        (total, passed, failed)
    }

    pub fn overall_pass_rate(&self) -> f64 {
        let (total, passed, _) = self.total_tests();
        if total == 0 {
            1.0
        } else {
            passed as f64 / total as f64
        }
    }

    pub fn generate_report(&self) -> String {
        let elapsed = self.start_time.elapsed().as_secs_f64();
        let (total, passed, failed) = self.total_tests();
        let mut report = String::new();
        report.push_str("=== KCM Testing Metrics Dashboard ===\n\n");
        report.push_str(&format!("Build Duration: {:.1}s\n\n", elapsed));
        report.push_str(&format!("Total Tests: {}\n", total));
        report.push_str(&format!(
            "Passed: {} ({:.1}%)\n",
            passed,
            passed as f64 / total.max(1) as f64 * 100.0
        ));
        report.push_str(&format!(
            "Failed: {} ({:.1}%)\n\n",
            failed,
            failed as f64 / total.max(1) as f64 * 100.0
        ));
        report.push_str("--- Per Suite ---\n");
        for (name, metrics) in &self.test_metrics {
            report.push_str(&format!(
                "  {}: {}/{} ({:.1}%)\n",
                name,
                metrics.passed,
                metrics.total,
                metrics.pass_rate() * 100.0
            ));
        }
        if !self.perf_metrics.is_empty() {
            report.push_str("\n--- Performance ---\n");
            for (name, metrics) in &self.perf_metrics {
                report.push_str(&format!("  {}:\n{}\n", name, metrics.to_report()));
            }
        }
        report
    }
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_collector() {
        let mut collector = MetricsCollector::new();
        collector.record_test_suite(
            "unit",
            TestMetrics {
                total: 100,
                passed: 98,
                failed: 2,
                skipped: 0,
                execution_time_secs: 1.5,
            },
        );
        collector.record_test_suite(
            "integration",
            TestMetrics {
                total: 50,
                passed: 50,
                failed: 0,
                skipped: 0,
                execution_time_secs: 3.0,
            },
        );
        assert_eq!(collector.total_tests(), (150, 148, 2));
        assert!((collector.overall_pass_rate() - 148.0 / 150.0).abs() < 0.001);
        let report = collector.generate_report();
        assert!(report.contains("unit"));
        assert!(report.contains("integration"));
    }

    #[test]
    fn test_performance_metrics() {
        let mut perf = PerformanceMetrics::new();
        perf.record("query_latency_ms", 12.5);
        perf.record("throughput_qps", 5000.0);
        assert_eq!(perf.get("query_latency_ms"), Some(12.5));
        assert_eq!(perf.get("throughput_qps"), Some(5000.0));
        assert!(perf.get("nonexistent").is_none());
        let report = perf.to_report();
        assert!(report.contains("12.50"));
    }

    #[test]
    fn test_test_metrics_pass_rate() {
        let metrics = TestMetrics {
            total: 100,
            passed: 95,
            failed: 5,
            skipped: 0,
            execution_time_secs: 1.0,
        };
        assert!((metrics.pass_rate() - 0.95).abs() < 0.001);
        let zero = TestMetrics {
            total: 0,
            passed: 0,
            failed: 0,
            skipped: 0,
            execution_time_secs: 0.0,
        };
        assert_eq!(zero.pass_rate(), 1.0);
    }
}
