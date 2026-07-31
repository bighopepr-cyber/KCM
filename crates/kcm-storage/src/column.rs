use kcm_core::bitmap::Bitmap;
use kcm_core::types::*;
use kcm_core::vec::DenseVec;

use crate::compress::{Compressor, Lz4Compressor, NoopCompressor, ZstdCompressor};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ColumnEncoding {
    Identity,
    Dictionary,
    Delta,
    FrameOfReference,
    Rle,
    Gorilla,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CompressionCodec {
    None,
    Zstd,
    Lz4,
    Rle,
}

fn make_compressor(codec: CompressionCodec) -> Box<dyn Compressor> {
    match codec {
        CompressionCodec::None => Box::new(NoopCompressor),
        CompressionCodec::Zstd => Box::new(ZstdCompressor::default_level()),
        CompressionCodec::Lz4 => Box::new(Lz4Compressor::default_level()),
        CompressionCodec::Rle => Box::new(crate::compress::RleCompressor),
    }
}

#[derive(Clone)]
pub struct Column<T: Copy> {
    data: DenseVec<T>,
    encoding: ColumnEncoding,
    compression: CompressionCodec,
    row_count: u64,
    raw_bytes: Vec<u8>,
    compressed: bool,
}

impl<T: Copy> Column<T> {
    pub fn new(
        capacity: usize,
        encoding: ColumnEncoding,
        compression: CompressionCodec,
    ) -> Result<Self, KcmError> {
        let data = DenseVec::new(capacity).map_err(KcmError::Io)?;
        Ok(Column {
            data,
            encoding,
            compression,
            row_count: 0,
            raw_bytes: Vec::new(),
            compressed: false,
        })
    }

    pub fn append(&mut self, value: T) -> Result<(), KcmError> {
        self.data.push(value).map_err(KcmError::Io)?;
        self.row_count += 1;
        Ok(())
    }

    pub fn get(&self, idx: usize) -> Option<T> {
        if idx >= self.row_count as usize {
            return None;
        }
        Some(self.data[idx])
    }

    pub fn set(&mut self, idx: usize, value: T) -> Result<(), KcmError> {
        if idx >= self.row_count as usize {
            return Err(KcmError::InvalidArgument(format!(
                "Index {} out of bounds (len {})",
                idx, self.row_count
            )));
        }
        self.data[idx] = value;
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.row_count as usize
    }
    pub fn is_empty(&self) -> bool {
        self.row_count == 0
    }
    pub fn as_slice(&self) -> &[T] {
        self.data.as_slice()
    }
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }
    pub fn encoding(&self) -> ColumnEncoding {
        self.encoding
    }
    pub fn compression(&self) -> CompressionCodec {
        self.compression
    }

    pub fn compress_data(&mut self) -> Result<(), KcmError> {
        let slice = self.data.as_slice();
        let byte_slice = unsafe {
            std::slice::from_raw_parts(slice.as_ptr() as *const u8, std::mem::size_of_val(slice))
        };
        let compressor = make_compressor(self.compression);
        self.raw_bytes = compressor.compress(byte_slice)?;
        self.compressed = true;
        Ok(())
    }

    pub fn decompress_data(&mut self) -> Result<(), KcmError> {
        if !self.compressed || self.raw_bytes.is_empty() {
            return Ok(());
        }
        let expected = self.row_count as usize * std::mem::size_of::<T>();
        let compressor = make_compressor(self.compression);
        let decompressed = compressor.decompress(&self.raw_bytes, expected)?;
        let ptr = self.data.as_mut_slice().as_mut_ptr();
        unsafe {
            std::ptr::copy_nonoverlapping(
                decompressed.as_ptr(),
                ptr as *mut u8,
                decompressed.len().min(expected),
            );
        }
        self.compressed = false;
        self.raw_bytes.clear();
        Ok(())
    }
}

pub type SubjectColumn = Column<u32>;
pub type ObjectColumn = Column<u32>;
pub type PredicateColumn = Column<u8>;
pub type ContextColumn = Column<u8>;
pub type EvidenceColumn = Column<u8>;
pub type ConfidenceColumn = Column<f64>;
pub type TimestampColumn = Column<i64>;
pub type VersionColumn = Column<i32>;
pub type PriorityColumn = Column<i8>;
pub type OwnerColumn = Column<u16>;

#[derive(Clone)]
pub struct Schema {
    pub subject_col: SubjectColumn,
    pub predicate_col: PredicateColumn,
    pub object_col: ObjectColumn,
    pub confidence_col: ConfidenceColumn,
    pub evidence_col: EvidenceColumn,
    pub timestamp_col: TimestampColumn,
    pub context_col: ContextColumn,
    pub version_col: VersionColumn,
    pub priority_col: PriorityColumn,
    pub owner_col: OwnerColumn,
    tombstones: Bitmap,
}

impl Schema {
    pub fn new(capacity: usize) -> Result<Self, KcmError> {
        Ok(Schema {
            subject_col: SubjectColumn::new(
                capacity,
                ColumnEncoding::Dictionary,
                CompressionCodec::Zstd,
            )?,
            predicate_col: PredicateColumn::new(
                capacity,
                ColumnEncoding::Dictionary,
                CompressionCodec::Rle,
            )?,
            object_col: ObjectColumn::new(
                capacity,
                ColumnEncoding::Dictionary,
                CompressionCodec::Zstd,
            )?,
            confidence_col: ConfidenceColumn::new(
                capacity,
                ColumnEncoding::Gorilla,
                CompressionCodec::Zstd,
            )?,
            evidence_col: EvidenceColumn::new(
                capacity,
                ColumnEncoding::Dictionary,
                CompressionCodec::Rle,
            )?,
            timestamp_col: TimestampColumn::new(
                capacity,
                ColumnEncoding::Delta,
                CompressionCodec::Zstd,
            )?,
            context_col: ContextColumn::new(
                capacity,
                ColumnEncoding::Dictionary,
                CompressionCodec::Rle,
            )?,
            version_col: VersionColumn::new(
                capacity,
                ColumnEncoding::Delta,
                CompressionCodec::Lz4,
            )?,
            priority_col: PriorityColumn::new(
                capacity,
                ColumnEncoding::Identity,
                CompressionCodec::Rle,
            )?,
            owner_col: OwnerColumn::new(
                capacity,
                ColumnEncoding::Dictionary,
                CompressionCodec::Zstd,
            )?,
            tombstones: Bitmap::new(capacity),
        })
    }

    pub fn append_fact(&mut self, fact: &Fact) -> Result<(), KcmError> {
        self.subject_col.append(fact.subject.0)?;
        self.predicate_col.append(fact.predicate.0)?;
        self.object_col.append(fact.object.0)?;
        self.confidence_col.append(fact.confidence)?;
        self.evidence_col.append(fact.evidence.0)?;
        self.timestamp_col.append(fact.timestamp)?;
        self.context_col.append(fact.context.0)?;
        self.version_col.append(fact.version)?;
        self.priority_col.append(fact.priority)?;
        self.owner_col.append(fact.owner)?;
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.subject_col.len()
    }
    pub fn is_empty(&self) -> bool {
        self.subject_col.is_empty()
    }
    pub fn active_count(&self) -> usize {
        self.len() - self.tombstones.count_ones()
    }

    pub fn get_fact(&self, idx: usize) -> Option<Fact> {
        if self.tombstones.get(idx) {
            return None;
        }
        Some(Fact {
            subject: SubjectID(self.subject_col.get(idx)?),
            predicate: PredicateID(self.predicate_col.get(idx)?),
            object: ObjectID(self.object_col.get(idx)?),
            confidence: self.confidence_col.get(idx)?,
            evidence: EvidenceID(self.evidence_col.get(idx)?),
            timestamp: self.timestamp_col.get(idx)?,
            context: ContextID(self.context_col.get(idx)?),
            version: self.version_col.get(idx)?,
            priority: self.priority_col.get(idx)?,
            owner: self.owner_col.get(idx)?,
        })
    }

    pub fn delete_fact(&mut self, idx: usize) -> Result<(), KcmError> {
        if idx >= self.len() {
            return Err(KcmError::InvalidArgument(format!(
                "Index {} out of bounds (len {})",
                idx,
                self.len()
            )));
        }
        self.tombstones.set(idx);
        Ok(())
    }

    pub fn update_fact(&mut self, idx: usize, fact: &Fact) -> Result<(), KcmError> {
        if idx >= self.len() {
            return Err(KcmError::InvalidArgument(format!(
                "Index {} out of bounds (len {})",
                idx,
                self.len()
            )));
        }
        self.subject_col.set(idx, fact.subject.0)?;
        self.predicate_col.set(idx, fact.predicate.0)?;
        self.object_col.set(idx, fact.object.0)?;
        self.confidence_col.set(idx, fact.confidence)?;
        self.evidence_col.set(idx, fact.evidence.0)?;
        self.timestamp_col.set(idx, fact.timestamp)?;
        self.context_col.set(idx, fact.context.0)?;
        self.version_col.set(idx, fact.version)?;
        self.priority_col.set(idx, fact.priority)?;
        self.owner_col.set(idx, fact.owner)?;
        Ok(())
    }

    pub fn is_deleted(&self, idx: usize) -> bool {
        self.tombstones.get(idx)
    }

    pub fn tombstone_bytes(&self) -> &[u8] {
        self.tombstones.as_bytes()
    }

    pub fn tombstone_len(&self) -> usize {
        self.tombstones.len()
    }

    pub fn restore_tombstones(&mut self, bytes: &[u8], len: usize) {
        self.tombstones = kcm_core::bitmap::Bitmap::from_bytes(bytes, len);
    }

    pub fn iter_active(&self) -> impl Iterator<Item = (usize, Fact)> + '_ {
        (0..self.len()).filter_map(move |idx| {
            if self.tombstones.get(idx) {
                None
            } else {
                self.get_fact(idx).map(|f| (idx, f))
            }
        })
    }

    pub fn compress_all_columns(&mut self) -> Result<(), KcmError> {
        self.subject_col.compress_data()?;
        self.predicate_col.compress_data()?;
        self.object_col.compress_data()?;
        self.confidence_col.compress_data()?;
        self.evidence_col.compress_data()?;
        self.timestamp_col.compress_data()?;
        self.context_col.compress_data()?;
        self.version_col.compress_data()?;
        self.priority_col.compress_data()?;
        self.owner_col.compress_data()?;
        Ok(())
    }

    pub fn decompress_all_columns(&mut self) -> Result<(), KcmError> {
        self.subject_col.decompress_data()?;
        self.predicate_col.decompress_data()?;
        self.object_col.decompress_data()?;
        self.confidence_col.decompress_data()?;
        self.evidence_col.decompress_data()?;
        self.timestamp_col.decompress_data()?;
        self.context_col.decompress_data()?;
        self.version_col.decompress_data()?;
        self.priority_col.decompress_data()?;
        self.owner_col.decompress_data()?;
        Ok(())
    }
}
