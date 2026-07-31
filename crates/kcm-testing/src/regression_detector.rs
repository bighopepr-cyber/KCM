use std::collections::HashMap;

pub struct RegressionBaseline {
    pub metrics: HashMap<String, f64>,
    pub label: String,
}

pub struct RegressionAlert {
    pub metric: String,
    pub baseline_value: f64,
    pub current_value: f64,
    pub change_ratio: f64,
    pub severity: Severity,
}

impl RegressionAlert {
    pub fn to_report(&self) -> String {
        format!(
            "[{:?}] {}: {:.2} -> {:.2} ({:+.1}%)",
            self.severity,
            self.metric,
            self.baseline_value,
            self.current_value,
            self.change_ratio * 100.0,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

pub struct RegressionDetector {
    baselines: Vec<RegressionBaseline>,
    threshold: f64,
}

impl RegressionDetector {
    pub fn new() -> Self {
        RegressionDetector {
            baselines: Vec::new(),
            threshold: 0.05,
        }
    }

    pub fn with_threshold(mut self, threshold: f64) -> Self {
        self.threshold = threshold;
        self
    }

    pub fn load_baseline(&mut self, baseline: RegressionBaseline) {
        self.baselines.push(baseline);
    }

    pub fn detect(&self, current: &HashMap<String, f64>) -> Vec<RegressionAlert> {
        let mut alerts = Vec::new();
        if let Some(baseline) = self.baselines.last() {
            for (name, current_value) in current {
                if let Some(baseline_value) = baseline.metrics.get(name) {
                    if *baseline_value == 0.0 {
                        continue;
                    }
                    let change = (baseline_value - current_value).abs() / baseline_value.abs();
                    if change > self.threshold {
                        let severity = if change > 0.20 {
                            Severity::Critical
                        } else if change > 0.10 {
                            Severity::High
                        } else if change > 0.05 {
                            Severity::Medium
                        } else {
                            Severity::Low
                        };
                        alerts.push(RegressionAlert {
                            metric: name.clone(),
                            baseline_value: *baseline_value,
                            current_value: *current_value,
                            change_ratio: change,
                            severity,
                        });
                    }
                }
            }
        }
        alerts
    }

    pub fn baseline_count(&self) -> usize {
        self.baselines.len()
    }
}

impl Default for RegressionDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_baseline(metrics: Vec<(&str, f64)>) -> RegressionBaseline {
        RegressionBaseline {
            metrics: metrics
                .into_iter()
                .map(|(k, v)| (k.to_string(), v))
                .collect(),
            label: "v1".to_string(),
        }
    }

    #[test]
    fn test_no_regression() {
        let mut detector = RegressionDetector::new();
        detector.load_baseline(make_baseline(vec![("qps", 1000.0), ("latency", 50.0)]));
        let current = vec![("qps", 1010.0), ("latency", 49.5)]
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect();
        assert!(detector.detect(&current).is_empty());
    }

    #[test]
    fn test_detects_high_regression() {
        let mut detector = RegressionDetector::new();
        detector.load_baseline(make_baseline(vec![("qps", 1000.0)]));
        let current = vec![("qps", 800.0)]
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect();
        let alerts = detector.detect(&current);
        assert_eq!(alerts.len(), 1);
        assert!(matches!(
            alerts[0].severity,
            Severity::High | Severity::Critical
        ));
    }

    #[test]
    fn test_detects_critical_regression() {
        let mut detector = RegressionDetector::new();
        detector.load_baseline(make_baseline(vec![("qps", 1000.0)]));
        let current = vec![("qps", 500.0)]
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect();
        let alerts = detector.detect(&current);
        assert_eq!(alerts.len(), 1);
        assert_eq!(alerts[0].severity, Severity::Critical);
    }

    #[test]
    fn test_empty_baseline() {
        let detector = RegressionDetector::new();
        let current = vec![("qps", 1000.0)]
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect();
        assert!(detector.detect(&current).is_empty());
    }

    #[test]
    fn test_metric_not_in_baseline() {
        let mut detector = RegressionDetector::new();
        detector.load_baseline(make_baseline(vec![("qps", 1000.0)]));
        let current = vec![("new_metric", 500.0)]
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect();
        assert!(detector.detect(&current).is_empty());
    }

    #[test]
    fn test_zero_baseline_ignored() {
        let mut detector = RegressionDetector::new();
        detector.load_baseline(make_baseline(vec![("qps", 0.0)]));
        let current = vec![("qps", 100.0)]
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect();
        assert!(detector.detect(&current).is_empty());
    }

    #[test]
    fn test_alert_report() {
        let alert = RegressionAlert {
            metric: "throughput".to_string(),
            baseline_value: 1000.0,
            current_value: 800.0,
            change_ratio: 0.2,
            severity: Severity::Critical,
        };
        let report = alert.to_report();
        assert!(report.contains("throughput"));
        assert!(report.contains("Critical"));
    }

    #[test]
    fn test_multiple_metrics() {
        let mut detector = RegressionDetector::new();
        detector.load_baseline(make_baseline(vec![
            ("qps", 1000.0),
            ("latency_p99", 50.0),
            ("memory_mb", 100.0),
        ]));
        let current = vec![("qps", 950.0), ("latency_p99", 100.0), ("memory_mb", 101.0)]
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect();
        let alerts = detector.detect(&current);
        assert_eq!(alerts.len(), 1);
        assert_eq!(alerts[0].metric, "latency_p99");
    }

    #[test]
    fn test_baseline_evolution() {
        let mut detector = RegressionDetector::new();
        detector.load_baseline(make_baseline(vec![("qps", 1000.0)]));
        detector.load_baseline(make_baseline(vec![("qps", 1050.0)]));
        assert_eq!(detector.baseline_count(), 2);
        let current = vec![("qps", 1040.0)]
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect();
        assert!(detector.detect(&current).is_empty());
    }
}
