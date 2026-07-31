use kcm_core::types::*;
use kcm_storage::column::Schema;
use parking_lot::RwLock;
use std::sync::Arc;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TransactionState {
    Active,
    Committed,
    RolledBack,
    Aborted,
}

pub struct Transaction {
    state: TransactionState,
    changes: Vec<(usize, Fact)>,
    #[allow(dead_code)]
    timestamp: i64,
}

impl Transaction {
    pub fn new() -> Self {
        Transaction {
            state: TransactionState::Active,
            changes: Vec::new(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos() as i64,
        }
    }

    pub fn insert(&mut self, fact: Fact) -> Result<(), KcmError> {
        if self.state != TransactionState::Active {
            return Err(KcmError::TransactionAborted);
        }
        self.changes.push((usize::MAX, fact));
        Ok(())
    }

    pub fn update(&mut self, row_idx: usize, fact: Fact) -> Result<(), KcmError> {
        if self.state != TransactionState::Active {
            return Err(KcmError::TransactionAborted);
        }
        self.changes.push((row_idx, fact));
        Ok(())
    }

    pub fn commit(mut self) -> Result<(), KcmError> {
        self.state = TransactionState::Committed;
        Ok(())
    }

    pub fn rollback(mut self) -> Result<(), KcmError> {
        self.state = TransactionState::RolledBack;
        self.changes.clear();
        Ok(())
    }

    pub fn state(&self) -> TransactionState {
        self.state
    }
}

impl Default for Transaction {
    fn default() -> Self {
        Self::new()
    }
}

pub struct VersionStore {
    versions: Vec<Arc<Schema>>,
    current_version: Arc<RwLock<usize>>,
}

impl VersionStore {
    pub fn new() -> Result<Self, KcmError> {
        let initial_schema = Schema::new(1_000_000)?;
        Ok(VersionStore {
            versions: vec![Arc::new(initial_schema)],
            current_version: Arc::new(RwLock::new(0)),
        })
    }

    pub fn current(&self) -> Arc<Schema> {
        let idx = *self.current_version.read();
        self.versions[idx].clone()
    }

    pub fn create_new_version(&mut self, schema: Schema) -> Result<(), KcmError> {
        self.versions.push(Arc::new(schema));
        *self.current_version.write() = self.versions.len() - 1;
        Ok(())
    }
}

impl Default for VersionStore {
    fn default() -> Self {
        Self::new().unwrap()
    }
}
