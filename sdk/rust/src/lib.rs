pub mod error;
pub mod fact;
pub mod query;
pub mod transaction;

pub use error::SdkError;
pub use fact::Fact;
pub use query::QueryResult;
pub use transaction::Transaction;

pub use kcm_core::types::ErrorCode;
pub use kcm_core::types::KcmError;

use kcm_runtime::database::KnowledgeDatabase;

pub struct Database {
    inner: KnowledgeDatabase,
}

impl Database {
    pub fn new() -> Result<Self, SdkError> {
        let inner = KnowledgeDatabase::new()?;
        Ok(Database { inner })
    }

    pub fn insert(&self, fact: &Fact) -> Result<u64, SdkError> {
        let core_fact = fact.to_core()?;
        let row_id = self.inner.insert(&core_fact)?;
        Ok(row_id.0)
    }

    pub fn update(&self, row_id: u64, fact: &Fact) -> Result<(), SdkError> {
        let core_fact = fact.to_core()?;
        let id = kcm_core::types::RowID::new(row_id);
        self.inner.update(id, &core_fact)?;
        Ok(())
    }

    pub fn delete(&self, row_id: u64) -> Result<(), SdkError> {
        let id = kcm_core::types::RowID::new(row_id);
        self.inner.delete(id)?;
        Ok(())
    }

    pub fn query(&self, kql: &str) -> Result<QueryResult, SdkError> {
        let _ = kql;
        let builder = self.inner.query();
        let facts = builder.execute()?;
        Ok(QueryResult::new(facts))
    }

    pub fn query_all(&self) -> Result<Vec<Fact>, SdkError> {
        let builder = self.inner.query();
        let core_facts = builder.execute()?;
        Ok(core_facts.into_iter().map(Fact::from_core).collect())
    }

    pub fn fact_count(&self) -> u64 {
        self.inner.fact_count() as u64
    }

    pub fn active_fact_count(&self) -> u64 {
        self.inner.active_fact_count() as u64
    }

    pub fn begin_transaction(&self) -> Result<Transaction, SdkError> {
        let txn = self.inner.begin_transaction();
        Ok(Transaction::new(txn))
    }

    pub fn save(&self, path: &str) -> Result<(), SdkError> {
        let schema = self.inner.get_schema();
        kcm_storage::file_format::DatabaseFile::save(&schema, path)?;
        Ok(())
    }

    pub fn load(path: &str) -> Result<Self, SdkError> {
        let loaded_schema = kcm_storage::file_format::DatabaseFile::load(path)?;
        let db = KnowledgeDatabase::new()?;
        let len = loaded_schema.len();
        for idx in 0..len {
            if let Some(fact) = loaded_schema.get_fact(idx) {
                db.insert(&fact)?;
            }
        }
        Ok(Database { inner: db })
    }

    pub fn verify(path: &str) -> Result<(), SdkError> {
        let valid = kcm_storage::file_format::DatabaseFile::verify(path)?;
        if valid {
            Ok(())
        } else {
            Err(SdkError::Corrupted(
                "Database file integrity check failed".to_string(),
            ))
        }
    }

    pub fn close(&self) {
        let _ = &self.inner;
    }

    pub fn get_fact(&self, row_id: u64) -> Result<Option<Fact>, SdkError> {
        let id = kcm_core::types::RowID::new(row_id);
        match self.inner.get_fact(id)? {
            Some(core_fact) => Ok(Some(Fact::from_core(core_fact))),
            None => Ok(None),
        }
    }
}

impl Default for Database {
    fn default() -> Self {
        Self::new()
            .expect("KnowledgeDatabase::new uses infallible defaults; this should never fail")
    }
}
