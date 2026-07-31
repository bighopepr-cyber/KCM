use std::fmt;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(
    feature = "serialization",
    derive(serde::Serialize, serde::Deserialize)
)]
/// Unique sequential row identifier (0-indexed).
pub struct RowID(pub u64);

impl RowID {
    pub fn new(id: u64) -> Self {
        RowID(id)
    }

    pub fn next(self) -> RowID {
        RowID(self.0 + 1)
    }

    pub fn as_usize(self) -> usize {
        self.0 as usize
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(
    feature = "serialization",
    derive(serde::Serialize, serde::Deserialize)
)]
/// Subject entity reference (0-indexed into dictionary).
pub struct SubjectID(pub u32);

impl SubjectID {
    pub fn new(id: u32) -> Self {
        SubjectID(id)
    }

    pub fn as_usize(self) -> usize {
        self.0 as usize
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(
    feature = "serialization",
    derive(serde::Serialize, serde::Deserialize)
)]
/// Predicate/relationship type (0-indexed).
pub struct PredicateID(pub u8);

impl PredicateID {
    pub fn new(id: u8) -> Self {
        PredicateID(id)
    }

    pub fn as_usize(self) -> usize {
        self.0 as usize
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(
    feature = "serialization",
    derive(serde::Serialize, serde::Deserialize)
)]
/// Object entity reference (0-indexed into dictionary).
pub struct ObjectID(pub u32);

impl ObjectID {
    pub fn new(id: u32) -> Self {
        ObjectID(id)
    }

    pub fn as_usize(self) -> usize {
        self.0 as usize
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(
    feature = "serialization",
    derive(serde::Serialize, serde::Deserialize)
)]
/// Context/domain scope for knowledge facts.
pub struct ContextID(pub u8);

impl ContextID {
    pub const NULL: Self = ContextID(0);

    pub fn new(id: u8) -> Self {
        ContextID(id)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(
    feature = "serialization",
    derive(serde::Serialize, serde::Deserialize)
)]
/// Evidence type identifier for provenance tracking.
pub struct EvidenceID(pub u8);

impl EvidenceID {
    pub const UNKNOWN: Self = EvidenceID(0);

    pub fn new(id: u8) -> Self {
        EvidenceID(id)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serialization",
    derive(serde::Serialize, serde::Deserialize)
)]
/// Confidence score (probabilistic, 0.0 to 1.0).
pub struct Confidence(pub f64);

impl Confidence {
    pub fn new(value: f64) -> Result<Self, String> {
        if value.is_nan() || value.is_infinite() {
            return Err("Confidence must be finite".to_string());
        }
        if !(0.0..=1.0).contains(&value) {
            return Err("Confidence must be in [0.0, 1.0]".to_string());
        }
        Ok(Confidence(value))
    }

    pub fn multiply(&self, other: Confidence) -> Confidence {
        let product = (self.0 * other.0).clamp(0.0, 1.0);
        Confidence(product)
    }

    pub fn combine_or(&self, other: Confidence) -> Confidence {
        let combined = (self.0 + other.0 - (self.0 * other.0)).clamp(0.0, 1.0);
        Confidence(combined)
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "serialization",
    derive(serde::Serialize, serde::Deserialize)
)]
/// A knowledge fact (triple with metadata) stored in the columnar schema.
pub struct Fact {
    pub subject: SubjectID,
    pub predicate: PredicateID,
    pub object: ObjectID,
    pub confidence: f64,
    pub evidence: EvidenceID,
    pub timestamp: i64,
    pub context: ContextID,
    pub version: i32,
    pub priority: i8,
    pub owner: u16,
}

impl Fact {
    pub fn new(
        subject: SubjectID,
        predicate: PredicateID,
        object: ObjectID,
        confidence: f64,
    ) -> Result<Self, String> {
        Confidence::new(confidence)?;

        Ok(Fact {
            subject,
            predicate,
            object,
            confidence,
            evidence: EvidenceID::UNKNOWN,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos() as i64,
            context: ContextID::NULL,
            version: 1,
            priority: 0,
            owner: 0,
        })
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
/// Column identifier for query operations.
pub enum ColumnID {
    RowID = 0,
    Subject = 1,
    Predicate = 2,
    Object = 3,
    Confidence = 4,
    Evidence = 5,
    Timestamp = 6,
    Context = 7,
    Version = 8,
    Priority = 9,
    Owner = 10,
}

impl ColumnID {
    pub fn as_usize(self) -> usize {
        self as usize
    }

    pub fn all() -> &'static [ColumnID] {
        &[
            ColumnID::RowID,
            ColumnID::Subject,
            ColumnID::Predicate,
            ColumnID::Object,
            ColumnID::Confidence,
            ColumnID::Evidence,
            ColumnID::Timestamp,
            ColumnID::Context,
            ColumnID::Version,
            ColumnID::Priority,
            ColumnID::Owner,
        ]
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
/// Error types for KCM operations.
pub enum KcmError {
    NotFound(String),
    OutOfMemory,
    InvalidArgument(String),
    Io(String),
    Corrupted(String),
    Conflict(String),
    TransactionAborted,
}

impl fmt::Display for KcmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            KcmError::NotFound(msg) => write!(f, "NotFound: {}", msg),
            KcmError::OutOfMemory => write!(f, "OutOfMemory"),
            KcmError::InvalidArgument(msg) => write!(f, "InvalidArgument: {}", msg),
            KcmError::Io(msg) => write!(f, "Io: {}", msg),
            KcmError::Corrupted(msg) => write!(f, "Corrupted: {}", msg),
            KcmError::Conflict(msg) => write!(f, "Conflict: {}", msg),
            KcmError::TransactionAborted => write!(f, "TransactionAborted"),
        }
    }
}

impl std::error::Error for KcmError {}

impl From<String> for KcmError {
    fn from(s: String) -> Self {
        KcmError::InvalidArgument(s)
    }
}

pub type KcmResult<T> = Result<T, KcmError>;
