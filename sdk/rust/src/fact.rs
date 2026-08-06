use crate::error::SdkError;

#[derive(Debug, Clone)]
pub struct Fact {
    pub subject: u32,
    pub predicate: u8,
    pub object: u32,
    pub confidence: f64,
    pub evidence: u8,
    pub timestamp: i64,
    pub context: u8,
    pub version: i32,
    pub priority: i8,
    pub owner: u16,
}

impl Fact {
    pub fn new(
        subject: u32,
        predicate: u8,
        object: u32,
        confidence: f64,
    ) -> Result<Self, SdkError> {
        if confidence < 0.0 || confidence > 1.0 {
            return Err(SdkError::InvalidArgument(
                "Confidence must be in [0.0, 1.0]".to_string(),
            ));
        }
        if confidence.is_nan() || confidence.is_infinite() {
            return Err(SdkError::InvalidArgument(
                "Confidence must be finite".to_string(),
            ));
        }
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
            .min(i64::MAX as u128) as i64;

        Ok(Fact {
            subject,
            predicate,
            object,
            confidence,
            evidence: 0,
            timestamp,
            context: 0,
            version: 1,
            priority: 0,
            owner: 0,
        })
    }

    pub fn with_evidence(mut self, evidence: u8) -> Self {
        self.evidence = evidence;
        self
    }

    pub fn with_context(mut self, context: u8) -> Self {
        self.context = context;
        self
    }

    pub fn with_version(mut self, version: i32) -> Self {
        self.version = version;
        self
    }

    pub fn with_priority(mut self, priority: i8) -> Self {
        self.priority = priority;
        self
    }

    pub fn with_owner(mut self, owner: u16) -> Self {
        self.owner = owner;
        self
    }

    pub fn with_timestamp(mut self, timestamp: i64) -> Self {
        self.timestamp = timestamp;
        self
    }

    pub(crate) fn to_core(&self) -> Result<kcm_core::types::Fact, SdkError> {
        Ok(kcm_core::types::Fact {
            subject: kcm_core::types::SubjectID::new(self.subject),
            predicate: kcm_core::types::PredicateID::new(self.predicate),
            object: kcm_core::types::ObjectID::new(self.object),
            confidence: self.confidence,
            evidence: kcm_core::types::EvidenceID::new(self.evidence),
            timestamp: self.timestamp,
            context: kcm_core::types::ContextID::new(self.context),
            version: self.version,
            priority: self.priority,
            owner: self.owner,
        })
    }

    pub(crate) fn from_core(fact: kcm_core::types::Fact) -> Self {
        Fact {
            subject: fact.subject.0,
            predicate: fact.predicate.0,
            object: fact.object.0,
            confidence: fact.confidence,
            evidence: fact.evidence.0,
            timestamp: fact.timestamp,
            context: fact.context.0,
            version: fact.version,
            priority: fact.priority,
            owner: fact.owner,
        }
    }
}
