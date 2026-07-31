use kcm_core::types::*;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use std::sync::Mutex;

pub const WAL_MAGIC: &[u8; 5] = b"WALDB";
pub const WAL_VERSION: u8 = 1;

#[derive(Debug, Clone)]
pub enum WALEntry {
    Insert {
        subject: SubjectID,
        predicate: PredicateID,
        object: ObjectID,
        confidence: f64,
        timestamp: i64,
        context: ContextID,
    },
    Delete {
        row_id: u64,
    },
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
        let mut buffer = self.buffer.lock().unwrap();

        buffer.extend_from_slice(&1u8.to_le_bytes());
        buffer.extend_from_slice(&fact.subject.0.to_le_bytes());
        buffer.extend_from_slice(&fact.predicate.0.to_le_bytes());
        buffer.extend_from_slice(&fact.object.0.to_le_bytes());
        buffer.extend_from_slice(&fact.confidence.to_le_bytes());
        buffer.extend_from_slice(&fact.timestamp.to_le_bytes());
        buffer.extend_from_slice(&fact.context.0.to_le_bytes());

        if buffer.len() >= self.buffer_threshold {
            drop(buffer);
            self.flush_buffer()?;
        }

        Ok(())
    }

    pub fn append_delete(&self, row_id: u64) -> Result<(), KcmError> {
        let mut buffer = self.buffer.lock().unwrap();

        buffer.extend_from_slice(&2u8.to_le_bytes());
        buffer.extend_from_slice(&row_id.to_le_bytes());

        if buffer.len() >= self.buffer_threshold {
            drop(buffer);
            self.flush_buffer()?;
        }

        Ok(())
    }

    pub fn flush_buffer(&self) -> Result<(), KcmError> {
        let mut buffer = self.buffer.lock().unwrap();
        if buffer.is_empty() {
            return Ok(());
        }

        let mut file = self.file.lock().unwrap();
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
                    if offset + 28 > all_data.len() {
                        break;
                    }
                    let subject = u32::from_le_bytes([
                        all_data[offset],
                        all_data[offset + 1],
                        all_data[offset + 2],
                        all_data[offset + 3],
                    ]);
                    offset += 4;
                    let predicate = all_data[offset];
                    offset += 1;
                    let object = u32::from_le_bytes([
                        all_data[offset],
                        all_data[offset + 1],
                        all_data[offset + 2],
                        all_data[offset + 3],
                    ]);
                    offset += 4;
                    let mut cb = [0u8; 8];
                    cb.copy_from_slice(&all_data[offset..offset + 8]);
                    let confidence = f64::from_le_bytes(cb);
                    offset += 8;
                    let mut tb = [0u8; 8];
                    tb.copy_from_slice(&all_data[offset..offset + 8]);
                    let timestamp = i64::from_le_bytes(tb);
                    offset += 8;
                    let context = all_data[offset];
                    offset += 1;

                    callback(WALEntry::Insert {
                        subject: SubjectID(subject),
                        predicate: PredicateID(predicate),
                        object: ObjectID(object),
                        confidence,
                        timestamp,
                        context: ContextID(context),
                    })?;
                    count += 1;
                }
                2 => {
                    if offset + 8 > all_data.len() {
                        break;
                    }
                    let mut rb = [0u8; 8];
                    rb.copy_from_slice(&all_data[offset..offset + 8]);
                    let row_id = u64::from_le_bytes(rb);
                    offset += 8;

                    callback(WALEntry::Delete { row_id })?;
                    count += 1;
                }
                _ => {
                    break;
                }
            }
        }

        Ok(count)
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}
