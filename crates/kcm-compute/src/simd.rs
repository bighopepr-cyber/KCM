pub trait SimdOps<T: Copy> {
    fn simd_filter_eq(&self, value: T) -> Vec<bool>;
    fn simd_filter_ge(&self, value: T) -> Vec<bool>;
    fn simd_count(&self) -> usize;
}

#[cfg(target_arch = "x86_64")]
mod x86_impl {
    #[cfg(target_arch = "x86_64")]
    use std::arch::x86_64::*;

    #[target_feature(enable = "avx2")]
    unsafe fn avx2_filter_eq_u8(data: &[u8], value: u8) -> Vec<bool> {
        let mut result = Vec::with_capacity(data.len());
        let value_vec = _mm256_set1_epi8(value as i8);
        let chunks = data.chunks_exact(32);
        let remainder = chunks.remainder();

        for chunk in chunks {
            let data_vec = _mm256_loadu_si256(chunk.as_ptr() as *const __m256i);
            let cmp = _mm256_cmpeq_epi8(data_vec, value_vec);
            let mask = _mm256_movemask_epi8(cmp) as u32;
            for i in 0..32 {
                result.push((mask & (1 << i)) != 0);
            }
        }

        for &v in remainder {
            result.push(v == value);
        }

        result
    }

    #[target_feature(enable = "avx2")]
    unsafe fn avx2_filter_ge_u8(data: &[u8], value: u8) -> Vec<bool> {
        let mut result = Vec::with_capacity(data.len());
        let value_vec = _mm256_set1_epi8(value as i8);
        let chunks = data.chunks_exact(32);
        let remainder = chunks.remainder();

        for chunk in chunks {
            let data_vec = _mm256_loadu_si256(chunk.as_ptr() as *const __m256i);
            let cmp = _mm256_max_epu8(data_vec, value_vec);
            let eq = _mm256_cmpeq_epi8(cmp, data_vec);
            let mask = _mm256_movemask_epi8(eq) as u32;
            for i in 0..32 {
                result.push((mask & (1 << i)) != 0);
            }
        }

        for &v in remainder {
            result.push(v >= value);
        }

        result
    }

    #[target_feature(enable = "avx2")]
    unsafe fn avx2_count_ones_u8(data: &[u8]) -> usize {
        let mut count = 0usize;
        let zero = _mm256_setzero_si256();
        let chunks = data.chunks_exact(32);
        let remainder = chunks.remainder();

        for chunk in chunks {
            let data_vec = _mm256_loadu_si256(chunk.as_ptr() as *const __m256i);
            let cmp = _mm256_cmpeq_epi8(data_vec, zero);
            let mask = _mm256_movemask_epi8(cmp) as u32;
            count += 32 - mask.count_ones() as usize;
        }

        for &v in remainder {
            if v != 0 {
                count += 1;
            }
        }

        count
    }

    pub fn try_avx2_filter_eq_u8(data: &[u8], value: u8) -> Option<Vec<bool>> {
        if is_x86_feature_detected!("avx2") {
            Some(unsafe { avx2_filter_eq_u8(data, value) })
        } else {
            None
        }
    }

    pub fn try_avx2_filter_ge_u8(data: &[u8], value: u8) -> Option<Vec<bool>> {
        if is_x86_feature_detected!("avx2") {
            Some(unsafe { avx2_filter_ge_u8(data, value) })
        } else {
            None
        }
    }

    pub fn try_avx2_count_ones_u8(data: &[u8]) -> Option<usize> {
        if is_x86_feature_detected!("avx2") {
            Some(unsafe { avx2_count_ones_u8(data) })
        } else {
            None
        }
    }
}

impl SimdOps<u8> for [u8] {
    fn simd_filter_eq(&self, value: u8) -> Vec<bool> {
        #[cfg(target_arch = "x86_64")]
        {
            if let Some(result) = x86_impl::try_avx2_filter_eq_u8(self, value) {
                return result;
            }
        }
        self.iter().map(|&v| v == value).collect()
    }

    fn simd_filter_ge(&self, value: u8) -> Vec<bool> {
        #[cfg(target_arch = "x86_64")]
        {
            if let Some(result) = x86_impl::try_avx2_filter_ge_u8(self, value) {
                return result;
            }
        }
        self.iter().map(|&v| v >= value).collect()
    }

    fn simd_count(&self) -> usize {
        #[cfg(target_arch = "x86_64")]
        {
            if let Some(count) = x86_impl::try_avx2_count_ones_u8(self) {
                return count;
            }
        }
        self.iter().filter(|&&v| v != 0).count()
    }
}

impl SimdOps<u32> for [u32] {
    fn simd_filter_eq(&self, value: u32) -> Vec<bool> {
        self.iter().map(|&v| v == value).collect()
    }

    fn simd_filter_ge(&self, value: u32) -> Vec<bool> {
        self.iter().map(|&v| v >= value).collect()
    }

    fn simd_count(&self) -> usize {
        self.len()
    }
}

impl SimdOps<f64> for [f64] {
    fn simd_filter_eq(&self, value: f64) -> Vec<bool> {
        self.iter()
            .map(|&v| (v - value).abs() < f64::EPSILON)
            .collect()
    }

    fn simd_filter_ge(&self, value: f64) -> Vec<bool> {
        self.iter().map(|&v| v >= value).collect()
    }

    fn simd_count(&self) -> usize {
        self.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simd_filter_eq_u8() {
        let data = vec![1u8, 2, 3, 2, 5, 2, 7];
        let result = data.simd_filter_eq(2);
        assert_eq!(result, vec![false, true, false, true, false, true, false]);
    }

    #[test]
    fn test_simd_filter_ge_u8() {
        let data = vec![10u8, 20, 30, 40, 50];
        let result = data.simd_filter_ge(30);
        assert_eq!(result, vec![false, false, true, true, true]);
    }

    #[test]
    fn test_simd_count_u8() {
        let data = vec![0u8, 1, 0, 3, 0, 5];
        assert_eq!(data.simd_count(), 3);
    }

    #[test]
    fn test_simd_large_data() {
        let data: Vec<u8> = (0..1000).map(|i| (i % 256) as u8).collect();
        let result = data.simd_filter_eq(42);
        assert_eq!(result.len(), 1000);
        let count = result.iter().filter(|&&b| b).count();
        let expected = data.iter().filter(|&&v| v == 42).count();
        assert_eq!(count, expected);
    }

    #[test]
    fn test_simd_empty() {
        let data: Vec<u8> = vec![];
        let result = data.simd_filter_eq(1);
        assert!(result.is_empty());
    }
}
