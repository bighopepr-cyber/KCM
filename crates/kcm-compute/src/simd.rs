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
    pub unsafe fn avx2_filter_eq_u8(data: &[u8], value: u8) -> Vec<bool> {
        // SAFETY: Caller must ensure AVX2 is available.
        unsafe {
            let mut result = Vec::with_capacity(data.len());
            let value_vec = _mm256_set1_epi8(value as i8);
            for chunk in data.chunks_exact(32) {
                let data_vec = _mm256_loadu_si256(chunk.as_ptr() as *const __m256i);
                let cmp = _mm256_cmpeq_epi8(data_vec, value_vec);
                let mask = _mm256_movemask_epi8(cmp) as u32;
                for i in 0..32 {
                    result.push((mask & (1 << i)) != 0);
                }
            }
            let remainder = data.len() % 32;
            if remainder > 0 {
                for &v in &data[data.len() - remainder..] {
                    result.push(v == value);
                }
            }
            result
        }
    }

    #[target_feature(enable = "avx2")]
    pub unsafe fn avx2_filter_ge_u8(data: &[u8], value: u8) -> Vec<bool> {
        // SAFETY: Caller must ensure AVX2 is available.
        unsafe {
            let mut result = Vec::with_capacity(data.len());
            let value_vec = _mm256_set1_epi8(value as i8);
            for chunk in data.chunks_exact(32) {
                let data_vec = _mm256_loadu_si256(chunk.as_ptr() as *const __m256i);
                let cmp = _mm256_max_epu8(data_vec, value_vec);
                let eq = _mm256_cmpeq_epi8(cmp, data_vec);
                let mask = _mm256_movemask_epi8(eq) as u32;
                for i in 0..32 {
                    result.push((mask & (1 << i)) != 0);
                }
            }
            let remainder = data.len() % 32;
            if remainder > 0 {
                for &v in &data[data.len() - remainder..] {
                    result.push(v >= value);
                }
            }
            result
        }
    }

    #[target_feature(enable = "avx2")]
    pub unsafe fn avx2_count_nonzero_u8(data: &[u8]) -> usize {
        // SAFETY: Caller must ensure AVX2 is available.
        unsafe {
            let mut count = 0usize;
            let zero = _mm256_setzero_si256();
            for chunk in data.chunks_exact(32) {
                let data_vec = _mm256_loadu_si256(chunk.as_ptr() as *const __m256i);
                let cmp = _mm256_cmpeq_epi8(data_vec, zero);
                let mask = _mm256_movemask_epi8(cmp) as u32;
                count += 32 - mask.count_ones() as usize;
            }
            let remainder = data.len() % 32;
            if remainder > 0 {
                for &v in &data[data.len() - remainder..] {
                    if v != 0 {
                        count += 1;
                    }
                }
            }
            count
        }
    }

    #[target_feature(enable = "avx2")]
    pub unsafe fn avx2_filter_eq_u32(data: &[u32], value: u32) -> Vec<bool> {
        // SAFETY: Caller must ensure AVX2 is available.
        unsafe {
            let mut result = Vec::with_capacity(data.len());
            let value_vec = _mm256_set1_epi32(value as i32);
            for chunk in data.chunks_exact(8) {
                let data_vec = _mm256_loadu_si256(chunk.as_ptr() as *const __m256i);
                let cmp = _mm256_cmpeq_epi32(data_vec, value_vec);
                let mask = _mm256_movemask_ps(std::mem::transmute::<__m256i, __m256>(cmp));
                for i in 0..8 {
                    result.push((mask & (1 << i)) != 0);
                }
            }
            for &v in &data[(data.len() & !7)..] {
                result.push(v == value);
            }
            result
        }
    }

    #[target_feature(enable = "avx2")]
    pub unsafe fn avx2_filter_ge_u32(data: &[u32], value: u32) -> Vec<bool> {
        // SAFETY: Caller must ensure AVX2 is available.
        unsafe {
            let mut result = Vec::with_capacity(data.len());
            let value_vec = _mm256_set1_epi32(value as i32);
            for chunk in data.chunks_exact(8) {
                let data_vec = _mm256_loadu_si256(chunk.as_ptr() as *const __m256i);
                let max = _mm256_max_epu32(data_vec, value_vec);
                let cmp = _mm256_cmpeq_epi32(max, data_vec);
                let mask = _mm256_movemask_ps(std::mem::transmute::<__m256i, __m256>(cmp));
                for i in 0..8 {
                    result.push((mask & (1 << i)) != 0);
                }
            }
            for &v in &data[(data.len() & !7)..] {
                result.push(v >= value);
            }
            result
        }
    }
}

impl SimdOps<u8> for [u8] {
    fn simd_filter_eq(&self, value: u8) -> Vec<bool> {
        #[cfg(target_arch = "x86_64")]
        {
            if is_x86_feature_detected!("avx2") {
                // SAFETY: is_x86_feature_detected!("avx2") guard guarantees AVX2 availability.
                // The called function has #[target_feature(enable = "avx2")] and validates all inputs.
                return unsafe { x86_impl::avx2_filter_eq_u8(self, value) };
            }
        }
        self.iter().map(|&v| v == value).collect()
    }

    fn simd_filter_ge(&self, value: u8) -> Vec<bool> {
        #[cfg(target_arch = "x86_64")]
        {
            if is_x86_feature_detected!("avx2") {
                // SAFETY: is_x86_feature_detected!("avx2") guard guarantees AVX2 availability.
                // The called function has #[target_feature(enable = "avx2")] and validates all inputs.
                return unsafe { x86_impl::avx2_filter_ge_u8(self, value) };
            }
        }
        self.iter().map(|&v| v >= value).collect()
    }

    fn simd_count(&self) -> usize {
        #[cfg(target_arch = "x86_64")]
        {
            if is_x86_feature_detected!("avx2") {
                // SAFETY: is_x86_feature_detected!("avx2") guard guarantees AVX2 availability.
                // The called function has #[target_feature(enable = "avx2")] and validates all inputs.
                return unsafe { x86_impl::avx2_count_nonzero_u8(self) };
            }
        }
        self.iter().filter(|&&v| v != 0).count()
    }
}

impl SimdOps<u32> for [u32] {
    fn simd_filter_eq(&self, value: u32) -> Vec<bool> {
        #[cfg(target_arch = "x86_64")]
        {
            if is_x86_feature_detected!("avx2") {
                // SAFETY: is_x86_feature_detected!("avx2") guard guarantees AVX2 availability.
                // The called function has #[target_feature(enable = "avx2")] and validates all inputs.
                return unsafe { x86_impl::avx2_filter_eq_u32(self, value) };
            }
        }
        self.iter().map(|&v| v == value).collect()
    }

    fn simd_filter_ge(&self, value: u32) -> Vec<bool> {
        #[cfg(target_arch = "x86_64")]
        {
            if is_x86_feature_detected!("avx2") {
                // SAFETY: is_x86_feature_detected!("avx2") guard guarantees AVX2 availability.
                // The called function has #[target_feature(enable = "avx2")] and validates all inputs.
                return unsafe { x86_impl::avx2_filter_ge_u32(self, value) };
            }
        }
        self.iter().map(|&v| v >= value).collect()
    }

    fn simd_count(&self) -> usize {
        self.iter().filter(|&&v| v != 0).count()
    }
}

impl SimdOps<f64> for [f64] {
    fn simd_filter_eq(&self, value: f64) -> Vec<bool> {
        self.iter().map(|&v| v == value).collect()
    }

    fn simd_filter_ge(&self, value: f64) -> Vec<bool> {
        self.iter().map(|&v| v >= value).collect()
    }

    fn simd_count(&self) -> usize {
        self.iter().filter(|&&v| v != 0.0).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simd_filter_eq_u8() {
        let data = [1u8, 2, 3, 2, 5, 2, 7];
        assert_eq!(
            data.simd_filter_eq(2),
            [false, true, false, true, false, true, false]
        );
    }

    #[test]
    fn test_simd_filter_ge_u8() {
        let data = [10u8, 20, 30, 40, 50];
        assert_eq!(data.simd_filter_ge(30), [false, false, true, true, true]);
    }

    #[test]
    fn test_simd_count_u8() {
        let data = [0u8, 1, 0, 3, 0, 5];
        assert_eq!(data.simd_count(), 3);
    }

    #[test]
    fn test_simd_count_u32() {
        let data = [0u32, 1, 0, 3, 5, 0];
        assert_eq!(data.simd_count(), 3);
    }

    #[test]
    fn test_simd_count_f64() {
        let data = [0.0f64, 1.5, 0.0, 3.0];
        assert_eq!(data.simd_count(), 2);
    }

    #[test]
    fn test_simd_large_data() {
        let data: Vec<u8> = (0..1000).map(|i| (i % 256) as u8).collect();
        let result = data.simd_filter_eq(42);
        assert_eq!(result.len(), 1000);
    }

    #[test]
    fn test_simd_empty() {
        let data: Vec<u8> = vec![];
        assert!(data.simd_filter_eq(1).is_empty());
    }

    #[test]
    fn test_simd_filter_eq_u32() {
        let data = [10u32, 20, 30, 20, 50];
        assert_eq!(data.simd_filter_eq(20), [false, true, false, true, false]);
    }

    #[test]
    fn test_simd_filter_ge_u32() {
        let data = [10u32, 20, 30, 40, 50];
        assert_eq!(data.simd_filter_ge(30), [false, false, true, true, true]);
    }

    #[test]
    fn test_simd_filter_ge_u8_boundary() {
        let data = [0u8, 127, 128, 255, 0];
        assert_eq!(data.simd_filter_ge(128), [false, false, true, true, false]);
    }

    #[test]
    fn test_simd_filter_ge_u32_boundary() {
        let data = [0u32, u32::MAX, 100, 200];
        assert_eq!(data.simd_filter_ge(200), [false, true, false, true]);
    }
}
