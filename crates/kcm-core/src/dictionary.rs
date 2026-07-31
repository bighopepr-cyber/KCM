use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

use crate::types::KcmError;

pub type DictID = u32;

pub struct Dictionary {
    entries: Vec<String>,
    reverse_map: HashMap<String, DictID>,
}

impl Dictionary {
    pub fn new() -> Self {
        Dictionary {
            entries: vec![String::new()],
            reverse_map: {
                let mut map = HashMap::new();
                map.insert(String::new(), 0);
                map
            },
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
        self.reverse_map.insert(owned.clone(), id);
        self.entries.push(owned);
        Ok(id)
    }

    pub fn get(&self, id: DictID) -> Option<&str> {
        self.entries.get(id as usize).map(|s| s.as_str())
    }

    pub fn lookup(&self, value: &str) -> Option<DictID> {
        self.reverse_map.get(value).copied()
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

    pub fn insert(&self, value: &str) -> Result<DictID, KcmError> {
        self.0.write().insert(value)
    }

    pub fn get(&self, id: DictID) -> Option<String> {
        self.0.read().get(id).map(|s| s.to_string())
    }

    pub fn lookup(&self, value: &str) -> Option<DictID> {
        self.0.read().lookup(value)
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
