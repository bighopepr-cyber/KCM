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
            .filter_map(|v| value_to_bitmap.remove(v))
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
    words: Vec<u64>,
    num_bits: usize,
    num_hashes: usize,
}

impl BloomFilter {
    pub fn new(capacity: usize) -> Self {
        let num_bits = (capacity * 10).max(1024);
        let num_words = num_bits.div_ceil(64);
        BloomFilter {
            words: vec![0u64; num_words],
            num_bits,
            num_hashes: 7,
        }
    }

    pub fn insert(&mut self, value: u32) {
        for i in 0..self.num_hashes {
            let bit = self.get_bit_index(value, i);
            self.words[bit / 64] |= 1u64 << (bit % 64);
        }
    }

    pub fn contains(&self, value: u32) -> bool {
        for i in 0..self.num_hashes {
            let bit = self.get_bit_index(value, i);
            if (self.words[bit / 64] & (1u64 << (bit % 64))) == 0 {
                return false;
            }
        }
        true
    }

    fn get_bit_index(&self, value: u32, seed: usize) -> usize {
        let combined = ((value as u64) << 32) | (seed as u64);
        let result = combined.wrapping_mul(0x9e3779b97f4a7c15);
        ((result >> 32) as usize) % self.num_bits
    }

    pub fn estimated_memory_bytes(&self) -> usize {
        self.words.len() * 8
    }
}

pub struct CompositeIndex {
    entries: HashMap<(u32, u8), Vec<usize>>,
}

impl CompositeIndex {
    pub fn new() -> Self {
        CompositeIndex {
            entries: HashMap::new(),
        }
    }

    pub fn build(subjects: &[u32], predicates: &[u8], row_count: usize) -> Self {
        let mut index = CompositeIndex::new();
        for i in 0..row_count.min(subjects.len()).min(predicates.len()) {
            let key = (subjects[i], predicates[i]);
            index.entries.entry(key).or_default().push(i);
        }
        index
    }

    pub fn lookup(&self, subject: u32, predicate: u8) -> Option<&[usize]> {
        self.entries
            .get(&(subject, predicate))
            .map(|v| v.as_slice())
    }

    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }

    pub fn total_rows(&self) -> usize {
        self.entries.values().map(|v| v.len()).sum()
    }
}

impl Default for CompositeIndex {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bloom_filter_memory_efficiency() {
        let bf = BloomFilter::new(100_000);
        let mem = bf.estimated_memory_bytes();
        assert!(
            mem < 200_000,
            "BloomFilter should use < 200KB for 100K elements, used {}",
            mem
        );
    }

    #[test]
    fn test_bloom_filter_basic() {
        let mut bf = BloomFilter::new(100);
        bf.insert(42);
        bf.insert(100);
        assert!(bf.contains(42));
        assert!(bf.contains(100));
        assert!(!bf.contains(999));
    }

    #[test]
    fn test_composite_index() {
        let subjects = vec![1u32, 2, 3, 1, 2];
        let predicates = vec![0u8, 0, 1, 0, 1];
        let idx = CompositeIndex::build(&subjects, &predicates, 5);
        assert_eq!(idx.entry_count(), 4);
        let results = idx.lookup(1, 0).unwrap();
        assert_eq!(results, &[0, 3]);
        assert!(idx.lookup(1, 1).is_none());
    }

    #[test]
    fn test_zone_map() {
        let col: Vec<i64> = vec![10, 20, 30, 5, 15, 25];
        let zm = ZoneMap::new(&col, 3).unwrap();
        let ranges = zm.range_query(15, 25);
        assert!(!ranges.is_empty());
    }
}
