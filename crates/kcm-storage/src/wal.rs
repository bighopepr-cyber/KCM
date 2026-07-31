use kcm_core::types::*;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use std::sync::Mutex;

pub const WAL_MAGIC: &[u8; 5] = b"WALDB";
pub const WAL_VERSION: u8 = 2;
pub const WAL_INSERT_SIZE: usize = 34;
pub const WAL_DELETE_SIZE: usize = 9;

#[derive(Debug, Clone)]
pub enum WALEntry {
    Insert {
        subject: SubjectID,
        predicate: PredicateID,
        object: ObjectID,
        confidence: f64,
        timestamp: i64,
        context: ContextID,
        version: i32,
        priority: i8,
        owner: u16,
    },
    Delete {
        row_id: u64,
    },
}

impl WALEntry {
    pub fn to_fact(&self) -> Option<Fact> {
        match self {
            WALEntry::Insert {
                subject,
                predicate,
                object,
                confidence,
                timestamp,
                context,
                version,
                priority,
                owner,
            } => Some(Fact {
                subject: *subject,
                predicate: *predicate,
                object: *object,
                confidence: *confidence,
                evidence: EvidenceID::UNKNOWN,
                timestamp: *timestamp,
                context: *context,
                version: *version,
                priority: *priority,
                owner: *owner,
            }),
            WALEntry::Delete { .. } => None,
        }
    }
}

pub struct WriteAheadLog {
    file: Mutex<File>,
    path: std::path::PathBuf,
    buffer: Mutex<Vec<u8>>,
    buffer_threshold: usize,
}

impl WriteAheadLog {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, KcmError> {
        let path = path.as_ref().to_path_buf();
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .map_err(|e| KcmError::Io(format!("Failed to open WAL: {}", e)))?;
        Ok(WriteAheadLog {
            file: Mutex::new(file),
            path,
            buffer: Mutex::new(Vec::with_capacity(65536)),
            buffer_threshold: 65536,
        })
    }

    pub fn append_fact(&self, fact: &Fact) -> Result<(), KcmError> {
        let mut buffer = self.buffer.lock().unwrap_or_else(|e| e.into_inner());
        buffer.extend_from_slice(&1u8.to_le_bytes());
        buffer.extend_from_slice(&fact.subject.0.to_le_bytes());
        buffer.extend_from_slice(&fact.predicate.0.to_le_bytes());
        buffer.extend_from_slice(&fact.object.0.to_le_bytes());
        buffer.extend_from_slice(&fact.confidence.to_le_bytes());
        buffer.extend_from_slice(&fact.timestamp.to_le_bytes());
        buffer.extend_from_slice(&fact.context.0.to_le_bytes());
        buffer.extend_from_slice(&fact.version.to_le_bytes());
        buffer.extend_from_slice(&fact.priority.to_le_bytes());
        buffer.extend_from_slice(&fact.owner.to_le_bytes());
        if buffer.len() >= self.buffer_threshold {
            drop(buffer);
            self.flush_buffer()?;
        }
        Ok(())
    }

    pub fn append_delete(&self, row_id: u64) -> Result<(), KcmError> {
        let mut buffer = self.buffer.lock().unwrap_or_else(|e| e.into_inner());
        buffer.extend_from_slice(&2u8.to_le_bytes());
        buffer.extend_from_slice(&row_id.to_le_bytes());
        if buffer.len() >= self.buffer_threshold {
            drop(buffer);
            self.flush_buffer()?;
        }
        Ok(())
    }

    pub fn flush_buffer(&self) -> Result<(), KcmError> {
        let mut buffer = self.buffer.lock().unwrap_or_else(|e| e.into_inner());
        if buffer.is_empty() {
            return Ok(());
        }
        let mut file = self.file.lock().unwrap_or_else(|e| e.into_inner());
        file.write_all(&buffer)
            .map_err(|e| KcmError::Io(format!("WAL write failed: {}", e)))?;
        file.sync_all()
            .map_err(|e| KcmError::Io(format!("WAL sync failed: {}", e)))?;
        buffer.clear();
        Ok(())
    }

    pub fn replay<F>(&self, mut callback: F) -> Result<usize, KcmError>
    where
        F: FnMut(WALEntry) -> Result<(), KcmError>,
    {
        let mut file = File::open(&self.path)
            .map_err(|e| KcmError::Io(format!("Failed to open WAL for replay: {}", e)))?;
        let mut count = 0;
        let mut all_data = Vec::new();
        file.read_to_end(&mut all_data)
            .map_err(|e| KcmError::Io(format!("WAL read error: {}", e)))?;

        let mut offset = 0;
        while offset < all_data.len() {
            let op_type = all_data[offset];
            offset += 1;
            match op_type {
                1 => {
                    if offset + 33 > all_data.len() {
                        break;
                    }
                    let subject = read_u32(&all_data, offset);
                    offset += 4;
                    let predicate = all_data[offset];
                    offset += 1;
                    let object = read_u32(&all_data, offset);
                    offset += 4;
                    let confidence = read_f64(&all_data, offset);
                    offset += 8;
                    let timestamp = read_i64(&all_data, offset);
                    offset += 8;
                    let context = all_data[offset];
                    offset += 1;
                    let version = read_i32(&all_data, offset);
                    offset += 4;
                    let priority = all_data[offset] as i8;
                    offset += 1;
                    let owner = read_u16(&all_data, offset);
                    offset += 2;

                    callback(WALEntry::Insert {
                        subject: SubjectID(subject),
                        predicate: PredicateID(predicate),
                        object: ObjectID(object),
                        confidence,
                        timestamp,
                        context: ContextID(context),
                        version,
                        priority,
                        owner,
                    })?;
                    count += 1;
                }
                2 => {
                    if offset + 8 > all_data.len() {
                        break;
                    }
                    let row_id = read_u64(&all_data, offset);
                    offset += 8;
                    callback(WALEntry::Delete { row_id })?;
                    count += 1;
                }
                _ => break,
            }
        }
        Ok(count)
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

fn read_u32(data: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes([
        data[offset],
        data[offset + 1],
        data[offset + 2],
        data[offset + 3],
    ])
}

fn read_u16(data: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes([data[offset], data[offset + 1]])
}

fn read_i32(data: &[u8], offset: usize) -> i32 {
    i32::from_le_bytes([
        data[offset],
        data[offset + 1],
        data[offset + 2],
        data[offset + 3],
    ])
}

fn read_u64(data: &[u8], offset: usize) -> u64 {
    u64::from_le_bytes([
        data[offset],
        data[offset + 1],
        data[offset + 2],
        data[offset + 3],
        data[offset + 4],
        data[offset + 5],
        data[offset + 6],
        data[offset + 7],
    ])
}

fn read_i64(data: &[u8], offset: usize) -> i64 {
    i64::from_le_bytes([
        data[offset],
        data[offset + 1],
        data[offset + 2],
        data[offset + 3],
        data[offset + 4],
        data[offset + 5],
        data[offset + 6],
        data[offset + 7],
    ])
}

fn read_f64(data: &[u8], offset: usize) -> f64 {
    f64::from_le_bytes([
        data[offset],
        data[offset + 1],
        data[offset + 2],
        data[offset + 3],
        data[offset + 4],
        data[offset + 5],
        data[offset + 6],
        data[offset + 7],
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wal_insert_entry_size() {
        let mut fact = Fact::new(SubjectID(1), PredicateID(0), ObjectID(2), 0.9).unwrap();
        fact.version = 3;
        fact.priority = 5;
        fact.owner = 42;
        let mut buffer = Vec::new();
        buffer.extend_from_slice(&1u8.to_le_bytes());
        buffer.extend_from_slice(&fact.subject.0.to_le_bytes());
        buffer.extend_from_slice(&fact.predicate.0.to_le_bytes());
        buffer.extend_from_slice(&fact.object.0.to_le_bytes());
        buffer.extend_from_slice(&fact.confidence.to_le_bytes());
        buffer.extend_from_slice(&fact.timestamp.to_le_bytes());
        buffer.extend_from_slice(&fact.context.0.to_le_bytes());
        buffer.extend_from_slice(&fact.version.to_le_bytes());
        buffer.extend_from_slice(&fact.priority.to_le_bytes());
        buffer.extend_from_slice(&fact.owner.to_le_bytes());
        assert_eq!(buffer.len(), WAL_INSERT_SIZE);
    }
}
