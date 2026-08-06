use ahash::AHashMap;
use parking_lot::RwLock;
use std::sync::Arc;

use crate::types::KcmError;

pub type DictID = u32;

pub struct Dictionary {
    entries: Vec<String>,
    reverse_map: AHashMap<String, DictID>,
}

impl Dictionary {
    pub fn new() -> Self {
        let mut reverse_map = AHashMap::with_capacity(256);
        reverse_map.insert(String::new(), 0);
        Dictionary {
            entries: vec![String::new()],
            reverse_map,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let mut reverse_map = AHashMap::with_capacity(capacity);
        reverse_map.insert(String::new(), 0);
        Dictionary {
            entries: vec![String::new()],
            reverse_map,
        }
    }

    pub fn insert(&mut self, value: &str) -> Result<DictID, KcmError> {
        if let Some(&id) = self.reverse_map.get(value) {
            return Ok(id);
        }

        let id = self.entries.len() as DictID;
        if id == u32::MAX {
            return Err(KcmError::OutOfMemory);
        }
        let owned = value.to_string();
        self.entries.push(owned.clone());
        self.reverse_map.insert(owned, id);
        Ok(id)
    }

    #[inline]
    pub fn get(&self, id: DictID) -> Option<&str> {
        self.entries.get(id as usize).map(|s| s.as_str())
    }

    #[inline]
    pub fn lookup(&self, value: &str) -> Option<DictID> {
        self.reverse_map.get(value).copied()
    }

    pub fn lookup_batch(&self, values: &[&str], results: &mut [Option<DictID>]) {
        debug_assert_eq!(values.len(), results.len());
        let map = &self.reverse_map;
        for (value, result) in values.iter().zip(results.iter_mut()) {
            *result = map.get(*value).copied();
        }
    }

    pub fn lookup_batch_into(&self, values: &[&str]) -> Vec<Option<DictID>> {
        let mut results = Vec::with_capacity(values.len());
        results.resize(values.len(), None);
        self.lookup_batch(values, &mut results);
        results
    }

    pub fn get_batch<'a>(&'a self, ids: &[DictID], results: &mut Vec<Option<&'a str>>) {
        results.clear();
        results.reserve(ids.len());
        for &id in ids {
            results.push(self.get(id));
        }
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.len() <= 1
    }

    pub fn entries(&self) -> &[String] {
        &self.entries
    }

    pub fn capacity(&self) -> usize {
        self.entries.capacity()
    }

    pub fn reserve(&mut self, additional: usize) {
        self.entries.reserve(additional);
        self.reverse_map.reserve(additional);
    }
}

impl Default for Dictionary {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SharedDictionary(Arc<RwLock<Dictionary>>);

impl SharedDictionary {
    pub fn new() -> Self {
        SharedDictionary(Arc::new(RwLock::new(Dictionary::new())))
    }

    pub fn with_capacity(capacity: usize) -> Self {
        SharedDictionary(Arc::new(RwLock::new(Dictionary::with_capacity(capacity))))
    }

    pub fn insert(&self, value: &str) -> Result<DictID, KcmError> {
        self.0.write().insert(value)
    }

    pub fn get(&self, id: DictID) -> Option<String> {
        self.0.read().get(id).map(|s| s.to_string())
    }

    pub fn lookup(&self, value: &str) -> Option<DictID> {
        self.0.read().lookup(value)
    }

    pub fn lookup_batch(&self, values: &[&str]) -> Vec<Option<DictID>> {
        self.0.read().lookup_batch_into(values)
    }

    pub fn len(&self) -> usize {
        self.0.read().len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.read().is_empty()
    }
}

impl Default for SharedDictionary {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for SharedDictionary {
    fn clone(&self) -> Self {
        SharedDictionary(self.0.clone())
    }
}
