use kcm_core::bitmap::Bitmap;
use kcm_core::types::*;
use std::collections::HashMap;

pub struct BitmapIndex {
    values: Vec<u8>,
    bitmaps: Vec<Bitmap>,
}

impl BitmapIndex {
    pub fn new(column: &[u8], row_count: usize) -> Result<Self, KcmError> {
        let mut value_to_bitmap: HashMap<u8, Bitmap> = HashMap::new();

        for (idx, &value) in column.iter().enumerate() {
            value_to_bitmap
                .entry(value)
                .or_insert_with(|| Bitmap::new(row_count))
                .set(idx);
        }

        let mut values: Vec<u8> = value_to_bitmap.keys().copied().collect();
        values.sort_unstable();

        let bitmaps = values
            .iter()
            .map(|v| value_to_bitmap.remove(v).unwrap())
            .collect();

        Ok(BitmapIndex { values, bitmaps })
    }

    pub fn lookup(&self, value: u8) -> Option<&Bitmap> {
        self.values
            .binary_search(&value)
            .ok()
            .and_then(|idx| self.bitmaps.get(idx))
    }

    pub fn range_query(&self, low: u8, high: u8) -> Result<Bitmap, KcmError> {
        let start_idx = self.values.binary_search(&low).unwrap_or_else(|idx| idx);
        let end_idx = self
            .values
            .binary_search(&high)
            .map(|idx| idx + 1)
            .unwrap_or_else(|idx| idx);

        let mut result = Bitmap::new(self.bitmaps[0].len());
        result.clear_all();

        for bitmap in &self.bitmaps[start_idx..end_idx] {
            result.or_inplace(bitmap);
        }

        Ok(result)
    }
}

pub struct ZoneMap {
    #[allow(dead_code)]
    block_size: usize,
    min_values: Vec<i64>,
    max_values: Vec<i64>,
    row_ranges: Vec<(usize, usize)>,
}

impl ZoneMap {
    pub fn new(column: &[i64], block_size: usize) -> Result<Self, KcmError> {
        let mut min_values = Vec::new();
        let mut max_values = Vec::new();
        let mut row_ranges = Vec::new();

        let mut i = 0;
        while i < column.len() {
            let end = (i + block_size).min(column.len());
            let block = &column[i..end];

            min_values.push(*block.iter().min().unwrap_or(&0));
            max_values.push(*block.iter().max().unwrap_or(&0));
            row_ranges.push((i, end));

            i = end;
        }

        Ok(ZoneMap {
            block_size,
            min_values,
            max_values,
            row_ranges,
        })
    }

    pub fn range_query(&self, low: i64, high: i64) -> Vec<(usize, usize)> {
        self.row_ranges
            .iter()
            .zip(self.min_values.iter().zip(self.max_values.iter()))
            .filter_map(|(range, (&min, &max))| {
                if max >= low && min <= high {
                    Some(*range)
                } else {
                    None
                }
            })
            .collect()
    }
}

pub struct BloomFilter {
    bits: Vec<bool>,
    num_hashes: usize,
}

impl BloomFilter {
    pub fn new(capacity: usize) -> Self {
        let bits_needed = (capacity * 10).max(1000);
        BloomFilter {
            bits: vec![false; bits_needed],
            num_hashes: 7,
        }
    }

    pub fn insert(&mut self, value: u32) {
        for i in 0..self.num_hashes {
            let hash = Self::hash(value, i);
            let idx = hash % self.bits.len();
            self.bits[idx] = true;
        }
    }

    pub fn contains(&self, value: u32) -> bool {
        for i in 0..self.num_hashes {
            let hash = Self::hash(value, i);
            let idx = hash % self.bits.len();
            if !self.bits[idx] {
                return false;
            }
        }
        true
    }

    fn hash(value: u32, seed: usize) -> usize {
        let combined = ((value as u64) << 32) | (seed as u64);
        let result = combined.wrapping_mul(0x9e3779b97f4a7c15);
        (result >> 32) as usize
    }
}
