use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Compression error: {0}")]
    Compression(String),

    #[error("Corruption detected: {0}")]
    Corrupted(String),

    #[error("Column full: capacity {capacity}, current {current}")]
    ColumnFull { capacity: usize, current: usize },

    #[error("Index out of bounds: {index} >= {len}")]
    IndexOutOfBounds { index: usize, len: usize },

    #[error("Invalid encoding: {0}")]
    InvalidEncoding(String),

    #[error("Hash mismatch: expected {expected}, got {actual}")]
    HashMismatch { expected: String, actual: String },
}

impl From<StorageError> for kcm_core::types::KcmError {
    fn from(err: StorageError) -> Self {
        match err {
            StorageError::Io(e) => kcm_core::types::KcmError::Io(e.to_string()),
            StorageError::Compression(s) => kcm_core::types::KcmError::Io(s),
            StorageError::Corrupted(s) => kcm_core::types::KcmError::Corrupted(s),
            StorageError::ColumnFull { .. } => kcm_core::types::KcmError::OutOfMemory,
            StorageError::IndexOutOfBounds { index, len } => {
                kcm_core::types::KcmError::InvalidArgument(format!(
                    "Index {} out of bounds (len {})",
                    index, len
                ))
            }
            StorageError::InvalidEncoding(s) => kcm_core::types::KcmError::InvalidArgument(s),
            StorageError::HashMismatch { expected, actual } => {
                kcm_core::types::KcmError::Corrupted(format!(
                    "Hash mismatch: expected {}, got {}",
                    expected, actual
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_error_display() {
        let err = StorageError::Corrupted("test corruption".to_string());
        assert!(err.to_string().contains("test corruption"));
    }

    #[test]
    fn test_storage_error_conversion() {
        let err = StorageError::ColumnFull {
            capacity: 100,
            current: 100,
        };
        let kcm_err: kcm_core::types::KcmError = err.into();
        assert!(matches!(kcm_err, kcm_core::types::KcmError::OutOfMemory));
    }

    #[test]
    fn test_storage_error_index_out_of_bounds() {
        let err = StorageError::IndexOutOfBounds { index: 50, len: 10 };
        let kcm_err: kcm_core::types::KcmError = err.into();
        match kcm_err {
            kcm_core::types::KcmError::InvalidArgument(msg) => {
                assert!(msg.contains("50"));
                assert!(msg.contains("10"));
            }
            _ => panic!("Expected InvalidArgument"),
        }
    }
}
