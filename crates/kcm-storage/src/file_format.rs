use crate::column::Schema;
use kcm_core::types::*;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Seek, Write};
use std::path::Path;

pub const DB_MAGIC: &[u8; 5] = b"KCMDB";
pub const DB_VERSION: u8 = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ColumnCodecId {
    None = 0,
    Zstd = 1,
    Lz4 = 2,
    Rle = 3,
}

impl ColumnCodecId {
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::None),
            1 => Some(Self::Zstd),
            2 => Some(Self::Lz4),
            3 => Some(Self::Rle),
            _ => None,
        }
    }
}

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
            .write_all(&[10u8])
            .map_err(|e| KcmError::Io(e.to_string()))?;
        writer
            .write_all(&now.to_le_bytes())
            .map_err(|e| KcmError::Io(e.to_string()))?;
        writer
            .write_all(&now.to_le_bytes())
            .map_err(|e| KcmError::Io(e.to_string()))?;

        let columns: Vec<(&[u8], usize, ColumnCodecId)> = vec![
            (
                as_bytes(schema.subject_col.as_slice()),
                schema.subject_col.len(),
                ColumnCodecId::Zstd,
            ),
            (
                as_bytes(schema.predicate_col.as_slice()),
                schema.predicate_col.len(),
                ColumnCodecId::Rle,
            ),
            (
                as_bytes(schema.object_col.as_slice()),
                schema.object_col.len(),
                ColumnCodecId::Zstd,
            ),
            (
                as_bytes(schema.confidence_col.as_slice()),
                schema.confidence_col.len(),
                ColumnCodecId::Zstd,
            ),
            (
                as_bytes(schema.evidence_col.as_slice()),
                schema.evidence_col.len(),
                ColumnCodecId::Rle,
            ),
            (
                as_bytes(schema.timestamp_col.as_slice()),
                schema.timestamp_col.len(),
                ColumnCodecId::Zstd,
            ),
            (
                as_bytes(schema.context_col.as_slice()),
                schema.context_col.len(),
                ColumnCodecId::Rle,
            ),
            (
                as_bytes(schema.version_col.as_slice()),
                schema.version_col.len(),
                ColumnCodecId::Lz4,
            ),
            (
                as_bytes(schema.priority_col.as_slice()),
                schema.priority_col.len(),
                ColumnCodecId::Rle,
            ),
            (
                as_bytes(schema.owner_col.as_slice()),
                schema.owner_col.len(),
                ColumnCodecId::Zstd,
            ),
        ];

        // Column data is written as raw bytes. The codec_id field records which
        // compression algorithm SHOULD be applied. Currently, compression is
        // applied at the Column level via compress_all_columns(), not at the
        // file format level. This keeps the save/load path simple.
        for (data, elem_count, codec) in &columns {
            writer
                .write_all(&(*elem_count as u64).to_le_bytes())
                .map_err(|e| KcmError::Io(e.to_string()))?;
            writer
                .write_all(&[*codec as u8])
                .map_err(|e| KcmError::Io(e.to_string()))?;
            writer
                .write_all(&(data.len() as u64).to_le_bytes())
                .map_err(|e| KcmError::Io(e.to_string()))?;
            writer
                .write_all(data)
                .map_err(|e| KcmError::Io(e.to_string()))?;
        }

        let tomb_bytes = schema.tombstone_bytes();
        let tomb_len = schema.tombstone_len();
        writer
            .write_all(&(tomb_len as u64).to_le_bytes())
            .map_err(|e| KcmError::Io(e.to_string()))?;
        writer
            .write_all(&(tomb_bytes.len() as u64).to_le_bytes())
            .map_err(|e| KcmError::Io(e.to_string()))?;
        writer
            .write_all(tomb_bytes)
            .map_err(|e| KcmError::Io(e.to_string()))?;

        writer.flush().map_err(|e| KcmError::Io(e.to_string()))?;
        let checksum = Self::compute_checksum(path)?;
        writer
            .write_all(&checksum)
            .map_err(|e| KcmError::Io(e.to_string()))?;

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

        let mut buf = [0u8; 1];
        reader
            .read_exact(&mut buf)
            .map_err(|e| KcmError::Corrupted(e.to_string()))?;
        let mut buf = [0u8; 8];
        reader
            .read_exact(&mut buf)
            .map_err(|e| KcmError::Corrupted(e.to_string()))?;
        let mut buf = [0u8; 8];
        reader
            .read_exact(&mut buf)
            .map_err(|e| KcmError::Corrupted(e.to_string()))?;

        let mut schema = Schema::new((row_count as usize * 2).max(1000))?;

        Self::deserialize_column_block(&mut reader, &mut schema.subject_col)?;
        Self::deserialize_column_block(&mut reader, &mut schema.predicate_col)?;
        Self::deserialize_column_block(&mut reader, &mut schema.object_col)?;
        Self::deserialize_column_block(&mut reader, &mut schema.confidence_col)?;
        Self::deserialize_column_block(&mut reader, &mut schema.evidence_col)?;
        Self::deserialize_column_block(&mut reader, &mut schema.timestamp_col)?;
        Self::deserialize_column_block(&mut reader, &mut schema.context_col)?;
        Self::deserialize_column_block(&mut reader, &mut schema.version_col)?;
        Self::deserialize_column_block(&mut reader, &mut schema.priority_col)?;
        Self::deserialize_column_block(&mut reader, &mut schema.owner_col)?;

        let mut tomb_row_count = [0u8; 8];
        reader
            .read_exact(&mut tomb_row_count)
            .map_err(|e| KcmError::Corrupted(e.to_string()))?;
        let tomb_len = u64::from_le_bytes(tomb_row_count) as usize;

        let mut tomb_byte_len = [0u8; 8];
        reader
            .read_exact(&mut tomb_byte_len)
            .map_err(|e| KcmError::Corrupted(e.to_string()))?;
        let tomb_bytes_len = u64::from_le_bytes(tomb_byte_len) as usize;

        if tomb_bytes_len > 0 {
            let mut tomb_data = vec![0u8; tomb_bytes_len];
            reader
                .read_exact(&mut tomb_data)
                .map_err(|e| KcmError::Corrupted(e.to_string()))?;
            schema.restore_tombstones(&tomb_data, tomb_len);
        }

        Ok(schema)
    }

    fn deserialize_column_block<T: Copy>(
        reader: &mut BufReader<File>,
        column: &mut crate::column::Column<T>,
    ) -> Result<(), KcmError> {
        let mut len_bytes = [0u8; 8];
        reader
            .read_exact(&mut len_bytes)
            .map_err(|e| KcmError::Corrupted(e.to_string()))?;
        let len = u64::from_le_bytes(len_bytes) as usize;

        let mut codec_byte = [0u8; 1];
        reader
            .read_exact(&mut codec_byte)
            .map_err(|e| KcmError::Corrupted(e.to_string()))?;
        let codec = ColumnCodecId::from_u8(codec_byte[0])
            .ok_or_else(|| KcmError::Corrupted(format!("Unknown codec ID: {}", codec_byte[0])))?;
        // Codec ID is stored for metadata. Raw column data is written uncompressed.
        // Compression is applied at the Schema level via compress_all_columns().

        let _codec = codec;

        let mut compressed_size = [0u8; 8];
        reader
            .read_exact(&mut compressed_size)
            .map_err(|e| KcmError::Corrupted(e.to_string()))?;
        let compressed_len = u64::from_le_bytes(compressed_size) as usize;

        let mut data = vec![0u8; compressed_len];
        reader
            .read_exact(&mut data)
            .map_err(|e| KcmError::Corrupted(e.to_string()))?;

        let expected_bytes = len * std::mem::size_of::<T>();
        if compressed_len != expected_bytes {
            return Err(KcmError::Corrupted(format!(
                "Column size mismatch: expected {} bytes, got {}",
                expected_bytes, compressed_len
            )));
        }

        let type_size = std::mem::size_of::<T>();
        let mut values = Vec::with_capacity(len);
        for i in 0..len {
            let offset = i * type_size;
            if offset + type_size > data.len() {
                return Err(KcmError::Corrupted(format!(
                    "Data too short at element {}: need {} bytes, have {}",
                    i,
                    offset + type_size,
                    data.len()
                )));
            }
            let mut buf = [0u8; 8];
            let copy_len = type_size.min(8);
            buf[..copy_len].copy_from_slice(&data[offset..offset + copy_len]);
            // SAFETY: buf contains valid bytes from the serialized column data.
            // For integer/float types (u8..u64, i8..i64, f32, f64), all bit patterns
            // are valid values. The source data was serialized from the same type.
            values.push(unsafe { std::ptr::read(buf.as_ptr() as *const T) });
        }

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

    fn compute_checksum_range<P: AsRef<Path>>(
        path: P,
        start: u64,
        end: u64,
    ) -> Result<[u8; 32], KcmError> {
        let mut file = File::open(path).map_err(|e| KcmError::Io(e.to_string()))?;
        file.seek(std::io::SeekFrom::Start(start))
            .map_err(|e| KcmError::Io(e.to_string()))?;
        let mut hasher = blake3::Hasher::new();
        let mut buffer = [0u8; 8192];
        let mut remaining = end - start;
        while remaining > 0 {
            let to_read = (remaining as usize).min(buffer.len());
            match file.read(&mut buffer[..to_read]) {
                Ok(0) => break,
                Ok(n) => {
                    hasher.update(&buffer[..n]);
                    remaining -= n as u64;
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
            file.seek(std::io::SeekFrom::Start(file_size - 32))
                .map_err(|e| KcmError::Io(e.to_string()))?;
            file.read_exact(&mut checksum_bytes)
                .map_err(|e| KcmError::Io(e.to_string()))?;
            checksum_bytes
        };
        let metadata = std::fs::metadata(path).map_err(|e| KcmError::Io(e.to_string()))?;
        let computed = Self::compute_checksum_range(path, 0, metadata.len() - 32)?;
        Ok(stored_checksum == computed)
    }
}

fn as_bytes<T>(slice: &[T]) -> &[u8] {
    // SAFETY: reinterpret &[T] as &[u8]. This is safe because:
    // 1. slice is a valid, properly aligned, initialized &[T]
    // 2. size_of_val computes the exact byte count
    // 3. u8 has alignment of 1, always satisfied by T alignment
    // 4. Resulting &[u8] has same lifetime as input slice
    unsafe { std::slice::from_raw_parts(slice.as_ptr() as *const u8, std::mem::size_of_val(slice)) }
}
