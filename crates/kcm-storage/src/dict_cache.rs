use crate::robin_hood::RobinHoodMap;
use std::sync::Arc;

const PREFETCH_STRIDE: usize = 8;

pub struct DictionaryCache {
    string_to_id: RobinHoodMap<String, u32>,
    id_to_string: Vec<Arc<str>>,
    hasher: ahash::RandomState,
}

impl DictionaryCache {
    pub fn new() -> Self {
        let mut cache = DictionaryCache {
            string_to_id: RobinHoodMap::new(),
            id_to_string: Vec::new(),
            hasher: ahash::RandomState::new(),
        };
        cache.id_to_string.push(Arc::from(""));
        cache
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let mut cache = DictionaryCache {
            string_to_id: RobinHoodMap::with_capacity(capacity),
            id_to_string: Vec::with_capacity(capacity),
            hasher: ahash::RandomState::new(),
        };
        cache.id_to_string.push(Arc::from(""));
        cache
    }

    #[inline]
    fn compute_hash(&self, value: &str) -> u64 {
        self.hasher.hash_one(value)
    }

    #[inline]
    pub fn encode(&mut self, value: &str) -> u32 {
        if let Some(&id) = self.string_to_id.get(value) {
            return id;
        }
        let id = self.id_to_string.len() as u32;
        let arc_str: Arc<str> = Arc::from(value);
        self.id_to_string.push(arc_str);
        self.string_to_id.insert(value.to_string(), id);
        id
    }

    #[inline]
    pub fn decode(&self, id: u32) -> Option<Arc<str>> {
        self.id_to_string.get(id as usize).cloned()
    }

    #[inline]
    pub fn lookup(&self, value: &str) -> Option<u32> {
        self.string_to_id.get(value).copied()
    }

    pub fn lookup_batch_prefetch(&self, values: &[&str], results: &mut [Option<u32>]) {
        debug_assert_eq!(values.len(), results.len());

        let len = values.len();
        let batches = len / PREFETCH_STRIDE;

        for batch in 0..batches {
            let base = batch * PREFETCH_STRIDE;

            #[cfg(target_arch = "x86_64")]
            {
                for i in base..(base + PREFETCH_STRIDE * 2).min(len) {
                    if i + PREFETCH_STRIDE < len {
                        let prefetch_key = values[i + PREFETCH_STRIDE];
                        let hash = self.compute_hash(prefetch_key);
                        let idx = (hash as usize) % self.string_to_id.capacity();
                        let ptr = &self.string_to_id as *const _ as *const i8;
                        // SAFETY: ptr points to valid memory within self.string_to_id.
                        // _mm_prefetch is a hint and does not modify memory.
                        unsafe {
                            std::arch::x86_64::_mm_prefetch(
                                ptr.add(idx * std::mem::size_of::<usize>()),
                                std::arch::x86_64::_MM_HINT_T0,
                            );
                        }
                    }
                }
            }

            for i in base..(base + PREFETCH_STRIDE).min(len) {
                results[i] = self.string_to_id.get(values[i]).copied();
            }
        }

        for i in (batches * PREFETCH_STRIDE)..len {
            results[i] = self.string_to_id.get(values[i]).copied();
        }
    }

    #[cfg(target_arch = "x86_64")]
    pub fn lookup_batch_simd(&self, values: &[&str], results: &mut [Option<u32>]) {
        debug_assert_eq!(values.len(), results.len());
        let len = values.len();

        if len >= 4 && is_x86_feature_detected!("avx2") {
            // SAFETY: AVX2 is detected via is_x86_feature_detected! before calling.
            unsafe {
                self.lookup_batch_avx2(values, results);
            }
            return;
        }

        self.lookup_batch_prefetch(values, results);
    }

    #[cfg(not(target_arch = "x86_64"))]
    pub fn lookup_batch_simd(&self, values: &[&str], results: &mut [Option<u32>]) {
        self.lookup_batch_prefetch(values, results);
    }

    /// SAFETY: Caller must ensure AVX2 is available (via is_x86_feature_detected!).
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx2")]
    unsafe fn lookup_batch_avx2(&self, values: &[&str], results: &mut [Option<u32>]) {
        let len = values.len();
        let mut i = 0;

        while i + 4 <= len {
            results[i] = self.string_to_id.get(values[i]).copied();
            results[i + 1] = self.string_to_id.get(values[i + 1]).copied();
            results[i + 2] = self.string_to_id.get(values[i + 2]).copied();
            results[i + 3] = self.string_to_id.get(values[i + 3]).copied();

            i += 4;
        }

        for j in i..len {
            results[j] = self.string_to_id.get(values[j]).copied();
        }
    }

    pub fn warm_up(&mut self, keys: &[&str]) {
        for key in keys {
            if self.string_to_id.get(*key).is_none() {
                self.encode(key);
            }
        }
    }

    pub fn len(&self) -> usize {
        self.id_to_string.len()
    }

    pub fn is_empty(&self) -> bool {
        self.id_to_string.len() <= 1
    }

    pub fn space_bytes(&self) -> usize {
        let string_bytes: usize = self.id_to_string.iter().map(|s| s.len()).sum();
        let map_overhead = self.string_to_id.capacity()
            * (std::mem::size_of::<String>() + std::mem::size_of::<u32>());
        string_bytes + map_overhead
    }

    pub fn clear(&mut self) {
        self.string_to_id.clear();
        self.id_to_string.clear();
        self.id_to_string.push(Arc::from(""));
    }

    pub fn reserve(&mut self, additional: usize) {
        self.string_to_id.reserve(additional);
        self.id_to_string.reserve(additional);
    }
}

impl Default for DictionaryCache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    #[test]
    fn test_dictionary_cache_basic() {
        let mut cache = DictionaryCache::new();
        let id1 = cache.encode("hello");
        let id2 = cache.encode("world");
        let id1_again = cache.encode("hello");

        assert_eq!(id1, id1_again);
        assert_ne!(id1, id2);
        assert_eq!(
            cache.decode(id1).map(|s| s.to_string()),
            Some("hello".to_string())
        );
        assert_eq!(
            cache.decode(id2).map(|s| s.to_string()),
            Some("world".to_string())
        );
    }

    #[test]
    fn test_dictionary_cache_lookup() {
        let mut cache = DictionaryCache::new();
        cache.encode("alice");
        cache.encode("bob");

        assert_eq!(cache.lookup("alice"), Some(1));
        assert_eq!(cache.lookup("bob"), Some(2));
        assert_eq!(cache.lookup("charlie"), None);
    }

    #[test]
    fn test_dictionary_cache_batch_lookup() {
        let mut cache = DictionaryCache::new();
        cache.encode("x");
        cache.encode("y");
        cache.encode("z");

        let values = ["x", "y", "z", "w"];
        let mut results = vec![None; 4];
        cache.lookup_batch_prefetch(&values, &mut results);
        assert_eq!(results, vec![Some(1), Some(2), Some(3), None]);
    }

    #[test]
    fn test_dictionary_cache_simd_lookup() {
        let mut cache = DictionaryCache::new();
        for i in 0..100 {
            cache.encode(&format!("item_{}", i));
        }

        let values: Vec<String> = (0..100).map(|i| format!("item_{}", i)).collect();
        let refs: Vec<&str> = values.iter().map(|s| s.as_str()).collect();
        let mut results = vec![None; 100];
        cache.lookup_batch_simd(&refs, &mut results);

        for (i, result) in results.iter().enumerate() {
            assert_eq!(*result, Some((i + 1) as u32));
        }
    }

    #[test]
    fn test_dictionary_cache_warm_up() {
        let mut cache = DictionaryCache::new();
        cache.warm_up(&["alpha", "beta", "gamma"]);
        assert_eq!(cache.len(), 4);
        assert_eq!(cache.lookup("alpha"), Some(1));
        assert_eq!(cache.lookup("beta"), Some(2));
        assert_eq!(cache.lookup("gamma"), Some(3));

        cache.warm_up(&["alpha", "delta"]);
        assert_eq!(cache.len(), 5);
        assert_eq!(cache.lookup("delta"), Some(4));
    }

    #[test]
    fn test_dictionary_cache_size() {
        let mut cache = DictionaryCache::new();
        assert!(cache.is_empty());
        cache.encode("test");
        assert!(!cache.is_empty());
        assert_eq!(cache.len(), 2);
    }

    #[test]
    fn test_dictionary_cache_space() {
        let mut cache = DictionaryCache::new();
        cache.encode("hello");
        cache.encode("world");
        assert!(cache.space_bytes() > 0);
    }
}
