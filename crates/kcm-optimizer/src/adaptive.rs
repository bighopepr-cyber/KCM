use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

pub struct ExecutionStats {
    pub actual_rows: usize,
    pub actual_time_ms: u64,
    pub estimated_rows: usize,
    pub estimated_time_ms: u64,
}

impl ExecutionStats {
    pub fn row_error_ratio(&self) -> f64 {
        if self.estimated_rows == 0 {
            if self.actual_rows == 0 {
                0.0
            } else {
                f64::INFINITY
            }
        } else {
            ((self.actual_rows as f64) / (self.estimated_rows as f64) - 1.0).abs()
        }
    }
}

struct ExecutionRecord {
    query_hash: u64,
    predicted_rows: usize,
    actual_rows: usize,
    predicted_cost: f64,
    actual_cost: f64,
}

pub struct AdaptiveExecutor {
    history: Arc<Mutex<VecDeque<ExecutionRecord>>>,
    max_history: usize,
    reoptimize_threshold: f64,
}

impl AdaptiveExecutor {
    pub fn new() -> Self {
        AdaptiveExecutor {
            history: Arc::new(Mutex::new(VecDeque::new())),
            max_history: 10_000,
            reoptimize_threshold: 0.5,
        }
    }

    pub fn with_threshold(mut self, threshold: f64) -> Self {
        self.reoptimize_threshold = threshold;
        self
    }

    pub fn record(
        &self,
        query_hash: u64,
        predicted_rows: usize,
        actual_rows: usize,
        predicted_cost: f64,
        actual_cost: f64,
    ) {
        let mut history = self.history.lock().unwrap_or_else(|e| e.into_inner());
        history.push_back(ExecutionRecord {
            query_hash,
            predicted_rows,
            actual_rows,
            predicted_cost,
            actual_cost,
        });
        if history.len() > self.max_history {
            history.pop_front();
        }
    }

    pub fn should_reoptimize(&self, error_ratio: f64) -> bool {
        error_ratio > self.reoptimize_threshold
    }

    pub fn cardinality_correction_factor(&self, query_hash: u64) -> f64 {
        let history = self.history.lock().unwrap_or_else(|e| e.into_inner());
        let relevant: Vec<&ExecutionRecord> = history
            .iter()
            .filter(|r| r.query_hash == query_hash)
            .collect();

        if relevant.is_empty() {
            return 1.0;
        }

        let total_ratio: f64 = relevant
            .iter()
            .map(|r| {
                if r.predicted_rows == 0 {
                    1.0
                } else {
                    r.actual_rows as f64 / r.predicted_rows as f64
                }
            })
            .sum();

        total_ratio / relevant.len() as f64
    }

    pub fn history_size(&self) -> usize {
        self.history.lock().unwrap_or_else(|e| e.into_inner()).len()
    }

    pub fn average_cost_error(&self) -> f64 {
        let history = self.history.lock().unwrap_or_else(|e| e.into_inner());
        if history.is_empty() {
            return 0.0;
        }
        let total: f64 = history
            .iter()
            .map(|r| {
                if r.predicted_cost == 0.0 {
                    0.0
                } else {
                    ((r.actual_cost / r.predicted_cost) - 1.0).abs()
                }
            })
            .sum();
        total / history.len() as f64
    }
}

impl Default for AdaptiveExecutor {
    fn default() -> Self {
        Self::new()
    }
}
