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
                Err(e) => {
                    eprintln!("DB load failed: {}, attempting backup recovery", e);
                    Self::recover_from_backup(db_path, wal_path)
                }
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
        let wal_path_buf = wal_path.as_ref().to_path_buf();
        let wal = WriteAheadLog::new(&wal_path_buf)?;
        wal.verify_integrity()?;
        let count = wal.replay(|entry| match entry {
            crate::wal::WALEntry::Insert { .. } => {
                if let Some(fact) = entry.to_fact() {
                    schema.append_fact(&fact)?;
                }
                Ok(())
            }
            crate::wal::WALEntry::Delete { row_id } => schema.delete_fact(row_id as usize),
        })?;
        if count > 0 {
            use std::io::Write;
            let mut wal_file = std::fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(&wal_path_buf)
                .map_err(|e| {
                    KcmError::Io(format!("Failed to truncate WAL after recovery: {}", e))
                })?;
            wal_file.flush().map_err(|e| KcmError::Io(e.to_string()))?;
            wal_file
                .sync_all()
                .map_err(|e| KcmError::Io(e.to_string()))?;
        }
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
