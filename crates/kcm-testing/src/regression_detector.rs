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
}

impl Default for RegressionDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_regression() {
        let mut detector = RegressionDetector::new();
        let baseline = RegressionBaseline {
            metrics: {
                let mut m = HashMap::new();
                m.insert("qps".to_string(), 1000.0);
                m
            },
            label: "v1".to_string(),
        };
        detector.load_baseline(baseline);
        let current = {
            let mut m = HashMap::new();
            m.insert("qps".to_string(), 1010.0);
            m
        };
        assert!(detector.detect(&current).is_empty());
    }

    #[test]
    fn test_detects_regression() {
        let mut detector = RegressionDetector::new();
        let baseline = RegressionBaseline {
            metrics: {
                let mut m = HashMap::new();
                m.insert("qps".to_string(), 1000.0);
                m
            },
            label: "v1".to_string(),
        };
        detector.load_baseline(baseline);
        let current = {
            let mut m = HashMap::new();
            m.insert("qps".to_string(), 800.0);
            m
        };
        let alerts = detector.detect(&current);
        assert_eq!(alerts.len(), 1);
        assert!(matches!(
            alerts[0].severity,
            Severity::High | Severity::Critical
        ));
    }

    #[test]
    fn test_empty_baseline() {
        let detector = RegressionDetector::new();
        let current = {
            let mut m = HashMap::new();
            m.insert("qps".to_string(), 1000.0);
            m
        };
        assert!(detector.detect(&current).is_empty());
    }
}
