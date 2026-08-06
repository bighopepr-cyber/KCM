use std::hash::{BuildHasher, Hash, Hasher};

const LOAD_FACTOR_PERCENT: usize = 90;
const INITIAL_CAPACITY: usize = 64;

#[derive(Clone)]
struct Bucket<K, V> {
    key: K,
    value: V,
    hash: u64,
    probe_distance: u32,
    occupied: bool,
}

pub struct RobinHoodMap<K, V> {
    entries: Vec<Option<Bucket<K, V>>>,
    len: usize,
    mask: usize,
    hasher: ahash::AHasher,
}

impl<K, V> RobinHoodMap<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    pub fn new() -> Self {
        Self::with_capacity(INITIAL_CAPACITY)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let capacity = capacity.max(INITIAL_CAPACITY).next_power_of_two();
        let mut entries = Vec::with_capacity(capacity);
        entries.resize_with(capacity, || None);
        RobinHoodMap {
            entries,
            len: 0,
            mask: capacity - 1,
            hasher: ahash::RandomState::new().build_hasher(),
        }
    }

    #[inline]
    fn hash_key(&self, key: &K) -> u64 {
        let mut h = self.hasher.clone();
        key.hash(&mut h);
        h.finish()
    }

    #[inline]
    fn bucket_index(&self, hash: u64) -> usize {
        (hash as usize) & self.mask
    }

    fn should_grow(&self) -> bool {
        self.len * 100 / (self.mask + 1) >= LOAD_FACTOR_PERCENT
    }

    fn grow(&mut self) {
        let old_entries = std::mem::replace(
            &mut self.entries,
            Vec::with_capacity((self.mask + 1) * 2),
        );
        self.mask = self.mask * 2 + 1;
        self.entries.resize_with(self.mask + 1, || None);
        self.len = 0;

        for bucket_opt in old_entries {
            if let Some(bucket) = bucket_opt {
                self.insert_inner(bucket.key, bucket.value, bucket.hash);
            }
        }
    }

    fn insert_inner(&mut self, key: K, value: V, hash: u64) {
        let mut index = self.bucket_index(hash);
        let mut current_key = key;
        let mut current_value = value;
        let mut current_hash = hash;
        let mut probe_dist = 0u32;

        loop {
            match &self.entries[index] {
                None => {
                    self.entries[index] = Some(Bucket {
                        key: current_key,
                        value: current_value,
                        hash: current_hash,
                        probe_distance: probe_dist,
                        occupied: true,
                    });
                    self.len += 1;
                    return;
                }
                Some(existing) => {
                    if existing.hash == current_hash && existing.key == current_key {
                        self.entries[index].as_mut().unwrap().value = current_value;
                        return;
                    }

                    if probe_dist > existing.probe_distance {
                        let old = self.entries[index].take().unwrap();
                        self.entries[index] = Some(Bucket {
                            key: current_key,
                            value: current_value,
                            hash: current_hash,
                            probe_distance: probe_dist,
                            occupied: true,
                        });
                        current_key = old.key;
                        current_value = old.value;
                        current_hash = old.hash;
                        probe_dist = old.probe_distance;
                    }
                }
            }

            index = (index + 1) & self.mask;
            probe_dist += 1;
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.should_grow() {
            self.grow();
        }
        let hash = self.hash_key(&key);
        let index = self.bucket_index(hash);
        let mut current_dist = 0u32;
        let mut idx = index;

        loop {
            match &self.entries[idx] {
                None => {
                    self.entries[idx] = Some(Bucket {
                        key,
                        value,
                        hash,
                        probe_distance: current_dist,
                        occupied: true,
                    });
                    self.len += 1;
                    return None;
                }
                Some(bucket) => {
                    if bucket.hash == hash && bucket.key == key {
                        let old_value = bucket.value.clone();
                        self.entries[idx].as_mut().unwrap().value = value;
                        return Some(old_value);
                    }
                }
            }

            idx = (idx + 1) & self.mask;
            current_dist += 1;
        }
    }

    #[inline]
    pub fn get(&self, key: &K) -> Option<&V> {
        let hash = self.hash_key(key);
        let mut index = self.bucket_index(hash);
        let mut probe_dist = 0u32;

        loop {
            match &self.entries[index] {
                None => return None,
                Some(bucket) => {
                    if probe_dist > bucket.probe_distance {
                        return None;
                    }
                    if bucket.hash == hash && bucket.key == *key {
                        return Some(&bucket.value);
                    }
                }
            }

            index = (index + 1) & self.mask;
            probe_dist += 1;
        }
    }

    #[inline]
    pub fn get_with_hash(&self, key: &K, hash: u64) -> Option<&V> {
        let mut index = self.bucket_index(hash);
        let mut probe_dist = 0u32;

        loop {
            match &self.entries[index] {
                None => return None,
                Some(bucket) => {
                    if probe_dist > bucket.probe_distance {
                        return None;
                    }
                    if bucket.hash == hash && bucket.key == *key {
                        return Some(&bucket.value);
                    }
                }
            }

            index = (index + 1) & self.mask;
            probe_dist += 1;
        }
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn capacity(&self) -> usize {
        self.mask + 1
    }

    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.entries
            .iter()
            .filter_map(|b| b.as_ref().map(|b| &b.key))
    }

    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.entries
            .iter()
            .filter_map(|b| b.as_ref().map(|b| &b.value))
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.entries
            .iter()
            .filter_map(|b| b.as_ref().map(|b| (&b.key, &b.value)))
    }

    pub fn clear(&mut self) {
        self.entries.iter_mut().for_each(|b| *b = None);
        self.len = 0;
    }

    pub fn average_probe_distance(&self) -> f64 {
        if self.len == 0 {
            return 0.0;
        }
        let total: u64 = self
            .entries
            .iter()
            .filter_map(|b| b.as_ref())
            .map(|b| b.probe_distance as u64)
            .sum();
        total as f64 / self.len as f64
    }

    pub fn max_probe_distance(&self) -> u32 {
        self.entries
            .iter()
            .filter_map(|b| b.as_ref())
            .map(|b| b.probe_distance)
            .max()
            .unwrap_or(0)
    }
}

impl<K, V> Default for RobinHoodMap<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl<K: Send, V: Send> Send for RobinHoodMap<K, V> {}
unsafe impl<K: Sync, V: Sync> Sync for RobinHoodMap<K, V> {}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    #[test]
    fn test_robin_hood_basic() {
        let mut map = RobinHoodMap::new();
        map.insert("hello", 1);
        map.insert("world", 2);
        assert_eq!(map.get(&"hello"), Some(&1));
        assert_eq!(map.get(&"world"), Some(&2));
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn test_robin_hood_overwrite() {
        let mut map = RobinHoodMap::new();
        assert_eq!(map.insert("key", 1), None);
        assert_eq!(map.insert("key", 2), Some(1));
        assert_eq!(map.get(&"key"), Some(&2));
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn test_robin_hood_growth() {
        let mut map = RobinHoodMap::with_capacity(4);
        for i in 0..100 {
            map.insert(format!("key_{}", i), i);
        }
        assert_eq!(map.len(), 100);
        for i in 0..100 {
            assert_eq!(map.get(&format!("key_{}", i)), Some(&i));
        }
    }

    #[test]
    fn test_robin_hood_max_probe() {
        let mut map = RobinHoodMap::with_capacity(8);
        for i in 0..50 {
            map.insert(i, i * 10);
        }
        assert!(map.max_probe_distance() < 32);
    }

    #[test]
    fn test_robin_hood_keys_values() {
        let mut map = RobinHoodMap::new();
        map.insert("a", 1);
        map.insert("b", 2);

        let mut keys: Vec<_> = map.keys().cloned().collect();
        keys.sort();
        assert_eq!(keys, vec!["a", "b"]);

        let mut values: Vec<_> = map.values().cloned().collect();
        values.sort();
        assert_eq!(values, vec![1, 2]);
    }

    #[test]
    fn test_robin_hood_get_with_hash() {
        let mut map = RobinHoodMap::new();
        map.insert("test", 42);
        let hash = map.hash_key(&"test");
        assert_eq!(map.get_with_hash(&"test", hash), Some(&42));
    }
}
