use kcm_core::types::{Fact, KcmError};

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
    pub fn max_retention_days(&self) -> u32 {
        match self {
            DataClassification::Public => 365,
            DataClassification::Internal => 730,
            DataClassification::Confidential => 1825,
            DataClassification::Restricted => 2555,
        }
    }

    pub fn validate_encryption(&self, is_encrypted: bool) -> Result<(), KcmError> {
        if self.requires_encryption() && !is_encrypted {
            return Err(KcmError::InvalidArgument(format!(
                "Classification {:?} requires encryption but fact is not encrypted",
                self
            )));
        }
        Ok(())
    }
}

pub struct ClassifiedFact {
    pub fact: Fact,
    pub classification: DataClassification,
}

impl ClassifiedFact {
    pub fn should_retain(&self, now: i64) -> bool {
        let days = self.classification.max_retention_days();
        (now - self.fact.timestamp) <= (days as i64) * 86400
    }

    pub fn is_expired(&self, now: i64) -> bool {
        !self.should_retain(now)
    }
}
