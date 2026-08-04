use kcm_core::types::*;
use kcm_storage::column::Schema;

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
}

impl Transaction {
    pub fn new() -> Self {
        Transaction {
            state: TransactionState::Active,
            changes: Vec::new(),
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
                    let last = schema.len().saturating_sub(1);
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
                    schema.clear_tombstone(*row_idx)?;
                    schema.update_fact(*row_idx, old_fact)?;
                }
            }
        }
        Ok(())
    }

    /// Commit the transaction.
    ///
    /// The caller must call `apply_to_schema()` before commit to persist changes.
    /// Commit marks the transaction as complete and clears the change buffer.
    pub fn commit(mut self) -> Result<(), KcmError> {
        self.state = TransactionState::Committed;
        self.changes.clear();
        Ok(())
    }

    pub fn rollback(mut self) -> Result<(), KcmError> {
        self.state = TransactionState::RolledBack;
        self.changes.clear();
        Ok(())
    }

    pub fn abort(mut self) -> Result<(), KcmError> {
        self.state = TransactionState::Aborted;
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
