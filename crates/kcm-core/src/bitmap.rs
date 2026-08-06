#[derive(Clone, Debug, PartialEq)]
pub struct Bitmap {
    words: Vec<u64>,
    len: usize,
}

impl Bitmap {
    const WORD_SIZE: usize = 64;

    pub fn new(capacity: usize) -> Self {
        let num_words = capacity.div_ceil(Self::WORD_SIZE);
        Bitmap {
            words: vec![0u64; num_words],
            len: capacity,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Set a bit at the given index.
    ///
    /// Returns `true` if successful, `false` if `idx >= self.len`.
    pub fn set(&mut self, idx: usize) -> bool {
        if idx >= self.len {
            return false;
        }
        let word_idx = idx / Self::WORD_SIZE;
        let bit_idx = idx % Self::WORD_SIZE;
        self.words[word_idx] |= 1u64 << bit_idx;
        true
    }

    /// Clear a bit at the given index.
    ///
    /// Returns `true` if successful, `false` if `idx >= self.len`.
    pub fn clear(&mut self, idx: usize) -> bool {
        if idx >= self.len {
            return false;
        }
        let word_idx = idx / Self::WORD_SIZE;
        let bit_idx = idx % Self::WORD_SIZE;
        self.words[word_idx] &= !(1u64 << bit_idx);
        true
    }

    pub fn get(&self, idx: usize) -> bool {
        if idx >= self.len {
            return false;
        }
        let word_idx = idx / Self::WORD_SIZE;
        let bit_idx = idx % Self::WORD_SIZE;
        (self.words[word_idx] & (1u64 << bit_idx)) != 0
    }

    pub fn set_all(&mut self) {
        self.words.fill(u64::MAX);
        // Mask off excess bits in the last word to maintain the invariant
        // that only bits [0, len) are meaningful.
        let bits_in_last = self.len % Self::WORD_SIZE;
        if bits_in_last > 0
            && let Some(last) = self.words.last_mut()
        {
            let mask = (1u64 << bits_in_last) - 1;
            *last &= mask;
        }
    }

    pub fn clear_all(&mut self) {
        self.words.fill(0);
    }

    pub fn count_ones(&self) -> usize {
        self.words.iter().map(|w| w.count_ones() as usize).sum()
    }

    /// Compute AND of self and other, storing result in self.
    ///
    /// Returns `true` if successful, `false` if bitmaps have different lengths.
    pub fn and_inplace(&mut self, other: &Bitmap) -> bool {
        if self.words.len() != other.words.len() {
            return false;
        }
        for (a, b) in self.words.iter_mut().zip(&other.words) {
            *a &= b;
        }
        true
    }

    /// Compute OR of self and other, storing result in self.
    ///
    /// Returns `true` if successful, `false` if bitmaps have different lengths.
    pub fn or_inplace(&mut self, other: &Bitmap) -> bool {
        if self.words.len() != other.words.len() {
            return false;
        }
        for (a, b) in self.words.iter_mut().zip(&other.words) {
            *a |= b;
        }
        true
    }

    pub fn not_inplace(&mut self) {
        if self.is_empty() {
            return;
        }
        for word in &mut self.words {
            *word = !*word;
        }
        // Mask off excess bits in the last word.
        let bits_in_last = self.len % Self::WORD_SIZE;
        if bits_in_last > 0
            && let Some(last) = self.words.last_mut()
        {
            let mask = (1u64 << bits_in_last) - 1;
            *last &= mask;
        }
    }

    /// Iterate over set bits efficiently using bit manipulation.
    /// O(popcount) instead of O(capacity).
    pub fn iter_set_bits(&self) -> impl Iterator<Item = usize> + '_ {
        let len = self.len;
        self.words
            .iter()
            .enumerate()
            .flat_map(move |(word_idx, &word)| {
                let base = word_idx * Self::WORD_SIZE;
                SetBitsIter { word, base, len }
            })
    }

    pub fn as_bytes(&self) -> &[u8] {
        // SAFETY: words is a Vec<u64>, valid for reinterpreting as &[u8].
        // Alignment of u64 (8 bytes) is >= alignment of u8 (1 byte).
        // Length is calculated correctly as words.len() * sizeof(u64).
        unsafe {
            std::slice::from_raw_parts(self.words.as_ptr() as *const u8, self.words.len() * 8)
        }
    }

    pub fn from_bytes(bytes: &[u8], len: usize) -> Self {
        let num_words = len.div_ceil(64);
        let mut words = vec![0u64; num_words];
        let copy_len = bytes.len().min(num_words * 8);
        // SAFETY: words is allocated with num_words * 8 bytes capacity (guaranteed by vec![0u64; num_words]).
        // copy_len <= num_words * 8 (ensured by .min()).
        // Source bytes is valid for copy_len bytes. Destination has copy_len bytes available.
        unsafe {
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), words.as_mut_ptr() as *mut u8, copy_len);
        }
        Bitmap { words, len }
    }
}

/// Efficient iterator over set bits using Brian Kernighan's algorithm.
/// O(popcount) per word instead of O(64).
struct SetBitsIter {
    word: u64,
    base: usize,
    len: usize,
}

impl Iterator for SetBitsIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while self.word != 0 {
            let bit = self.word.trailing_zeros() as usize;
            let idx = self.base + bit;
            // Clear the lowest set bit
            self.word &= self.word - 1;
            if idx < self.len {
                return Some(idx);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitmap_operations() {
        let mut bitmap = Bitmap::new(128);

        bitmap.set(0);
        bitmap.set(63);
        bitmap.set(64);
        bitmap.set(127);

        assert!(bitmap.get(0));
        assert!(bitmap.get(63));
        assert!(bitmap.get(64));
        assert!(bitmap.get(127));
        assert!(!bitmap.get(1));

        assert_eq!(bitmap.count_ones(), 4);
    }

    #[test]
    fn test_bitmap_empty_not_inplace() {
        let mut bitmap = Bitmap::new(0);
        bitmap.not_inplace(); // Should not panic
        assert!(bitmap.is_empty());
    }

    #[test]
    fn test_bitmap_set_all_masking() {
        let mut bitmap = Bitmap::new(100);
        bitmap.set_all();
        // count_ones should be exactly 100, not 128 (which would be the case
        // if excess bits in the last word were not masked)
        assert_eq!(bitmap.count_ones(), 100);
    }

    #[test]
    fn test_bitmap_not_inplace_masking() {
        let mut bitmap = Bitmap::new(100);
        bitmap.set_all();
        bitmap.not_inplace();
        // After not, count_ones should be 100 total - 100 set = 0
        // Actually: set_all sets bits 0..99, not_inplace flips them all to 0
        // But the last word had excess bits masked off before not,
        // then not flips the remaining bits, then masks again.
        // bits 0..99 were 1, after not they become 0 (within len)
        // bits 100..127 were 0 (masked), after not they become 1 but then masked back to 0
        assert_eq!(bitmap.count_ones(), 0);
    }

    #[test]
    fn test_bitmap_set_out_of_bounds() {
        let mut bitmap = Bitmap::new(10);
        assert!(!bitmap.set(10));
        assert!(!bitmap.set(100));
        assert!(!bitmap.clear(10));
    }

    #[test]
    fn test_bitmap_iter_set_bits_efficient() {
        let mut bitmap = Bitmap::new(200);
        bitmap.set(5);
        bitmap.set(65);
        bitmap.set(130);
        bitmap.set(199);

        let bits: Vec<usize> = bitmap.iter_set_bits().collect();
        assert_eq!(bits, vec![5, 65, 130, 199]);
    }
}
