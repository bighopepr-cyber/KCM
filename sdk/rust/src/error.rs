use std::fmt;

use kcm_core::types::ErrorCode;

#[derive(Debug, Clone, PartialEq)]
pub enum SdkError {
    NotFound(String),
    OutOfMemory,
    InvalidArgument(String),
    Io(String),
    Corrupted(String),
    Conflict(String),
    TransactionAborted,
}

impl SdkError {
    pub fn error_code(&self) -> ErrorCode {
        match self {
            SdkError::NotFound(_) => ErrorCode::NotFound,
            SdkError::OutOfMemory => ErrorCode::OutOfMemory,
            SdkError::InvalidArgument(_) => ErrorCode::InvalidArgument,
            SdkError::Io(_) => ErrorCode::Io,
            SdkError::Corrupted(_) => ErrorCode::Corrupted,
            SdkError::Conflict(_) => ErrorCode::Conflict,
            SdkError::TransactionAborted => ErrorCode::TransactionAborted,
        }
    }

    pub fn code(&self) -> u16 {
        self.error_code() as u16
    }

    pub fn name(&self) -> &'static str {
        self.error_code().name()
    }

    pub fn description(&self) -> &'static str {
        self.error_code().description()
    }

    pub fn to_json(&self) -> String {
        let code = self.code();
        let name = self.name();
        let msg = match self {
            SdkError::NotFound(m) => m.clone(),
            SdkError::OutOfMemory => self.description().to_string(),
            SdkError::InvalidArgument(m) => m.clone(),
            SdkError::Io(m) => m.clone(),
            SdkError::Corrupted(m) => m.clone(),
            SdkError::Conflict(m) => m.clone(),
            SdkError::TransactionAborted => self.description().to_string(),
        };
        format!(
            r#"{{"code":{},"error":"{}","message":"{}"}}"#,
            code,
            name,
            msg.replace('\\', "\\\\").replace('"', "\\\"")
        )
    }
}

impl fmt::Display for SdkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SdkError::NotFound(msg) => write!(f, "NotFound: {}", msg),
            SdkError::OutOfMemory => write!(f, "OutOfMemory"),
            SdkError::InvalidArgument(msg) => write!(f, "InvalidArgument: {}", msg),
            SdkError::Io(msg) => write!(f, "Io: {}", msg),
            SdkError::Corrupted(msg) => write!(f, "Corrupted: {}", msg),
            SdkError::Conflict(msg) => write!(f, "Conflict: {}", msg),
            SdkError::TransactionAborted => write!(f, "TransactionAborted"),
        }
    }
}

impl std::error::Error for SdkError {}

impl From<kcm_core::types::KcmError> for SdkError {
    fn from(err: kcm_core::types::KcmError) -> Self {
        match err {
            kcm_core::types::KcmError::NotFound(m) => SdkError::NotFound(m),
            kcm_core::types::KcmError::OutOfMemory => SdkError::OutOfMemory,
            kcm_core::types::KcmError::InvalidArgument(m) => SdkError::InvalidArgument(m),
            kcm_core::types::KcmError::Io(m) => SdkError::Io(m),
            kcm_core::types::KcmError::Corrupted(m) => SdkError::Corrupted(m),
            kcm_core::types::KcmError::Conflict(m) => SdkError::Conflict(m),
            kcm_core::types::KcmError::TransactionAborted => SdkError::TransactionAborted,
        }
    }
}
