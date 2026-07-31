use crate::column::Schema;
use kcm_core::types::*;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

pub const DB_MAGIC: &[u8; 5] = b"KCMDB";
pub const DB_VERSION: u8 = 1;

pub struct DatabaseFile;

impl DatabaseFile {
    pub fn save<P: AsRef<Path>>(schema: &Schema, path: P) -> Result<(), KcmError> {
        let path = path.as_ref();
        let file = File::create(path)
            .map_err(|e| KcmError::Io(format!("Failed to create DB file: {}", e)))?;
        let mut writer = BufWriter::new(file);

        writer
            .write_all(DB_MAGIC)
            .map_err(|e| KcmError::Io(e.to_string()))?;
        writer
            .write_all(&[DB_VERSION])
            .map_err(|e| KcmError::Io(e.to_string()))?;

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as i64;

        writer
            .write_all(&(schema.len() as u64).to_le_bytes())
            .map_err(|e| KcmError::Io(e.to_string()))?;
        writer
            .write_all(&[11u8])
            .map_err(|e| KcmError::Io(e.to_string()))?;
        writer
            .write_all(&now.to_le_bytes())
            .map_err(|e| KcmError::Io(e.to_string()))?;
        writer
            .write_all(&now.to_le_bytes())
            .map_err(|e| KcmError::Io(e.to_string()))?;

        Self::serialize_column_raw(&mut writer, schema.subject_col.as_slice())?;
        Self::serialize_column_raw(&mut writer, schema.predicate_col.as_slice())?;
        Self::serialize_column_raw(&mut writer, schema.object_col.as_slice())?;
        Self::serialize_column_raw(&mut writer, schema.confidence_col.as_slice())?;
        Self::serialize_column_raw(&mut writer, schema.evidence_col.as_slice())?;
        Self::serialize_column_raw(&mut writer, schema.timestamp_col.as_slice())?;
        Self::serialize_column_raw(&mut writer, schema.context_col.as_slice())?;
        Self::serialize_column_raw(&mut writer, schema.version_col.as_slice())?;
        Self::serialize_column_raw(&mut writer, schema.priority_col.as_slice())?;
        Self::serialize_column_raw(&mut writer, schema.owner_col.as_slice())?;

        let checksum = Self::compute_checksum(path)?;
        writer
            .write_all(&checksum)
            .map_err(|e| KcmError::Io(e.to_string()))?;

        writer.flush().map_err(|e| KcmError::Io(e.to_string()))?;

        let file = writer
            .into_inner()
            .map_err(|e| KcmError::Io(e.to_string()))?;
        file.sync_all().map_err(|e| KcmError::Io(e.to_string()))?;

        Ok(())
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Schema, KcmError> {
        let path = path.as_ref();
        let file =
            File::open(path).map_err(|e| KcmError::Io(format!("Failed to open DB file: {}", e)))?;
        let mut reader = BufReader::new(file);

        let mut magic = [0u8; 5];
        reader
            .read_exact(&mut magic)
            .map_err(|e| KcmError::Corrupted(e.to_string()))?;
        if &magic != DB_MAGIC {
            return Err(KcmError::Corrupted("Invalid database magic".to_string()));
        }

        let mut version = [0u8; 1];
        reader
            .read_exact(&mut version)
            .map_err(|e| KcmError::Corrupted(e.to_string()))?;
        if version[0] != DB_VERSION {
            return Err(KcmError::Corrupted(format!(
                "Unsupported version: {}",
                version[0]
            )));
        }

        let mut row_count_bytes = [0u8; 8];
        reader
            .read_exact(&mut row_count_bytes)
            .map_err(|e| KcmError::Corrupted(e.to_string()))?;
        let row_count = u64::from_le_bytes(row_count_bytes);

        let mut _col_count = [0u8; 1];
        reader
            .read_exact(&mut _col_count)
            .map_err(|e| KcmError::Corrupted(e.to_string()))?;

        let mut _created = [0u8; 8];
        reader
            .read_exact(&mut _created)
            .map_err(|e| KcmError::Corrupted(e.to_string()))?;
        let mut _modified = [0u8; 8];
        reader
            .read_exact(&mut _modified)
            .map_err(|e| KcmError::Corrupted(e.to_string()))?;

        let mut schema = Schema::new(row_count as usize)?;

        Self::deserialize_column_raw(&mut reader, &mut schema.subject_col, row_count as usize)?;
        Self::deserialize_column_raw(&mut reader, &mut schema.predicate_col, row_count as usize)?;
        Self::deserialize_column_raw(&mut reader, &mut schema.object_col, row_count as usize)?;
        Self::deserialize_column_raw(&mut reader, &mut schema.confidence_col, row_count as usize)?;
        Self::deserialize_column_raw(&mut reader, &mut schema.evidence_col, row_count as usize)?;
        Self::deserialize_column_raw(&mut reader, &mut schema.timestamp_col, row_count as usize)?;
        Self::deserialize_column_raw(&mut reader, &mut schema.context_col, row_count as usize)?;
        Self::deserialize_column_raw(&mut reader, &mut schema.version_col, row_count as usize)?;
        Self::deserialize_column_raw(&mut reader, &mut schema.priority_col, row_count as usize)?;
        Self::deserialize_column_raw(&mut reader, &mut schema.owner_col, row_count as usize)?;

        Ok(schema)
    }

    fn serialize_column_raw<T: Copy>(
        writer: &mut BufWriter<File>,
        column: &[T],
    ) -> Result<(), KcmError> {
        let len = column.len();
        writer
            .write_all(&(len as u64).to_le_bytes())
            .map_err(|e| KcmError::Io(e.to_string()))?;

        let bytes = unsafe {
            std::slice::from_raw_parts(column.as_ptr() as *const u8, std::mem::size_of_val(column))
        };

        writer
            .write_all(bytes)
            .map_err(|e| KcmError::Io(e.to_string()))?;
        Ok(())
    }

    fn deserialize_column_raw<T: Copy>(
        reader: &mut BufReader<File>,
        column: &mut crate::column::Column<T>,
        expected_count: usize,
    ) -> Result<(), KcmError> {
        let mut len_bytes = [0u8; 8];
        reader
            .read_exact(&mut len_bytes)
            .map_err(|e| KcmError::Corrupted(e.to_string()))?;
        let len = u64::from_le_bytes(len_bytes) as usize;

        if len != expected_count {
            return Err(KcmError::Corrupted(format!(
                "Column length mismatch: expected {}, got {}",
                expected_count, len
            )));
        }

        let mut data = vec![0u8; len * std::mem::size_of::<T>()];
        reader
            .read_exact(&mut data)
            .map_err(|e| KcmError::Corrupted(e.to_string()))?;

        let values: Vec<T> =
            unsafe { std::slice::from_raw_parts(data.as_ptr() as *const T, len).to_vec() };

        for value in values {
            column.append(value)?;
        }

        Ok(())
    }

    pub fn compute_checksum<P: AsRef<Path>>(path: P) -> Result<[u8; 32], KcmError> {
        let mut file = File::open(path).map_err(|e| KcmError::Io(e.to_string()))?;
        let mut hasher = blake3::Hasher::new();

        let mut buffer = [0u8; 8192];
        loop {
            match file.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => {
                    hasher.update(&buffer[..n]);
                }
                Err(e) => return Err(KcmError::Io(e.to_string())),
            }
        }

        let hash = hasher.finalize();
        let mut result = [0u8; 32];
        result.copy_from_slice(hash.as_bytes());
        Ok(result)
    }

    pub fn verify<P: AsRef<Path>>(path: P) -> Result<bool, KcmError> {
        let path = path.as_ref();
        let stored_checksum = {
            let mut file = File::open(path).map_err(|e| KcmError::Io(e.to_string()))?;
            let metadata = std::fs::metadata(path).map_err(|e| KcmError::Io(e.to_string()))?;

            let file_size = metadata.len();
            if file_size < 32 {
                return Ok(false);
            }

            let mut checksum_bytes = [0u8; 32];
            let seek_pos = file_size - 32;
            use std::io::Seek;
            file.seek(std::io::SeekFrom::Start(seek_pos))
                .map_err(|e| KcmError::Io(e.to_string()))?;
            file.read_exact(&mut checksum_bytes)
                .map_err(|e| KcmError::Io(e.to_string()))?;
            checksum_bytes
        };

        let computed = Self::compute_checksum(path)?;
        Ok(stored_checksum == computed)
    }
}
