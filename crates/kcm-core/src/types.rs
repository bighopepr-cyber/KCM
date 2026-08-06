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
    pub fn new(value: f64) -> Result<Self, KcmError> {
        if value.is_nan() || value.is_infinite() {
            return Err(KcmError::InvalidArgument(
                "Confidence must be finite".to_string(),
            ));
        }
        if !(0.0..=1.0).contains(&value) {
            return Err(KcmError::InvalidArgument(
                "Confidence must be in [0.0, 1.0]".to_string(),
            ));
        }
        Ok(Confidence(value))
    }

    pub fn multiply(&self, other: Confidence) -> Confidence {
        let product = (self.0 * other.0).clamp(0.0, 1.0);
        if product.is_finite() {
            Confidence(product)
        } else {
            Confidence(0.0)
        }
    }

    pub fn combine_or(&self, other: Confidence) -> Confidence {
        let combined = (self.0 + other.0 - (self.0 * other.0)).clamp(0.0, 1.0);
        if combined.is_finite() {
            Confidence(combined)
        } else {
            Confidence(0.0)
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "serialization",
    derive(serde::Serialize, serde::Deserialize)
)]
/// A knowledge fact (triple with metadata) stored in the columnar schema.
/// Note: `Eq` is not derived because `confidence: f64` does not implement `Eq`.
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
    ) -> Result<Self, KcmError> {
        Confidence::new(confidence)?;

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
            evidence: EvidenceID::UNKNOWN,
            timestamp,
            context: ContextID::NULL,
            version: 1,
            priority: 0,
            owner: 0,
        })
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(
    feature = "serialization",
    derive(serde::Serialize, serde::Deserialize)
)]
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

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(u16)]
pub enum ErrorCode {
    NotFound = 1001,
    OutOfMemory = 1002,
    InvalidArgument = 1003,
    Io = 1004,
    Corrupted = 1005,
    Conflict = 1006,
    TransactionAborted = 1007,
}

impl ErrorCode {
    pub fn name(&self) -> &'static str {
        match self {
            ErrorCode::NotFound => "NOT_FOUND",
            ErrorCode::OutOfMemory => "OUT_OF_MEMORY",
            ErrorCode::InvalidArgument => "INVALID_ARGUMENT",
            ErrorCode::Io => "IO_ERROR",
            ErrorCode::Corrupted => "CORRUPTED",
            ErrorCode::Conflict => "CONFLICT",
            ErrorCode::TransactionAborted => "TRANSACTION_ABORTED",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            ErrorCode::NotFound => "The requested resource was not found",
            ErrorCode::OutOfMemory => "Insufficient memory to complete the operation",
            ErrorCode::InvalidArgument => "One or more arguments are invalid",
            ErrorCode::Io => "An I/O error occurred",
            ErrorCode::Corrupted => "Data corruption detected",
            ErrorCode::Conflict => "A conflict occurred with concurrent operations",
            ErrorCode::TransactionAborted => "The transaction was aborted",
        }
    }
}

impl KcmError {
    pub fn error_code(&self) -> ErrorCode {
        match self {
            KcmError::NotFound(_) => ErrorCode::NotFound,
            KcmError::OutOfMemory => ErrorCode::OutOfMemory,
            KcmError::InvalidArgument(_) => ErrorCode::InvalidArgument,
            KcmError::Io(_) => ErrorCode::Io,
            KcmError::Corrupted(_) => ErrorCode::Corrupted,
            KcmError::Conflict(_) => ErrorCode::Conflict,
            KcmError::TransactionAborted => ErrorCode::TransactionAborted,
        }
    }

    pub fn to_json(&self) -> String {
        let code = self.error_code();
        let msg = match self {
            KcmError::NotFound(m) => m.clone(),
            KcmError::OutOfMemory => code.description().to_string(),
            KcmError::InvalidArgument(m) => m.clone(),
            KcmError::Io(m) => m.clone(),
            KcmError::Corrupted(m) => m.clone(),
            KcmError::Conflict(m) => m.clone(),
            KcmError::TransactionAborted => code.description().to_string(),
        };
        format!(
            r#"{{"code":{},"error":"{}","message":"{}"}}"#,
            code as u16,
            code.name(),
            msg.replace('\\', "\\\\").replace('"', "\\\"")
        )
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use static_assertions::assert_eq_size;

    #[test]
    fn fact_is_34_bytes() {
        assert_eq_size!(Fact, [u8; 40]);
    }
}
