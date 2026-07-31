use kcm_core::types::ColumnID;
use kcm_storage::column::Schema;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Histogram {
    Uniform { buckets: Vec<u64> },
    FrequencyBased { values: Vec<u64> },
}

impl Histogram {
    pub fn uniform_from_range(min: i64, max: i64, num_buckets: usize) -> Self {
        let range = if max > min { max - min } else { 1 };
        let _bucket_size = (range / num_buckets as i64).max(1);
        Histogram::Uniform {
            buckets: vec![0; num_buckets],
        }
    }

    pub fn bucket_boundaries(&self, min: i64, max: i64) -> Vec<(i64, i64)> {
        match self {
            Histogram::Uniform { buckets } => {
                let range = if max > min { max - min } else { 1 };
                let bucket_size = (range / buckets.len() as i64).max(1);
                buckets
                    .iter()
                    .enumerate()
                    .map(|(i, _)| {
                        let lo = min + i as i64 * bucket_size;
                        let hi = (lo + bucket_size - 1).min(max);
                        (lo, hi)
                    })
                    .collect()
            }
            Histogram::FrequencyBased { values } => values
                .windows(2)
                .map(|w| (w[0] as i64, w[1] as i64))
                .collect(),
        }
    }
}

pub struct ColumnStatistics {
    pub row_count: u64,
    pub null_count: u64,
    pub cardinality: u64,
    pub min_value: Option<i64>,
    pub max_value: Option<i64>,
    pub histogram: Option<Histogram>,
}

pub struct Statistics {
    pub column_stats: HashMap<ColumnID, ColumnStatistics>,
}

impl Statistics {
    pub fn new() -> Self {
        Statistics {
            column_stats: HashMap::new(),
        }
    }

    pub fn update_from_schema(&mut self, schema: &Schema) {
        let row_count = schema.len() as u64;

        let mut subject_cardinality = std::collections::HashSet::new();
        let mut predicate_cardinality = std::collections::HashSet::new();
        let mut object_cardinality = std::collections::HashSet::new();
        let mut conf_min = f64::INFINITY;
        let mut conf_max = f64::NEG_INFINITY;
        let mut ts_min = i64::MAX;
        let mut ts_max = i64::MIN;
        let mut null_count = 0u64;

        for idx in 0..schema.len() {
            if schema.is_deleted(idx) {
                null_count += 1;
                continue;
            }
            if let Some(s) = schema.subject_col.get(idx) {
                subject_cardinality.insert(s);
            }
            if let Some(p) = schema.predicate_col.get(idx) {
                predicate_cardinality.insert(p);
            }
            if let Some(o) = schema.object_col.get(idx) {
                object_cardinality.insert(o);
            }
            if let Some(c) = schema.confidence_col.get(idx) {
                if c < conf_min {
                    conf_min = c;
                }
                if c > conf_max {
                    conf_max = c;
                }
            }
            if let Some(ts) = schema.timestamp_col.get(idx) {
                if ts < ts_min {
                    ts_min = ts;
                }
                if ts > ts_max {
                    ts_max = ts;
                }
            }
        }

        self.column_stats.insert(
            ColumnID::Subject,
            ColumnStatistics {
                row_count,
                null_count,
                cardinality: subject_cardinality.len() as u64,
                min_value: None,
                max_value: None,
                histogram: None,
            },
        );
        self.column_stats.insert(
            ColumnID::Predicate,
            ColumnStatistics {
                row_count,
                null_count,
                cardinality: predicate_cardinality.len() as u64,
                min_value: None,
                max_value: None,
                histogram: None,
            },
        );
        self.column_stats.insert(
            ColumnID::Object,
            ColumnStatistics {
                row_count,
                null_count,
                cardinality: object_cardinality.len() as u64,
                min_value: None,
                max_value: None,
                histogram: None,
            },
        );
        self.column_stats.insert(
            ColumnID::Confidence,
            ColumnStatistics {
                row_count,
                null_count,
                cardinality: 0,
                min_value: if conf_min.is_finite() {
                    Some((conf_min * 1000.0) as i64)
                } else {
                    None
                },
                max_value: if conf_max.is_finite() {
                    Some((conf_max * 1000.0) as i64)
                } else {
                    None
                },
                histogram: None,
            },
        );
        self.column_stats.insert(
            ColumnID::Timestamp,
            ColumnStatistics {
                row_count,
                null_count,
                cardinality: 0,
                min_value: if ts_min != i64::MAX {
                    Some(ts_min)
                } else {
                    None
                },
                max_value: if ts_max != i64::MIN {
                    Some(ts_max)
                } else {
                    None
                },
                histogram: None,
            },
        );
    }

    pub fn estimate_selectivity(&self, column: ColumnID, low: i64, high: i64) -> f64 {
        if let Some(stats) = self.column_stats.get(&column) {
            if let (Some(min), Some(max)) = (stats.min_value, stats.max_value) {
                let range = (max - min) as f64;
                if range <= 0.0 {
                    return 0.5;
                }
                let filter_range = (high - low) as f64;
                (filter_range / range).clamp(0.0, 1.0)
            } else {
                0.5
            }
        } else {
            0.5
        }
    }

    pub fn estimate_join_selectivity(&self, cardinality_left: u64, cardinality_right: u64) -> f64 {
        let max_cardinality = cardinality_left.max(cardinality_right);
        if max_cardinality == 0 {
            0.0
        } else {
            1.0 / max_cardinality as f64
        }
    }
}

impl Default for Statistics {
    fn default() -> Self {
        Self::new()
    }
}
