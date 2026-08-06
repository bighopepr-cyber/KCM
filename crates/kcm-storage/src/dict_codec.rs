use ahash::AHashMap;
use parking_lot::RwLock;
use std::sync::Arc;

#[derive(Clone)]
pub struct DictionaryCodec {
    inner: Arc<RwLock<DictionaryCodecInner>>,
}

struct DictionaryCodecInner {
    string_to_id: AHashMap<String, u32>,
    id_to_string: Vec<String>,
}

impl DictionaryCodec {
    pub fn new() -> Self {
        let mut inner = DictionaryCodecInner {
            string_to_id: AHashMap::with_capacity(256),
            id_to_string: Vec::with_capacity(256),
        };
        inner.id_to_string.push(String::new());
        DictionaryCodec {
            inner: Arc::new(RwLock::new(inner)),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let mut inner = DictionaryCodecInner {
            string_to_id: AHashMap::with_capacity(capacity),
            id_to_string: Vec::with_capacity(capacity),
        };
        inner.id_to_string.push(String::new());
        DictionaryCodec {
            inner: Arc::new(RwLock::new(inner)),
        }
    }

    #[inline]
    pub fn encode(&self, value: &str) -> u32 {
        {
            let inner = self.inner.read();
            if let Some(&id) = inner.string_to_id.get(value) {
                return id;
            }
        }
        let mut inner = self.inner.write();
        if let Some(&id) = inner.string_to_id.get(value) {
            return id;
        }
        let id = inner.id_to_string.len() as u32;
        let owned = value.to_string();
        inner.id_to_string.push(owned.clone());
        inner.string_to_id.insert(owned, id);
        id
    }

    #[inline]
    pub fn decode(&self, id: u32) -> Option<String> {
        let inner = self.inner.read();
        inner.id_to_string.get(id as usize).cloned()
    }

    #[inline]
    pub fn decode_ref(&self, id: u32) -> Option<std::sync::Arc<str>> {
        let inner = self.inner.read();
        inner.id_to_string.get(id as usize).map(|s| {
            let arc: std::sync::Arc<str> = std::sync::Arc::from(s.as_str());
            arc
        })
    }

    #[inline]
    pub fn lookup(&self, value: &str) -> Option<u32> {
        let inner = self.inner.read();
        inner.string_to_id.get(value).copied()
    }

    pub fn lookup_batch(&self, values: &[&str], results: &mut [Option<u32>]) {
        debug_assert_eq!(values.len(), results.len());
        let inner = self.inner.read();
        for (value, result) in values.iter().zip(results.iter_mut()) {
            *result = inner.string_to_id.get(*value).copied();
        }
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
            if let Some(&id) = inner.string_to_id.get(v) {
                results.push(id);
                continue;
            }
            let id = inner.id_to_string.len() as u32;
            let owned = v.to_string();
            inner.id_to_string.push(owned.clone());
            inner.string_to_id.insert(owned, id);
            results.push(id);
        }
        results
    }

    pub fn decode_batch(&self, ids: &[u32]) -> Vec<Option<String>> {
        let inner = self.inner.read();
        ids.iter()
            .map(|&id| inner.id_to_string.get(id as usize).cloned())
            .collect()
    }

    pub fn decode_batch_into(&self, ids: &[u32], results: &mut Vec<Option<String>>) {
        let inner = self.inner.read();
        results.clear();
        results.reserve(ids.len());
        for &id in ids {
            results.push(inner.id_to_string.get(id as usize).cloned());
        }
    }

    pub fn len(&self) -> usize {
        let inner = self.inner.read();
        inner.id_to_string.len()
    }

    pub fn is_empty(&self) -> bool {
        let inner = self.inner.read();
        inner.id_to_string.len() <= 1
    }

    pub fn space_bytes(&self) -> usize {
        let inner = self.inner.read();
        let string_bytes: usize = inner.id_to_string.iter().map(|s| s.len()).sum();
        let map_overhead = inner.string_to_id.capacity()
            * (std::mem::size_of::<String>() + std::mem::size_of::<u32>());
        string_bytes + map_overhead
    }

    pub fn clear(&self) {
        let mut inner = self.inner.write();
        inner.string_to_id.clear();
        inner.id_to_string.clear();
        inner.id_to_string.push(String::new());
    }

    pub fn reserve(&self, additional: usize) {
        let mut inner = self.inner.write();
        inner.id_to_string.reserve(additional);
        inner.string_to_id.reserve(additional);
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
