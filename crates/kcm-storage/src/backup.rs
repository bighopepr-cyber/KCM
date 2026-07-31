use crate::column::Schema;
use crate::file_format::DatabaseFile;
use kcm_core::types::*;
use std::fs;
use std::path::{Path, PathBuf};

pub struct BackupManager {
    backup_dir: PathBuf,
}

impl BackupManager {
    pub fn new<P: AsRef<Path>>(backup_dir: P) -> Result<Self, KcmError> {
        fs::create_dir_all(backup_dir.as_ref()).map_err(|e| KcmError::Io(e.to_string()))?;
        Ok(BackupManager {
            backup_dir: backup_dir.as_ref().to_path_buf(),
        })
    }

    pub fn create_full_backup(&self, schema: &Schema) -> Result<PathBuf, KcmError> {
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        let name = format!("backup_full_{}.kcm", ts);
        let path = self.backup_dir.join(&name);
        DatabaseFile::save(schema, &path)?;
        if !DatabaseFile::verify(&path).map_err(|e| KcmError::Io(e.to_string()))? {
            std::fs::remove_file(&path).ok();
            return Err(KcmError::Corrupted(
                "Backup verification failed after save".to_string(),
            ));
        }
        self.write_manifest(&path, "full")?;
        Ok(path)
    }

    pub fn create_incremental_backup(
        &self,
        schema: &Schema,
        last_backup: &Path,
    ) -> Result<PathBuf, KcmError> {
        if !last_backup.exists() {
            return self.create_full_backup(schema);
        }
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let name = format!("backup_incr_{}.kcm", ts);
        let path = self.backup_dir.join(&name);
        DatabaseFile::save(schema, &path)?;
        let manifest_path = path.with_extension("manifest");
        let content = format!(
            "backup_type: incremental\ncreated: {}\nbase: {}\nactive_rows: {}\n",
            ts,
            last_backup.display(),
            schema.active_count()
        );
        std::fs::write(manifest_path, content)
            .map_err(|e| KcmError::Io(format!("Failed to write manifest: {}", e)))?;
        Ok(path)
    }

    fn write_manifest(&self, backup_path: &Path, backup_type: &str) -> Result<(), KcmError> {
        let manifest = backup_path.with_extension("manifest");
        let content = format!(
            "backup_type: {}\ncreated: {}\n",
            backup_type,
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
        );
        fs::write(manifest, content).map_err(|e| KcmError::Io(e.to_string()))?;
        Ok(())
    }

    pub fn list_backups(&self) -> Result<Vec<PathBuf>, KcmError> {
        let mut backups = Vec::new();
        let entries = fs::read_dir(&self.backup_dir).map_err(|e| KcmError::Io(e.to_string()))?;
        for entry in entries {
            let entry = entry.map_err(|e| KcmError::Io(e.to_string()))?;
            let path = entry.path();
            if path.extension().map(|e| e == "kcm").unwrap_or(false) {
                backups.push(path);
            }
        }
        backups.sort();
        Ok(backups)
    }
}

pub struct RestoreManager;

impl RestoreManager {
    pub fn restore<P: AsRef<Path>>(backup_path: P) -> Result<Schema, KcmError> {
        DatabaseFile::load(backup_path)
    }
}
