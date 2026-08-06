use crate::dict_cache::DictionaryCache;
use parking_lot::RwLock;
use std::sync::Arc;

#[derive(Clone)]
pub struct DictionaryCodec {
    inner: Arc<RwLock<DictionaryCache>>,
}

impl DictionaryCodec {
    pub fn new() -> Self {
        DictionaryCodec {
            inner: Arc::new(RwLock::new(DictionaryCache::new())),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        DictionaryCodec {
            inner: Arc::new(RwLock::new(DictionaryCache::with_capacity(capacity))),
        }
    }

    #[inline]
    pub fn encode(&self, value: &str) -> u32 {
        self.inner.write().encode(value)
    }

    #[inline]
    pub fn decode(&self, id: u32) -> Option<String> {
        self.inner.read().decode(id).map(|s| s.to_string())
    }

    #[inline]
    pub fn decode_ref(&self, id: u32) -> Option<std::sync::Arc<str>> {
        self.inner.read().decode(id)
    }

    #[inline]
    pub fn lookup(&self, value: &str) -> Option<u32> {
        self.inner.read().lookup(value)
    }

    pub fn lookup_batch(&self, values: &[&str], results: &mut [Option<u32>]) {
        let inner = self.inner.read();
        for (value, result) in values.iter().zip(results.iter_mut()) {
            *result = inner.lookup(value);
        }
    }

    pub fn lookup_batch_simd(&self, values: &[&str], results: &mut [Option<u32>]) {
        let inner = self.inner.read();
        inner.lookup_batch_simd(values, results);
    }

    pub fn lookup_batch_prefetch(&self, values: &[&str], results: &mut [Option<u32>]) {
        let inner = self.inner.read();
        inner.lookup_batch_prefetch(values, results);
    }

    pub fn lookup_batch_into(&self, values: &[&str]) -> Vec<Option<u32>> {
        let mut results = Vec::with_capacity(values.len());
        results.resize(values.len(), None);
        self.lookup_batch(values, &mut results);
        results
    }

    pub fn encode_batch(&self, values: &[&str]) -> Vec<u32> {
        let mut inner = self.inner.write();
        let mut results = Vec::with_capacity(values.len());
        for &v in values {
            results.push(inner.encode(v));
        }
        results
    }

    pub fn decode_batch(&self, ids: &[u32]) -> Vec<Option<String>> {
        let inner = self.inner.read();
        ids.iter()
            .map(|&id| inner.decode(id).map(|s| s.to_string()))
            .collect()
    }

    pub fn decode_batch_into(&self, ids: &[u32], results: &mut Vec<Option<String>>) {
        let inner = self.inner.read();
        results.clear();
        results.reserve(ids.len());
        for &id in ids {
            results.push(inner.decode(id).map(|s| s.to_string()));
        }
    }

    pub fn warm_up(&self, keys: &[&str]) {
        self.inner.write().warm_up(keys);
    }

    pub fn len(&self) -> usize {
        self.inner.read().len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.read().is_empty()
    }

    pub fn space_bytes(&self) -> usize {
        self.inner.read().space_bytes()
    }

    pub fn clear(&self) {
        self.inner.write().clear();
    }

    pub fn reserve(&self, additional: usize) {
        self.inner.write().reserve(additional);
    }
}

impl Default for DictionaryCodec {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    #[test]
    fn test_dictionary_codec_basic() {
        let codec = DictionaryCodec::new();

        let id1 = codec.encode("hello");
        let id2 = codec.encode("world");
        let id1_again = codec.encode("hello");

        assert_eq!(id1, id1_again);
        assert_ne!(id1, id2);
        assert_eq!(codec.decode(id1), Some("hello".to_string()));
        assert_eq!(codec.decode(id2), Some("world".to_string()));
    }

    #[test]
    fn test_dictionary_codec_lookup() {
        let codec = DictionaryCodec::new();

        codec.encode("alice");
        codec.encode("bob");

        assert_eq!(codec.lookup("alice"), Some(1));
        assert_eq!(codec.lookup("bob"), Some(2));
        assert_eq!(codec.lookup("charlie"), None);
    }

    #[test]
    fn test_dictionary_codec_batch() {
        let codec = DictionaryCodec::new();

        let ids = codec.encode_batch(&["a", "b", "c", "a", "b"]);
        assert_eq!(ids, vec![1, 2, 3, 1, 2]);

        let decoded = codec.decode_batch(&ids);
        assert_eq!(
            decoded,
            vec![
                Some("a".to_string()),
                Some("b".to_string()),
                Some("c".to_string()),
                Some("a".to_string()),
                Some("b".to_string())
            ]
        );
    }

    #[test]
    fn test_dictionary_codec_lookup_batch() {
        let codec = DictionaryCodec::new();
        codec.encode("x");
        codec.encode("y");
        codec.encode("z");

        let results = codec.lookup_batch_into(&["x", "y", "z", "w"]);
        assert_eq!(results, vec![Some(1), Some(2), Some(3), None]);
    }

    #[test]
    fn test_dictionary_codec_decode_batch_into() {
        let codec = DictionaryCodec::new();
        codec.encode("a");
        codec.encode("b");

        let mut results = Vec::new();
        codec.decode_batch_into(&[1, 2, 3], &mut results);
        assert_eq!(
            results,
            vec![Some("a".to_string()), Some("b".to_string()), None]
        );
    }

    #[test]
    fn test_dictionary_codec_size() {
        let codec = DictionaryCodec::new();
        assert!(codec.is_empty());

        codec.encode("test");
        assert!(!codec.is_empty());
        assert_eq!(codec.len(), 2);
    }

    #[test]
    fn test_dictionary_codec_space() {
        let codec = DictionaryCodec::new();
        codec.encode("hello");
        codec.encode("world");

        let space = codec.space_bytes();
        assert!(space > 0);
    }

    #[test]
    fn test_dictionary_codec_thread_safety() {
        let codec = DictionaryCodec::new();
        let codec2 = codec.clone();

        let mut handles = vec![];
        for i in 0..10 {
            let c = codec.clone();
            handles.push(std::thread::spawn(move || {
                c.encode(&format!("item_{}", i));
            }));
        }
        for h in handles {
            h.join().unwrap();
        }
        assert_eq!(codec.len(), 11);
        assert_eq!(codec2.len(), 11);
    }

    #[test]
    fn test_dictionary_codec_clear() {
        let codec = DictionaryCodec::new();
        codec.encode("hello");
        codec.encode("world");
        assert_eq!(codec.len(), 3);
        codec.clear();
        assert_eq!(codec.len(), 1);
        assert!(codec.is_empty());
    }

    #[test]
    fn test_dictionary_codec_with_capacity() {
        let codec = DictionaryCodec::with_capacity(1024);
        assert!(codec.is_empty());
        codec.encode("test");
        assert_eq!(codec.len(), 2);
    }
}
