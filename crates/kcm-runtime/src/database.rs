use crate::transaction::{Transaction, VersionStore};
use kcm_core::dictionary::{DictID, SharedDictionary};
use kcm_core::types::*;
use kcm_storage::column::Schema;
use parking_lot::RwLock;
use std::sync::Arc;

/// Knowledge database with columnar storage.
///
/// Provides Insert, Query, Update, and Delete operations on facts.
/// Thread-safe via internal RwLock on the schema.
pub struct KnowledgeDatabase {
    schema: Arc<RwLock<Schema>>,
    dictionaries: Arc<Dictionaries>,
    #[allow(dead_code)]
    version_store: Arc<RwLock<VersionStore>>,
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
        let schema = Arc::new(RwLock::new(Schema::new(1_000_000)?));
        let dictionaries = Arc::new(Dictionaries {
            subjects: SharedDictionary::new(),
            objects: SharedDictionary::new(),
            predicates: SharedDictionary::new(),
            evidence: SharedDictionary::new(),
            context: SharedDictionary::new(),
            owner: SharedDictionary::new(),
        });
        let version_store = Arc::new(RwLock::new(VersionStore::new()?));

        Ok(KnowledgeDatabase {
            schema,
            dictionaries,
            version_store,
        })
    }

    pub fn begin_transaction(&self) -> Transaction {
        Transaction::new()
    }

    pub fn insert(&self, fact: &Fact) -> Result<RowID, KcmError> {
        let mut schema = self.schema.write();
        schema.append_fact(fact)?;
        let row_id = RowID(schema.len() as u64 - 1);
        Ok(row_id)
    }

    pub fn insert_batch(&self, facts: &[Fact]) -> Result<Vec<RowID>, KcmError> {
        let mut schema = self.schema.write();
        let mut row_ids = Vec::new();
        for fact in facts {
            schema.append_fact(fact)?;
            row_ids.push(RowID(schema.len() as u64 - 1));
        }
        Ok(row_ids)
    }

    pub fn update(&self, row_id: RowID, fact: &Fact) -> Result<(), KcmError> {
        let mut schema = self.schema.write();
        schema.update_fact(row_id.as_usize(), fact)
    }

    pub fn delete(&self, row_id: RowID) -> Result<(), KcmError> {
        let mut schema = self.schema.write();
        schema.delete_fact(row_id.as_usize())
    }

    pub fn query(&self) -> QueryBuilder {
        QueryBuilder::new((*self.schema.read()).clone())
    }

    pub fn get_fact(&self, row_id: RowID) -> Result<Option<Fact>, KcmError> {
        let schema = self.schema.read();
        Ok(schema.get_fact(row_id.as_usize()))
    }

    pub fn dict_insert_subject(&self, name: &str) -> DictID {
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
}

impl Default for KnowledgeDatabase {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

pub struct QueryBuilder {
    schema: Schema,
    subject_filter: Option<SubjectID>,
    predicate_filter: Option<PredicateID>,
    object_filter: Option<ObjectID>,
    confidence_filter: Option<f64>,
}

impl QueryBuilder {
    pub fn new(schema: Schema) -> Self {
        QueryBuilder {
            schema,
            subject_filter: None,
            predicate_filter: None,
            object_filter: None,
            confidence_filter: None,
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

    pub fn execute(self) -> Result<Vec<Fact>, KcmError> {
        let mut result = Vec::new();

        for idx in 0..self.schema.len() {
            if let Some(fact) = self.schema.get_fact(idx) {
                let mut matches = true;

                if let Some(subj) = self.subject_filter {
                    if fact.subject != subj {
                        matches = false;
                    }
                }

                if let Some(pred) = self.predicate_filter {
                    if fact.predicate != pred {
                        matches = false;
                    }
                }

                if let Some(obj) = self.object_filter {
                    if fact.object != obj {
                        matches = false;
                    }
                }

                if let Some(conf_threshold) = self.confidence_filter {
                    if fact.confidence < conf_threshold {
                        matches = false;
                    }
                }

                if matches {
                    result.push(fact);
                }
            }
        }

        Ok(result)
    }
}
