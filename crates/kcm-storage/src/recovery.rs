use crate::column::Schema;
use crate::file_format::DatabaseFile;
use crate::wal::WriteAheadLog;
use kcm_core::types::*;
use std::path::Path;

pub struct RecoveryManager;

impl RecoveryManager {
    pub fn recover<P: AsRef<Path>>(db_path: P, wal_path: P) -> Result<Schema, KcmError> {
        let db_path = db_path.as_ref();
        let wal_path = wal_path.as_ref();

        if db_path.exists()
            && std::fs::metadata(db_path)
                .map_err(|e| KcmError::Io(e.to_string()))?
                .len()
                > 32
        {
            match DatabaseFile::load(db_path) {
                Ok(mut schema) => {
                    if wal_path.exists() {
                        Self::replay_wal(&mut schema, wal_path)?;
                    }
                    Ok(schema)
                }
                Err(_) => Self::recover_from_backup(db_path, wal_path),
            }
        } else if wal_path.exists() {
            let mut schema = Schema::new(1_000_000)?;
            Self::replay_wal(&mut schema, wal_path)?;
            Ok(schema)
        } else {
            Schema::new(1_000_000)
        }
    }

    fn recover_from_backup<P: AsRef<Path>>(db_path: P, wal_path: P) -> Result<Schema, KcmError> {
        let db_path = db_path.as_ref();
        let wal_path = wal_path.as_ref();
        let backup_path = format!("{}.backup", db_path.display());

        if std::path::Path::new(&backup_path).exists() {
            if let Ok(mut schema) = DatabaseFile::load(&backup_path) {
                if wal_path.exists() {
                    Self::replay_wal(&mut schema, wal_path)?;
                }
                std::fs::copy(&backup_path, db_path).map_err(|e| KcmError::Io(e.to_string()))?;
                return Ok(schema);
            }
        }

        Err(KcmError::Corrupted(
            "Database and backup both corrupted".to_string(),
        ))
    }

    fn replay_wal(schema: &mut Schema, wal_path: impl AsRef<Path>) -> Result<(), KcmError> {
        let wal = WriteAheadLog::new(wal_path)?;

        wal.replay(|entry| match entry {
            crate::wal::WALEntry::Insert {
                subject,
                predicate,
                object,
                confidence,
                timestamp,
                context,
            } => {
                let fact = Fact {
                    subject,
                    predicate,
                    object,
                    confidence,
                    evidence: EvidenceID::UNKNOWN,
                    timestamp,
                    context,
                    version: 1,
                    priority: 0,
                    owner: 0,
                };
                schema.append_fact(&fact)
            }
            crate::wal::WALEntry::Delete { row_id } => schema.delete_fact(row_id as usize),
        })?;

        Ok(())
    }

    pub fn backup<P: AsRef<Path>>(db_path: P) -> Result<(), KcmError> {
        let db_path = db_path.as_ref();
        let backup_path = format!("{}.backup", db_path.display());
        std::fs::copy(db_path, &backup_path)
            .map_err(|e| KcmError::Io(format!("Backup failed: {}", e)))?;
        Ok(())
    }
}
