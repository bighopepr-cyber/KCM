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

#[derive(Clone, Debug)]
pub enum TransactionChange {
    Insert(Fact),
    Update {
        row_idx: usize,
        old_fact: Option<Fact>,
        new_fact: Fact,
    },
    Delete {
        row_idx: usize,
        old_fact: Fact,
    },
}

pub struct Transaction {
    state: TransactionState,
    changes: Vec<TransactionChange>,
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
        self.changes.push(TransactionChange::Insert(fact));
        Ok(())
    }

    pub fn update(
        &mut self,
        row_idx: usize,
        old_fact: Option<Fact>,
        new_fact: Fact,
    ) -> Result<(), KcmError> {
        if self.state != TransactionState::Active {
            return Err(KcmError::TransactionAborted);
        }
        self.changes.push(TransactionChange::Update {
            row_idx,
            old_fact,
            new_fact,
        });
        Ok(())
    }

    pub fn delete(&mut self, row_idx: usize, old_fact: Fact) -> Result<(), KcmError> {
        if self.state != TransactionState::Active {
            return Err(KcmError::TransactionAborted);
        }
        self.changes
            .push(TransactionChange::Delete { row_idx, old_fact });
        Ok(())
    }

    pub fn apply_to_schema(&self, schema: &mut Schema) -> Result<(), KcmError> {
        for change in &self.changes {
            match change {
                TransactionChange::Insert(fact) => {
                    schema.append_fact(fact)?;
                }
                TransactionChange::Update {
                    row_idx, new_fact, ..
                } => {
                    schema.update_fact(*row_idx, new_fact)?;
                }
                TransactionChange::Delete { row_idx, .. } => {
                    schema.delete_fact(*row_idx)?;
                }
            }
        }
        Ok(())
    }

    pub fn rollback_changes(&self, schema: &mut Schema) -> Result<(), KcmError> {
        for change in self.changes.iter().rev() {
            match change {
                TransactionChange::Insert(_) => {
                    let last = schema.len() - 1;
                    schema.delete_fact(last)?;
                }
                TransactionChange::Update {
                    row_idx, old_fact, ..
                } => {
                    if let Some(old) = old_fact {
                        schema.update_fact(*row_idx, old)?;
                    }
                }
                TransactionChange::Delete { row_idx, old_fact } => {
                    schema.update_fact(*row_idx, old_fact)?;
                    schema.delete_fact(*row_idx).ok();
                }
            }
        }
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

    pub fn changes(&self) -> &[TransactionChange] {
        &self.changes
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

    pub fn create_new_version(&mut self, schema: Schema) -> Result<usize, KcmError> {
        self.versions.push(Arc::new(schema));
        let new_idx = self.versions.len() - 1;
        *self.current_version.write() = new_idx;
        Ok(new_idx)
    }

    pub fn version_count(&self) -> usize {
        self.versions.len()
    }

    pub fn get_version(&self, idx: usize) -> Option<Arc<Schema>> {
        self.versions.get(idx).cloned()
    }
}

impl Default for VersionStore {
    fn default() -> Self {
        Self::new().unwrap()
    }
}
