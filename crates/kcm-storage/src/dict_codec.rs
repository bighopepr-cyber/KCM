use std::collections::HashMap;

pub struct DictionaryCodec {
    string_to_id: HashMap<String, u32>,
    id_to_string: Vec<String>,
}

impl DictionaryCodec {
    pub fn new() -> Self {
        let mut codec = DictionaryCodec {
            string_to_id: HashMap::new(),
            id_to_string: Vec::new(),
        };
        codec.id_to_string.push(String::new());
        codec
    }

    pub fn encode(&mut self, value: &str) -> u32 {
        if let Some(&id) = self.string_to_id.get(value) {
            return id;
        }
        let id = self.id_to_string.len() as u32;
        self.id_to_string.push(value.to_string());
        self.string_to_id.insert(value.to_string(), id);
        id
    }

    pub fn decode(&self, id: u32) -> Option<&str> {
        self.id_to_string.get(id as usize).map(|s| s.as_str())
    }

    pub fn lookup(&self, value: &str) -> Option<u32> {
        self.string_to_id.get(value).copied()
    }

    pub fn len(&self) -> usize {
        self.id_to_string.len()
    }

    pub fn is_empty(&self) -> bool {
        self.id_to_string.len() <= 1
    }

    pub fn encode_batch(&mut self, values: &[&str]) -> Vec<u32> {
        values.iter().map(|v| self.encode(v)).collect()
    }

    pub fn decode_batch(&self, ids: &[u32]) -> Vec<Option<&str>> {
        ids.iter().map(|&id| self.decode(id)).collect()
    }

    pub fn space_bytes(&self) -> usize {
        self.id_to_string.iter().map(|s| s.len()).sum::<usize>()
            + self.string_to_id.capacity()
                * (std::mem::size_of::<String>() + std::mem::size_of::<u32>())
    }
}

impl Default for DictionaryCodec {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dictionary_codec_basic() {
        let mut codec = DictionaryCodec::new();

        let id1 = codec.encode("hello");
        let id2 = codec.encode("world");
        let id1_again = codec.encode("hello");

        assert_eq!(id1, id1_again);
        assert_ne!(id1, id2);
        assert_eq!(codec.decode(id1), Some("hello"));
        assert_eq!(codec.decode(id2), Some("world"));
    }

    #[test]
    fn test_dictionary_codec_lookup() {
        let mut codec = DictionaryCodec::new();

        codec.encode("alice");
        codec.encode("bob");

        assert_eq!(codec.lookup("alice"), Some(1));
        assert_eq!(codec.lookup("bob"), Some(2));
        assert_eq!(codec.lookup("charlie"), None);
    }

    #[test]
    fn test_dictionary_codec_batch() {
        let mut codec = DictionaryCodec::new();

        let ids = codec.encode_batch(&["a", "b", "c", "a", "b"]);
        assert_eq!(ids, vec![1, 2, 3, 1, 2]);

        let decoded = codec.decode_batch(&ids);
        assert_eq!(
            decoded,
            vec![Some("a"), Some("b"), Some("c"), Some("a"), Some("b")]
        );
    }

    #[test]
    fn test_dictionary_codec_size() {
        let mut codec = DictionaryCodec::new();
        assert!(codec.is_empty());

        codec.encode("test");
        assert!(!codec.is_empty());
        assert_eq!(codec.len(), 2);
    }

    #[test]
    fn test_dictionary_codec_space() {
        let mut codec = DictionaryCodec::new();
        codec.encode("hello");
        codec.encode("world");

        let space = codec.space_bytes();
        assert!(space > 0);
    }
}
