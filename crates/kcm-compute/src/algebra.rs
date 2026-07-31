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

        for (idx, confidence) in self.schema.confidence_col.iter().enumerate() {
            if let Some(conf_filter) = self.confidence_filter {
                if *confidence < conf_filter {
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
    #[allow(dead_code)]
    schema: &'a Schema,
    #[allow(dead_code)]
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
    #[allow(dead_code)]
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
}

impl<'a> Operator for JoinOp<'a> {
    fn execute(&self) -> Result<Vec<usize>, KcmError> {
        use std::collections::HashMap;

        let mut hash_table: HashMap<u32, Vec<usize>> = HashMap::new();

        for &idx in &self.right_rowids {
            if let Some(key) = self.schema.object_col.get(idx) {
                hash_table.entry(key).or_default().push(idx);
            }
        }

        let mut result = Vec::new();

        for &idx in &self.left_rowids {
            if let Some(key) = self.schema.object_col.get(idx) {
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
}

impl<'a> Operator for AggregateOp<'a> {
    fn execute(&self) -> Result<Vec<usize>, KcmError> {
        match self.agg_func {
            AggregateFunc::Count => {
                println!("Count: {}", self.rowids.len());
            }
            AggregateFunc::Sum => {
                let sum: f64 = self
                    .rowids
                    .iter()
                    .filter_map(|&idx| self.schema.confidence_col.get(idx))
                    .sum();
                println!("Sum: {}", sum);
            }
            _ => {}
        }

        Ok(self.rowids.clone())
    }

    fn estimated_rows(&self) -> usize {
        if self.group_by.is_some() {
            256
        } else {
            1
        }
    }
}
