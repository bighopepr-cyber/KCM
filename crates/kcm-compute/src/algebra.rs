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
                match self.schema.context_col.get(idx) {
                    Some(ctx) if ctx == ctx_filter => {}
                    _ => continue,
                }
            }
            if let Some(conf_filter) = self.confidence_filter {
                match self.schema.confidence_col.get(idx) {
                    Some(conf) if conf >= conf_filter => {}
                    _ => continue,
                }
            }
            result.push(idx);
        }
        Ok(result)
    }

    fn estimated_rows(&self) -> usize {
        let total = self.schema.len();
        if self.context_filter.is_some() {
            (total as f64 * 0.1).ceil() as usize
        } else if self.confidence_filter.is_some() {
            (total as f64 * 0.3).ceil() as usize
        } else {
            total
        }
    }
}

pub enum FilterPredicate {
    EqualSubject(u32),
    EqualPredicate(u8),
    EqualObject(u32),
    EqualContext(u8),
    InSet(std::collections::HashSet<u32>),
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
                FilterPredicate::InSet(vals) => self
                    .schema
                    .object_col
                    .get(idx)
                    .is_some_and(|v| vals.contains(&v)),
                FilterPredicate::RangeTimestamp(low, high) => self
                    .schema
                    .timestamp_col
                    .get(idx)
                    .is_some_and(|ts| ts >= *low && ts <= *high),
            };
            if matches {
                result.push(idx);
            }
        }
        Ok(result)
    }

    fn estimated_rows(&self) -> usize {
        let input = self.rowids.len();
        let selectivity = match &self.predicate {
            FilterPredicate::EqualSubject(_) => 0.05,
            FilterPredicate::EqualPredicate(_) => 0.15,
            FilterPredicate::EqualObject(_) => 0.05,
            FilterPredicate::EqualContext(_) => 0.2,
            FilterPredicate::InSet(vals) => {
                let set_size = vals.len() as f64;
                (set_size / 255.0).min(0.5).max(0.01)
            }
            FilterPredicate::RangeTimestamp(_, _) => 0.3,
        };
        (input as f64 * selectivity).ceil() as usize
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
                        self.schema.confidence_col.get(idx).unwrap_or(0.0).to_bits()
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
    /// Execute projection: return rowids that pass through.
    /// Actual column extraction is done via `execute_projection()`.
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
        let left = self.left_rowids.len();
        let right = self.right_rowids.len();
        if left == 0 || right == 0 {
            return 0;
        }
        let distinct_right = (right as f64 * 0.3).max(1.0);
        let join_selectivity = 1.0 / distinct_right;
        (left as f64 * right as f64 * join_selectivity).ceil() as usize
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
        let group_col = self
            .group_by
            .ok_or_else(|| KcmError::InvalidArgument("No group_by column".to_string()))?;
        let mut groups: std::collections::HashMap<u32, Vec<f64>> = std::collections::HashMap::new();
        for &idx in &self.rowids {
            let key = match group_col {
                ColumnID::Subject => self.schema.subject_col.get(idx).unwrap_or(0),
                ColumnID::Predicate => self.schema.predicate_col.get(idx).unwrap_or(0) as u32,
                ColumnID::Object => self.schema.object_col.get(idx).unwrap_or(0),
                ColumnID::Context => self.schema.context_col.get(idx).unwrap_or(0) as u32,
                ColumnID::Evidence => self.schema.evidence_col.get(idx).unwrap_or(0) as u32,
                ColumnID::Owner => self.schema.owner_col.get(idx).unwrap_or(0) as u32,
                _ => self.schema.subject_col.get(idx).unwrap_or(0),
            };
            if let Some(conf) = self.schema.confidence_col.get(idx) {
                groups.entry(key).or_default().push(conf);
            }
        }
        let mut result = Vec::new();
        for (key, values) in groups {
            let val = match self.agg_func {
                AggregateFunc::Count => values.len() as f64,
                AggregateFunc::Sum => values.iter().sum(),
                AggregateFunc::Avg => values.iter().sum::<f64>() / values.len() as f64,
                AggregateFunc::Min => values.iter().cloned().fold(f64::INFINITY, f64::min),
                AggregateFunc::Max => values.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
            };
            result.push((key, val));
        }
        Ok(result)
    }
}

impl<'a> Operator for AggregateOp<'a> {
    /// Execute aggregate operator.
    ///
    /// The Operator trait returns row IDs for pipeline compatibility.
    /// For actual aggregate values, call `execute_aggregate()` or `execute_grouped()`.
    fn execute(&self) -> Result<Vec<usize>, KcmError> {
        Ok(self.rowids.clone())
    }

    fn estimated_rows(&self) -> usize {
        if let Some(group_col) = self.group_by {
            let mut groups = std::collections::HashSet::new();
            for &idx in &self.rowids {
                let key = match group_col {
                    ColumnID::Subject => self.schema.subject_col.get(idx).unwrap_or(0) as u64,
                    ColumnID::Predicate => self.schema.predicate_col.get(idx).unwrap_or(0) as u64,
                    ColumnID::Object => self.schema.object_col.get(idx).unwrap_or(0) as u64,
                    ColumnID::Context => self.schema.context_col.get(idx).unwrap_or(0) as u64,
                    ColumnID::Evidence => self.schema.evidence_col.get(idx).unwrap_or(0) as u64,
                    ColumnID::Owner => self.schema.owner_col.get(idx).unwrap_or(0) as u64,
                    _ => self.schema.subject_col.get(idx).unwrap_or(0) as u64,
                };
                groups.insert(key);
            }
            groups.len()
        } else {
            1
        }
    }
}
