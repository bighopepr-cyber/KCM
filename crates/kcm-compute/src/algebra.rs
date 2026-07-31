use kcm_core::types::*;
use kcm_storage::column::Schema;

pub trait Operator: Send + Sync {
    fn execute(&self) -> Result<Vec<usize>, KcmError>;
    fn estimated_rows(&self) -> usize;
}

pub struct ScanOp<'a> {
    schema: &'a Schema,
    context_filter: Option<u8>,
    confidence_filter: Option<f64>,
}

impl<'a> ScanOp<'a> {
    pub fn new(schema: &'a Schema) -> Self {
        ScanOp {
            schema,
            context_filter: None,
            confidence_filter: None,
        }
    }

    pub fn with_context(mut self, ctx: u8) -> Self {
        self.context_filter = Some(ctx);
        self
    }

    pub fn with_confidence(mut self, conf: f64) -> Self {
        self.confidence_filter = Some(conf);
        self
    }
}

impl<'a> Operator for ScanOp<'a> {
    fn execute(&self) -> Result<Vec<usize>, KcmError> {
        let mut result = Vec::new();

        for idx in 0..self.schema.len() {
            if self.schema.is_deleted(idx) {
                continue;
            }

            if let Some(ctx_filter) = self.context_filter {
                if let Some(ctx) = self.schema.context_col.get(idx) {
                    if ctx != ctx_filter {
                        continue;
                    }
                } else {
                    continue;
                }
            }

            if let Some(conf_filter) = self.confidence_filter {
                if let Some(conf) = self.schema.confidence_col.get(idx) {
                    if conf < conf_filter {
                        continue;
                    }
                } else {
                    continue;
                }
            }

            result.push(idx);
        }

        Ok(result)
    }

    fn estimated_rows(&self) -> usize {
        self.schema.len()
    }
}

pub enum FilterPredicate {
    EqualSubject(u32),
    EqualPredicate(u8),
    EqualObject(u32),
    EqualContext(u8),
    InSet(Vec<u32>),
    RangeTimestamp(i64, i64),
}

pub struct FilterOp<'a> {
    rowids: Vec<usize>,
    schema: &'a Schema,
    predicate: FilterPredicate,
}

impl<'a> FilterOp<'a> {
    pub fn new(rowids: Vec<usize>, schema: &'a Schema, predicate: FilterPredicate) -> Self {
        FilterOp {
            rowids,
            schema,
            predicate,
        }
    }
}

impl<'a> Operator for FilterOp<'a> {
    fn execute(&self) -> Result<Vec<usize>, KcmError> {
        let mut result = Vec::new();

        for &idx in &self.rowids {
            let matches = match &self.predicate {
                FilterPredicate::EqualSubject(val) => {
                    self.schema.subject_col.get(idx) == Some(*val)
                }
                FilterPredicate::EqualPredicate(val) => {
                    self.schema.predicate_col.get(idx) == Some(*val)
                }
                FilterPredicate::EqualObject(val) => self.schema.object_col.get(idx) == Some(*val),
                FilterPredicate::EqualContext(val) => {
                    self.schema.context_col.get(idx) == Some(*val)
                }
                FilterPredicate::InSet(vals) => {
                    if let Some(obj) = self.schema.object_col.get(idx) {
                        vals.contains(&obj)
                    } else {
                        false
                    }
                }
                FilterPredicate::RangeTimestamp(low, high) => {
                    if let Some(ts) = self.schema.timestamp_col.get(idx) {
                        ts >= *low && ts <= *high
                    } else {
                        false
                    }
                }
            };

            if matches {
                result.push(idx);
            }
        }

        Ok(result)
    }

    fn estimated_rows(&self) -> usize {
        (self.rowids.len() as f64 * 0.1).ceil() as usize
    }
}

pub struct ProjectOp<'a> {
    rowids: Vec<usize>,
    schema: &'a Schema,
    columns: Vec<ColumnID>,
}

impl<'a> ProjectOp<'a> {
    pub fn new(rowids: Vec<usize>, schema: &'a Schema, columns: Vec<ColumnID>) -> Self {
        ProjectOp {
            rowids,
            schema,
            columns,
        }
    }

    pub fn execute_projection(&self) -> Result<Vec<Vec<u64>>, KcmError> {
        let mut result = Vec::new();

        for &idx in &self.rowids {
            let mut row = Vec::new();
            for col in &self.columns {
                let value = match col {
                    ColumnID::RowID => idx as u64,
                    ColumnID::Subject => self.schema.subject_col.get(idx).unwrap_or(0) as u64,
                    ColumnID::Predicate => self.schema.predicate_col.get(idx).unwrap_or(0) as u64,
                    ColumnID::Object => self.schema.object_col.get(idx).unwrap_or(0) as u64,
                    ColumnID::Confidence => {
                        let v = self.schema.confidence_col.get(idx).unwrap_or(0.0);
                        v.to_bits()
                    }
                    ColumnID::Evidence => self.schema.evidence_col.get(idx).unwrap_or(0) as u64,
                    ColumnID::Timestamp => self.schema.timestamp_col.get(idx).unwrap_or(0) as u64,
                    ColumnID::Context => self.schema.context_col.get(idx).unwrap_or(0) as u64,
                    ColumnID::Version => self.schema.version_col.get(idx).unwrap_or(0) as u64,
                    ColumnID::Priority => self.schema.priority_col.get(idx).unwrap_or(0) as u64,
                    ColumnID::Owner => self.schema.owner_col.get(idx).unwrap_or(0) as u64,
                };
                row.push(value);
            }
            result.push(row);
        }

        Ok(result)
    }
}

impl<'a> Operator for ProjectOp<'a> {
    fn execute(&self) -> Result<Vec<usize>, KcmError> {
        Ok(self.rowids.clone())
    }

    fn estimated_rows(&self) -> usize {
        self.rowids.len()
    }
}

pub struct JoinOp<'a> {
    left_rowids: Vec<usize>,
    right_rowids: Vec<usize>,
    schema: &'a Schema,
    join_column: ColumnID,
}

impl<'a> JoinOp<'a> {
    pub fn new(
        left_rowids: Vec<usize>,
        right_rowids: Vec<usize>,
        schema: &'a Schema,
        join_column: ColumnID,
    ) -> Self {
        JoinOp {
            left_rowids,
            right_rowids,
            schema,
            join_column,
        }
    }

    fn get_join_value(&self, idx: usize) -> Option<u32> {
        match self.join_column {
            ColumnID::Subject => self.schema.subject_col.get(idx),
            ColumnID::Object => self.schema.object_col.get(idx),
            ColumnID::Predicate => self.schema.predicate_col.get(idx).map(|v| v as u32),
            ColumnID::Context => self.schema.context_col.get(idx).map(|v| v as u32),
            ColumnID::Evidence => self.schema.evidence_col.get(idx).map(|v| v as u32),
            ColumnID::Owner => self.schema.owner_col.get(idx).map(|v| v as u32),
            _ => self.schema.object_col.get(idx),
        }
    }
}

impl<'a> Operator for JoinOp<'a> {
    fn execute(&self) -> Result<Vec<usize>, KcmError> {
        use std::collections::HashMap;

        let mut hash_table: HashMap<u32, Vec<usize>> = HashMap::new();

        for &idx in &self.right_rowids {
            if let Some(key) = self.get_join_value(idx) {
                hash_table.entry(key).or_default().push(idx);
            }
        }

        let mut result = Vec::new();

        for &idx in &self.left_rowids {
            if let Some(key) = self.get_join_value(idx) {
                if let Some(matches) = hash_table.get(&key) {
                    for &right_idx in matches {
                        result.push(idx);
                        result.push(right_idx);
                    }
                }
            }
        }

        Ok(result)
    }

    fn estimated_rows(&self) -> usize {
        let selectivity = 0.1;
        (self.left_rowids.len() as f64 * self.right_rowids.len() as f64 * selectivity) as usize
    }
}

#[derive(Debug, Clone, Copy)]
pub enum AggregateFunc {
    Count,
    Sum,
    Avg,
    Min,
    Max,
}

pub struct AggregateOp<'a> {
    rowids: Vec<usize>,
    schema: &'a Schema,
    group_by: Option<ColumnID>,
    agg_func: AggregateFunc,
}

impl<'a> AggregateOp<'a> {
    pub fn new(
        rowids: Vec<usize>,
        schema: &'a Schema,
        group_by: Option<ColumnID>,
        agg_func: AggregateFunc,
    ) -> Self {
        AggregateOp {
            rowids,
            schema,
            group_by,
            agg_func,
        }
    }

    pub fn execute_aggregate(&self) -> Result<f64, KcmError> {
        let values: Vec<f64> = self
            .rowids
            .iter()
            .filter_map(|&idx| self.schema.confidence_col.get(idx))
            .collect();

        if values.is_empty() {
            return Ok(0.0);
        }

        match self.agg_func {
            AggregateFunc::Count => Ok(values.len() as f64),
            AggregateFunc::Sum => Ok(values.iter().sum()),
            AggregateFunc::Avg => Ok(values.iter().sum::<f64>() / values.len() as f64),
            AggregateFunc::Min => Ok(values.iter().cloned().fold(f64::INFINITY, f64::min)),
            AggregateFunc::Max => Ok(values.iter().cloned().fold(f64::NEG_INFINITY, f64::max)),
        }
    }

    pub fn execute_grouped(&self) -> Result<Vec<(u32, f64)>, KcmError> {
        let group_col = match self.group_by {
            Some(col) => col,
            None => {
                return Err(KcmError::InvalidArgument(
                    "No group_by column specified".to_string(),
                ))
            }
        };

        let mut groups: std::collections::HashMap<u32, Vec<f64>> = std::collections::HashMap::new();

        for &idx in &self.rowids {
            let group_key = match group_col {
                ColumnID::Subject => self.schema.subject_col.get(idx).unwrap_or(0),
                ColumnID::Predicate => self.schema.predicate_col.get(idx).unwrap_or(0) as u32,
                ColumnID::Object => self.schema.object_col.get(idx).unwrap_or(0),
                ColumnID::Context => self.schema.context_col.get(idx).unwrap_or(0) as u32,
                ColumnID::Evidence => self.schema.evidence_col.get(idx).unwrap_or(0) as u32,
                ColumnID::Owner => self.schema.owner_col.get(idx).unwrap_or(0) as u32,
                _ => self.schema.subject_col.get(idx).unwrap_or(0),
            };

            if let Some(conf) = self.schema.confidence_col.get(idx) {
                groups.entry(group_key).or_default().push(conf);
            }
        }

        let mut result = Vec::new();
        for (key, values) in groups {
            let agg_value = match self.agg_func {
                AggregateFunc::Count => values.len() as f64,
                AggregateFunc::Sum => values.iter().sum(),
                AggregateFunc::Avg => values.iter().sum::<f64>() / values.len() as f64,
                AggregateFunc::Min => values.iter().cloned().fold(f64::INFINITY, f64::min),
                AggregateFunc::Max => values.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
            };
            result.push((key, agg_value));
        }

        Ok(result)
    }
}

impl<'a> Operator for AggregateOp<'a> {
    fn execute(&self) -> Result<Vec<usize>, KcmError> {
        Ok(self.rowids.clone())
    }

    fn estimated_rows(&self) -> usize {
        if let Some(group_col) = self.group_by {
            let mut groups = std::collections::HashSet::new();
            for &idx in &self.rowids {
                let key = match group_col {
                    ColumnID::Subject => self.schema.subject_col.get(idx).unwrap_or(0) as u64,
                    ColumnID::Object => self.schema.object_col.get(idx).unwrap_or(0) as u64,
                    _ => 0,
                };
                groups.insert(key);
            }
            groups.len()
        } else {
            1
        }
    }
}
