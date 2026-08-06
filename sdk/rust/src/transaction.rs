use crate::error::SdkError;
use crate::fact::Fact;

pub struct Transaction {
    inner: kcm_runtime::transaction::Transaction,
}

impl Transaction {
    pub(crate) fn new(inner: kcm_runtime::transaction::Transaction) -> Self {
        Transaction { inner }
    }

    pub fn insert(&mut self, fact: &Fact) -> Result<(), SdkError> {
        let core_fact = fact.to_core()?;
        self.inner.insert(core_fact)?;
        Ok(())
    }

    pub fn update(
        &mut self,
        row_idx: usize,
        old_fact: Option<&Fact>,
        new_fact: &Fact,
    ) -> Result<(), SdkError> {
        let old_core = match old_fact {
            Some(f) => Some(f.to_core()?),
            None => None,
        };
        let new_core = new_fact.to_core()?;
        self.inner.update(row_idx, old_core, new_core)?;
        Ok(())
    }

    pub fn delete(&mut self, row_idx: usize, old_fact: &Fact) -> Result<(), SdkError> {
        let core_fact = old_fact.to_core()?;
        self.inner.delete(row_idx, core_fact)?;
        Ok(())
    }

    pub fn commit(mut self) -> Result<(), SdkError> {
        self.inner.commit()?;
        Ok(())
    }

    pub fn rollback(self) -> Result<(), SdkError> {
        self.inner.rollback()?;
        Ok(())
    }

    pub fn is_active(&self) -> bool {
        self.inner.state() == kcm_runtime::transaction::TransactionState::Active
    }

    pub fn change_count(&self) -> usize {
        self.inner.changes().len()
    }
}
