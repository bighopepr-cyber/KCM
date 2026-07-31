#[derive(Clone)]
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

    pub fn set(&mut self, idx: usize) {
        assert!(idx < self.len);
        let word_idx = idx / Self::WORD_SIZE;
        let bit_idx = idx % Self::WORD_SIZE;
        self.words[word_idx] |= 1u64 << bit_idx;
    }

    pub fn clear(&mut self, idx: usize) {
        assert!(idx < self.len);
        let word_idx = idx / Self::WORD_SIZE;
        let bit_idx = idx % Self::WORD_SIZE;
        self.words[word_idx] &= !(1u64 << bit_idx);
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
    }

    pub fn clear_all(&mut self) {
        self.words.fill(0);
    }

    pub fn count_ones(&self) -> usize {
        self.words.iter().map(|w| w.count_ones() as usize).sum()
    }

    pub fn and_inplace(&mut self, other: &Bitmap) {
        assert_eq!(self.words.len(), other.words.len());
        for (a, b) in self.words.iter_mut().zip(&other.words) {
            *a &= b;
        }
    }

    pub fn or_inplace(&mut self, other: &Bitmap) {
        assert_eq!(self.words.len(), other.words.len());
        for (a, b) in self.words.iter_mut().zip(&other.words) {
            *a |= b;
        }
    }

    pub fn not_inplace(&mut self) {
        for word in &mut self.words {
            *word = !*word;
        }
        let last_word_idx = self.len.div_ceil(Self::WORD_SIZE) - 1;
        let bits_in_last = self.len % Self::WORD_SIZE;
        if bits_in_last > 0 {
            let mask = (1u64 << bits_in_last) - 1;
            self.words[last_word_idx] &= mask;
        }
    }

    pub fn iter_set_bits(&self) -> impl Iterator<Item = usize> + '_ {
        let len = self.len;
        self.words
            .iter()
            .enumerate()
            .flat_map(move |(word_idx, &word)| {
                (0..Self::WORD_SIZE).filter_map(move |bit_idx| {
                    if (word & (1u64 << bit_idx)) != 0 {
                        let idx = word_idx * Self::WORD_SIZE + bit_idx;
                        if idx < len {
                            return Some(idx);
                        }
                    }
                    None
                })
            })
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(self.words.as_ptr() as *const u8, self.words.len() * 8)
        }
    }

    pub fn from_bytes(bytes: &[u8], len: usize) -> Self {
        let num_words = (len + 63) / 64;
        let mut words = vec![0u64; num_words];
        let copy_len = bytes.len().min(num_words * 8);
        unsafe {
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), words.as_mut_ptr() as *mut u8, copy_len);
        }
        Bitmap { words, len }
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
}
