use kcm_core::bitmap::Bitmap;
use kcm_core::types::*;
use kcm_core::vec::DenseVec;

use crate::compress::{Compressor, Lz4Compressor, NoopCompressor, ZstdCompressor};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ColumnEncoding {
    Identity,
    Dictionary,
    Delta,
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

fn encode_delta_i64(values: &[i64]) -> Vec<u8> {
    if values.is_empty() {
        return Vec::new();
    }
    let mut result = Vec::with_capacity(values.len() * 8);
    result.extend_from_slice(&values[0].to_le_bytes());
    for i in 1..values.len() {
        let delta = values[i].wrapping_sub(values[i - 1]);
        result.extend_from_slice(&delta.to_le_bytes());
    }
    result
}

fn decode_delta_i64(data: &[u8], count: usize) -> Result<Vec<i64>, KcmError> {
    if count == 0 {
        return Ok(Vec::new());
    }
    if data.len() < count * 8 {
        return Err(KcmError::Corrupted(format!(
            "Delta decode i64: expected {} bytes, got {}",
            count * 8,
            data.len()
        )));
    }
    let mut values = Vec::with_capacity(count);
    let first = i64::from_le_bytes([
        data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
    ]);
    values.push(first);
    for i in 1..count {
        let offset = i * 8;
        let delta = i64::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
            data[offset + 4],
            data[offset + 5],
            data[offset + 6],
            data[offset + 7],
        ]);
        values.push(values[i - 1].wrapping_add(delta));
    }
    Ok(values)
}

fn encode_delta_i32(values: &[i32]) -> Vec<u8> {
    if values.is_empty() {
        return Vec::new();
    }
    let mut result = Vec::with_capacity(values.len() * 4);
    result.extend_from_slice(&values[0].to_le_bytes());
    for i in 1..values.len() {
        let delta = values[i].wrapping_sub(values[i - 1]);
        result.extend_from_slice(&delta.to_le_bytes());
    }
    result
}

fn decode_delta_i32(data: &[u8], count: usize) -> Result<Vec<i32>, KcmError> {
    if count == 0 {
        return Ok(Vec::new());
    }
    if data.len() < count * 4 {
        return Err(KcmError::Corrupted(format!(
            "Delta decode i32: expected {} bytes, got {}",
            count * 4,
            data.len()
        )));
    }
    let mut values = Vec::with_capacity(count);
    let first = i32::from_le_bytes([data[0], data[1], data[2], data[3]]);
    values.push(first);
    for i in 1..count {
        let offset = i * 4;
        let delta = i32::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
        ]);
        values.push(values[i - 1].wrapping_add(delta));
    }
    Ok(values)
}

fn encode_gorilla_f64(values: &[f64]) -> Vec<u8> {
    if values.is_empty() {
        return Vec::new();
    }
    let mut result = Vec::with_capacity(values.len() * 8);
    result.extend_from_slice(&values[0].to_le_bytes());
    let mut prev_bits = values[0].to_bits();
    for &val in &values[1..] {
        let cur_bits = val.to_bits();
        let xor = cur_bits ^ prev_bits;
        if xor == 0 {
            result.push(0x00);
        } else {
            result.push(0x01);
            let leading = xor.leading_zeros() as u8;
            let trailing = xor.trailing_zeros() as u8;
            result.push(leading);
            let shifted = xor >> trailing;
            let shifted_bytes = shifted.to_le_bytes();
            result.extend_from_slice(&shifted_bytes);
            result.push(trailing);
        }
        prev_bits = cur_bits;
    }
    result
}

fn decode_gorilla_f64(data: &[u8], count: usize) -> Result<Vec<f64>, KcmError> {
    if count == 0 {
        return Ok(Vec::new());
    }
    if data.len() < 8 {
        return Err(KcmError::Corrupted(
            "Gorilla decode: insufficient data for first value".to_string(),
        ));
    }
    let mut values = Vec::with_capacity(count);
    let first = f64::from_le_bytes([
        data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
    ]);
    values.push(first);
    let mut prev_bits = first.to_bits();
    let mut offset = 8;
    for _ in 1..count {
        if offset >= data.len() {
            return Err(KcmError::Corrupted(format!(
                "Gorilla decode: insufficient data at offset {}, have {}",
                offset,
                data.len()
            )));
        }
        let flag = data[offset];
        offset += 1;
        if flag == 0x00 {
            values.push(f64::from_bits(prev_bits));
        } else {
            if offset + 9 > data.len() {
                return Err(KcmError::Corrupted(format!(
                    "Gorilla decode: need 9 bytes for entry at offset {}, have {}",
                    offset,
                    data.len() - offset
                )));
            }
            let leading = data[offset];
            offset += 1;
            let mut shifted_bytes = [0u8; 8];
            shifted_bytes.copy_from_slice(&data[offset..offset + 8]);
            offset += 8;
            let trailing = data[offset];
            offset += 1;
            let shifted = u64::from_le_bytes(shifted_bytes);
            let xor = shifted << trailing;
            let cur_bits = prev_bits ^ xor;
            let val = f64::from_bits(cur_bits);
            values.push(val);
            let _ = leading;
            prev_bits = cur_bits;
        }
    }
    Ok(values)
}

fn encode_identity<T: Copy>(values: &[T]) -> Vec<u8> {
    let byte_slice = unsafe {
        std::slice::from_raw_parts(values.as_ptr() as *const u8, std::mem::size_of_val(values))
    };
    byte_slice.to_vec()
}

fn decode_to_slice<T: Copy>(data: &[u8], count: usize) -> Result<Vec<T>, KcmError> {
    let expected = count * std::mem::size_of::<T>();
    if data.len() < expected {
        return Err(KcmError::Corrupted(format!(
            "Decode: expected {} bytes for {} elements, got {}",
            expected,
            count,
            data.len()
        )));
    }
    let mut values = Vec::with_capacity(count);
    let type_size = std::mem::size_of::<T>();
    for i in 0..count {
        let offset = i * type_size;
        let mut buf = [0u8; 8];
        let copy_len = type_size.min(8);
        buf[..copy_len].copy_from_slice(&data[offset..offset + copy_len]);
        values.push(unsafe { std::ptr::read(buf.as_ptr() as *const T) });
    }
    Ok(values)
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
        let data = DenseVec::new(capacity)?;
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
        self.data.push(value)?;
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
        let encoded = encode_column::<T>(self.data.as_slice(), self.encoding)?;
        let compressor = make_compressor(self.compression);
        self.raw_bytes = compressor.compress(&encoded)?;
        self.compressed = true;
        Ok(())
    }

    pub fn decompress_data(&mut self) -> Result<(), KcmError> {
        if !self.compressed || self.raw_bytes.is_empty() {
            return Ok(());
        }
        let compressor = make_compressor(self.compression);
        let decompressed = compressor.decompress(&self.raw_bytes, 0)?;
        let values: Vec<T> =
            decode_column::<T>(&decompressed, self.row_count as usize, self.encoding)?;
        let ptr = self.data.as_mut_slice().as_mut_ptr();
        let expected = values.len() * std::mem::size_of::<T>();
        let raw_len = decompressed.len();
        if raw_len < expected {
            return Err(KcmError::Corrupted(format!(
                "Decompression size mismatch: got {} bytes, expected {} bytes",
                raw_len, expected
            )));
        }
        unsafe {
            std::ptr::copy_nonoverlapping(values.as_ptr() as *const u8, ptr as *mut u8, expected);
        }
        self.compressed = false;
        self.raw_bytes.clear();
        Ok(())
    }
}

fn encode_column<T: Copy>(values: &[T], encoding: ColumnEncoding) -> Result<Vec<u8>, KcmError> {
    match encoding {
        ColumnEncoding::Delta => {
            let type_name = std::any::type_name::<T>();
            if type_name == "i64" {
                let typed = unsafe { &*(values as *const [T] as *const [i64]) };
                Ok(encode_delta_i64(typed))
            } else if type_name == "i32" {
                let typed = unsafe { &*(values as *const [T] as *const [i32]) };
                Ok(encode_delta_i32(typed))
            } else {
                Ok(encode_identity(values))
            }
        }
        ColumnEncoding::Gorilla => {
            let type_name = std::any::type_name::<T>();
            if type_name == "f64" {
                let typed = unsafe { &*(values as *const [T] as *const [f64]) };
                Ok(encode_gorilla_f64(typed))
            } else {
                Ok(encode_identity(values))
            }
        }
        ColumnEncoding::Dictionary | ColumnEncoding::Identity => Ok(encode_identity(values)),
        ColumnEncoding::Rle => {
            let type_name = std::any::type_name::<T>();
            if type_name == "u8" || type_name == "i8" {
                let typed = unsafe { &*(values as *const [T] as *const [u8]) };
                Ok(encode_identity(typed))
            } else {
                Ok(encode_identity(values))
            }
        }
    }
}

fn decode_column<T: Copy>(
    data: &[u8],
    count: usize,
    encoding: ColumnEncoding,
) -> Result<Vec<T>, KcmError> {
    match encoding {
        ColumnEncoding::Delta => {
            let type_name = std::any::type_name::<T>();
            if type_name == "i64" {
                let decoded = decode_delta_i64(data, count)?;
                let ptr = decoded.as_ptr() as *const T;
                Ok(unsafe { std::slice::from_raw_parts(ptr, count) }.to_vec())
            } else if type_name == "i32" {
                let decoded = decode_delta_i32(data, count)?;
                let ptr = decoded.as_ptr() as *const T;
                Ok(unsafe { std::slice::from_raw_parts(ptr, count) }.to_vec())
            } else {
                decode_to_slice::<T>(data, count)
            }
        }
        ColumnEncoding::Gorilla => {
            let type_name = std::any::type_name::<T>();
            if type_name == "f64" {
                let decoded = decode_gorilla_f64(data, count)?;
                let ptr = decoded.as_ptr() as *const T;
                Ok(unsafe { std::slice::from_raw_parts(ptr, count) }.to_vec())
            } else {
                decode_to_slice::<T>(data, count)
            }
        }
        ColumnEncoding::Dictionary | ColumnEncoding::Identity => decode_to_slice::<T>(data, count),
        ColumnEncoding::Rle => decode_to_slice::<T>(data, count),
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

    pub fn clear_tombstone(&mut self, idx: usize) -> Result<(), KcmError> {
        if idx >= self.len() {
            return Err(KcmError::InvalidArgument(format!(
                "Index {} out of bounds (len {})",
                idx,
                self.len()
            )));
        }
        self.tombstones.clear(idx);
        Ok(())
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

    pub fn compact(&self) -> Result<Self, KcmError> {
        let active_count = self.active_count();
        let mut new_schema = Schema::new(active_count.max(1))?;
        for fact in self.iter_active().map(|(_, f)| f) {
            new_schema.append_fact(&fact)?;
        }
        Ok(new_schema)
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
