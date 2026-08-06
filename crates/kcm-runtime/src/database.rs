use crate::transaction::Transaction;
use kcm_core::dictionary::{DictID, SharedDictionary};
use kcm_core::types::*;
use kcm_optimizer::planner::Planner;
use kcm_optimizer::statistics::Statistics;
use kcm_storage::column::Schema;
use parking_lot::RwLock;
use std::{cmp::Ordering, sync::Arc};

/// Knowledge database with columnar storage.
///
/// Provides Insert, Query, Update, and Delete operations on facts.
/// Thread-safe via internal RwLock on the schema.
pub struct KnowledgeDatabase {
    schema: Arc<RwLock<Schema>>,
    dictionaries: Arc<Dictionaries>,
}

pub struct Dictionaries {
    pub subjects: SharedDictionary,
    pub objects: SharedDictionary,
    pub predicates: SharedDictionary,
    pub evidence: SharedDictionary,
    pub context: SharedDictionary,
    pub owner: SharedDictionary,
}

impl KnowledgeDatabase {
    pub fn new() -> Result<Self, KcmError> {
        const DEFAULT_SCHEMA_CAPACITY: usize = 1_000_000;
        let schema = Arc::new(RwLock::new(Schema::new(DEFAULT_SCHEMA_CAPACITY)?));
        let dictionaries = Arc::new(Dictionaries {
            subjects: SharedDictionary::new(),
            objects: SharedDictionary::new(),
            predicates: SharedDictionary::new(),
            evidence: SharedDictionary::new(),
            context: SharedDictionary::new(),
            owner: SharedDictionary::new(),
        });

        Ok(KnowledgeDatabase {
            schema,
            dictionaries,
        })
    }

    pub fn get_schema(&self) -> parking_lot::RwLockReadGuard<'_, Schema> {
        self.schema.read()
    }

    pub fn get_schema_mut(&self) -> parking_lot::RwLockWriteGuard<'_, Schema> {
        self.schema.write()
    }

    pub fn begin_transaction(&self) -> Transaction {
        Transaction::new()
    }

    pub fn insert(&self, fact: &Fact) -> Result<RowID, KcmError> {
        let mut schema = self.schema.write();
        schema.append_fact(fact)?;
        let row_id = RowID(schema.len() as u64 - 1);
        log::debug!("Inserted fact at row_id={}", row_id.0);
        Ok(row_id)
    }

    pub fn insert_batch(&self, facts: &[Fact]) -> Result<Vec<RowID>, KcmError> {
        let mut schema = self.schema.write();
        let mut row_ids = Vec::new();
        for fact in facts {
            schema.append_fact(fact)?;
            row_ids.push(RowID(schema.len() as u64 - 1));
        }
        log::debug!("Batch inserted {} facts", facts.len());
        Ok(row_ids)
    }

    pub fn update(&self, row_id: RowID, fact: &Fact) -> Result<(), KcmError> {
        let mut schema = self.schema.write();
        schema.update_fact(row_id.as_usize(), fact)
    }

    pub fn delete(&self, row_id: RowID) -> Result<(), KcmError> {
        let mut schema = self.schema.write();
        schema.delete_fact(row_id.as_usize())?;
        log::debug!("Deleted fact at row_id={}", row_id.0);
        Ok(())
    }

    pub fn query(&self) -> QueryBuilder {
        let row_count = self.schema.read().len();
        QueryBuilder::new(self.schema.clone(), row_count)
    }

    pub fn get_fact(&self, row_id: RowID) -> Result<Option<Fact>, KcmError> {
        let schema = self.schema.read();
        Ok(schema.get_fact(row_id.as_usize()))
    }

    pub fn dict_insert_subject(&self, name: &str) -> Result<DictID, KcmError> {
        self.dictionaries.subjects.insert(name)
    }

    pub fn dict_get_subject(&self, id: DictID) -> Option<String> {
        self.dictionaries.subjects.get(id)
    }

    pub fn dict_lookup_subject(&self, name: &str) -> Option<DictID> {
        self.dictionaries.subjects.lookup(name)
    }

    pub fn fact_count(&self) -> usize {
        self.schema.read().len()
    }

    pub fn active_fact_count(&self) -> usize {
        self.schema.read().active_count()
    }

    /// Compact the schema by removing tombstoned rows.
    /// Returns a new KnowledgeDatabase with only active facts.
    pub fn compact(&self) -> Result<Self, KcmError> {
        let (total_before, active_before) = {
            let schema = self.schema.read();
            (schema.len(), schema.active_count())
        };
        log::info!("Compacting database: {} total, {} active", total_before, active_before);
        let compacted = {
            let schema = self.schema.read();
            schema.compact()?
        };
        let new_kb = KnowledgeDatabase {
            schema: Arc::new(RwLock::new(compacted)),
            dictionaries: Arc::clone(&self.dictionaries),
        };
        let total_after = new_kb.fact_count();
        log::info!("Compaction complete: {} -> {} facts", total_before, total_after);
        Ok(new_kb)
    }
}

impl Default for KnowledgeDatabase {
    fn default() -> Self {
        Self::new()
            .expect("KnowledgeDatabase::new uses infallible defaults; this should never fail")
    }
}

/// Ordered filter for cost-based execution.
#[derive(Debug, Clone)]
enum OrderedFilter {
    Subject(u32),
    Predicate(u8),
    Object(u32),
    Confidence(f64),
}

impl OrderedFilter {
    /// Returns estimated selectivity of this filter (lower = more selective = execute first).
    fn selectivity(&self, planner: &Planner) -> f64 {
        match self {
            OrderedFilter::Subject(v) => {
                planner.estimate_selectivity(ColumnID::Subject, *v as i64, *v as i64)
            }
            OrderedFilter::Predicate(v) => {
                planner.estimate_selectivity(ColumnID::Predicate, *v as i64, *v as i64)
            }
            OrderedFilter::Object(v) => {
                planner.estimate_selectivity(ColumnID::Object, *v as i64, *v as i64)
            }
            OrderedFilter::Confidence(v) => {
                planner.estimate_selectivity(ColumnID::Confidence, (*v * 100.0) as i64, 100)
            }
        }
    }

    fn matches(&self, fact: &Fact) -> bool {
        match self {
            OrderedFilter::Subject(v) => fact.subject.0 == *v,
            OrderedFilter::Predicate(v) => fact.predicate.0 == *v,
            OrderedFilter::Object(v) => fact.object.0 == *v,
            OrderedFilter::Confidence(threshold) => fact.confidence >= *threshold,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryOrder {
    ConfidenceAsc,
    ConfidenceDesc,
}

pub struct QueryBuilder {
    schema: Arc<RwLock<Schema>>,
    subject_filter: Option<SubjectID>,
    predicate_filter: Option<PredicateID>,
    object_filter: Option<ObjectID>,
    confidence_filter: Option<f64>,
    row_count: usize,
    order_by: Option<QueryOrder>,
    limit: Option<usize>,
}

impl QueryBuilder {
    pub fn new(schema: Arc<RwLock<Schema>>, row_count: usize) -> Self {
        QueryBuilder {
            schema,
            subject_filter: None,
            predicate_filter: None,
            object_filter: None,
            confidence_filter: None,
            row_count,
            order_by: None,
            limit: None,
        }
    }

    pub fn with_subject(mut self, subject: SubjectID) -> Self {
        self.subject_filter = Some(subject);
        self
    }

    pub fn with_predicate(mut self, predicate: PredicateID) -> Self {
        self.predicate_filter = Some(predicate);
        self
    }

    pub fn with_object(mut self, object: ObjectID) -> Self {
        self.object_filter = Some(object);
        self
    }

    pub fn with_confidence(mut self, threshold: f64) -> Self {
        self.confidence_filter = Some(threshold);
        self
    }

    pub fn with_order_by_confidence_desc(mut self) -> Self {
        self.order_by = Some(QueryOrder::ConfidenceDesc);
        self
    }

    pub fn with_order_by_confidence_asc(mut self) -> Self {
        self.order_by = Some(QueryOrder::ConfidenceAsc);
        self
    }

    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Build ordered filters using the cost-based optimizer.
    ///
    /// The planner estimates selectivity for each filter and orders them
    /// from most selective to least selective. This minimizes the number
    /// of rows that need to be checked by subsequent filters.
    fn ordered_filters(&self) -> Vec<OrderedFilter> {
        let planner = Planner::with_statistics(self.row_count.max(1), Statistics::new());

        let mut filters = Vec::new();
        if let Some(subj) = self.subject_filter {
            filters.push(OrderedFilter::Subject(subj.0));
        }
        if let Some(pred) = self.predicate_filter {
            filters.push(OrderedFilter::Predicate(pred.0));
        }
        if let Some(obj) = self.object_filter {
            filters.push(OrderedFilter::Object(obj.0));
        }
        if let Some(conf) = self.confidence_filter {
            filters.push(OrderedFilter::Confidence(conf));
        }

        // Sort by selectivity: most selective (lowest selectivity) first.
        // This ensures we reject non-matching rows as early as possible.
        filters.sort_by(|a, b| {
            let sa = a.selectivity(&planner);
            let sb = b.selectivity(&planner);
            sa.partial_cmp(&sb).unwrap_or(std::cmp::Ordering::Equal)
        });

        filters
    }

    /// Execute the query with cost-based filter ordering.
    ///
    /// Strategy:
    /// 1. Collect all active filters
    /// 2. Use the optimizer to estimate selectivity for each
    /// 3. Sort filters from most selective to least selective
    /// 4. Scan rows and apply filters in optimized order
    ///    (first non-match short-circuits the remaining filters)
    pub fn execute(self) -> Result<Vec<Fact>, KcmError> {
        let schema = self.schema.read();
        let filters = self.ordered_filters();

        // Fast path: no filters, return all active rows
        if filters.is_empty() {
            let mut result = Vec::new();
            for idx in 0..schema.len() {
                if !schema.is_deleted(idx) {
                    if let Some(fact) = schema.get_fact(idx) {
                        result.push(fact);
                    }
                }
            }
            return Ok(result);
        }

        // Optimized path: apply filters in selectivity order with short-circuit
        let mut result = Vec::new();
        for idx in 0..schema.len() {
            if schema.is_deleted(idx) {
                continue;
            }
            if let Some(fact) = schema.get_fact(idx) {
                let mut matches = true;
                for filter in &filters {
                    if !filter.matches(&fact) {
                        matches = false;
                        break; // Short-circuit: skip remaining filters
                    }
                }
                if matches {
                    result.push(fact);
                }
            }
        }

        if let Some(order_by) = self.order_by {
            result.sort_by(|lhs, rhs| match order_by {
                QueryOrder::ConfidenceAsc => lhs
                    .confidence
                    .partial_cmp(&rhs.confidence)
                    .unwrap_or(Ordering::Equal),
                QueryOrder::ConfidenceDesc => rhs
                    .confidence
                    .partial_cmp(&lhs.confidence)
                    .unwrap_or(Ordering::Equal),
            });
        }

        if let Some(limit) = self.limit {
            result.truncate(limit);
        }

        Ok(result)
    }
}
