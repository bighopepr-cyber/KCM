use std::collections::HashMap;

pub struct ConfidenceLearner {
    fact_sources: HashMap<String, Vec<f64>>,
    rule_accuracy: HashMap<u32, f64>,
}

impl ConfidenceLearner {
    pub fn new() -> Self {
        ConfidenceLearner {
            fact_sources: HashMap::new(),
            rule_accuracy: HashMap::new(),
        }
    }

    pub fn observe_fact(&mut self, fact_key: String, confidence: f64, is_correct: bool) {
        self.fact_sources
            .entry(fact_key)
            .or_default()
            .push(if is_correct { confidence } else { -confidence });
    }

    pub fn observe_rule_inference(&mut self, rule_id: u32, predicted: f64, actual: f64) {
        let error = (predicted - actual).abs();
        self.rule_accuracy
            .entry(rule_id)
            .and_modify(|acc| {
                *acc = 0.9 * *acc + 0.1 * (1.0 - error);
            })
            .or_insert(1.0 - error);
    }

    pub fn predict_confidence(&self, fact_key: &str) -> Option<f64> {
        self.fact_sources.get(fact_key).map(|c| {
            let avg = c.iter().sum::<f64>() / c.len() as f64;
            avg.clamp(0.0, 1.0)
        })
    }

    pub fn get_rule_accuracy(&self, rule_id: u32) -> f64 {
        self.rule_accuracy.get(&rule_id).copied().unwrap_or(0.5)
    }

    pub fn adjust_confidence(&self, rule_id: u32, base: f64) -> f64 {
        (base * self.get_rule_accuracy(rule_id)).clamp(0.0, 1.0)
    }

    pub fn rules_tracked(&self) -> usize {
        self.rule_accuracy.len()
    }
}

impl Default for ConfidenceLearner {
    fn default() -> Self {
        Self::new()
    }
}
