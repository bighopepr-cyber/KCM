#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum DataClassification {
    Public,
    Internal,
    Confidential,
    Restricted,
}

impl DataClassification {
    pub fn requires_encryption(&self) -> bool {
        matches!(
            self,
            DataClassification::Confidential | DataClassification::Restricted
        )
    }
    pub fn requires_audit_log(&self) -> bool {
        matches!(self, DataClassification::Restricted)
    }
    pub fn max_retention_days(&self) -> Option<i32> {
        match self {
            DataClassification::Public => Some(2555),
            DataClassification::Internal => Some(1095),
            DataClassification::Confidential => Some(365),
            DataClassification::Restricted => Some(180),
        }
    }
}

pub struct ClassifiedFact {
    pub fact_id: u64,
    pub classification: DataClassification,
    pub owner: String,
    pub created_at: i64,
}

impl ClassifiedFact {
    pub fn should_retain(&self, now: i64) -> bool {
        match self.classification.max_retention_days() {
            Some(days) => (now - self.created_at) <= (days as i64) * 86400,
            None => true,
        }
    }
}
