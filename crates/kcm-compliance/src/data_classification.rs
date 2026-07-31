use kcm_core::types::Fact;

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
        matches!(
            self,
            DataClassification::Internal
                | DataClassification::Confidential
                | DataClassification::Restricted
        )
    }
    pub fn max_retention_days(&self) -> Option<i32> {
        match self {
            DataClassification::Public => Some(365),
            DataClassification::Internal => Some(730),
            DataClassification::Confidential => Some(1825),
            DataClassification::Restricted => Some(2555),
        }
    }
}

pub struct ClassifiedFact {
    pub fact: Fact,
    pub classification: DataClassification,
}

impl ClassifiedFact {
    pub fn should_retain(&self, now: i64) -> bool {
        match self.classification.max_retention_days() {
            Some(days) => (now - self.fact.timestamp) <= (days as i64) * 86400,
            None => true,
        }
    }

    pub fn is_expired(&self, now: i64) -> bool {
        !self.should_retain(now)
    }
}
