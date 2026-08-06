use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

use parking_lot::RwLock;

pub trait ShardingStrategy: Send + Sync {
    fn get_shard_id(&self, key: u32, num_shards: usize) -> usize;
    fn get_all_shards(&self, num_shards: usize) -> Vec<usize>;
}

pub struct HashSharding;

impl ShardingStrategy for HashSharding {
    fn get_shard_id(&self, key: u32, num_shards: usize) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % num_shards
    }
    fn get_all_shards(&self, num_shards: usize) -> Vec<usize> {
        (0..num_shards).collect()
    }
}

pub struct RangeSharding {
    boundaries: Vec<u32>,
}

impl RangeSharding {
    pub fn new(boundaries: Vec<u32>) -> Self {
        let mut sorted = boundaries;
        sorted.sort_unstable();
        RangeSharding { boundaries: sorted }
    }
}

impl ShardingStrategy for RangeSharding {
    fn get_shard_id(&self, key: u32, _num_shards: usize) -> usize {
        self.boundaries.partition_point(|&b| b <= key)
    }
    fn get_all_shards(&self, num_shards: usize) -> Vec<usize> {
        (0..num_shards).collect()
    }
}

pub struct ConsistentHashSharding {
    ring: Vec<(u64, usize)>,
}

impl ConsistentHashSharding {
    pub fn new(num_shards: usize, virtual_nodes: usize) -> Self {
        let mut ring = Vec::new();
        for shard in 0..num_shards {
            for vnode in 0..virtual_nodes {
                let mut hasher = DefaultHasher::new();
                format!("shard:{}-vnode:{}", shard, vnode).hash(&mut hasher);
                ring.push((hasher.finish(), shard));
            }
        }
        ring.sort_by_key(|&(h, _)| h);
        ConsistentHashSharding { ring }
    }

    pub fn get_shard_for_key(&self, key: u32) -> usize {
        if self.ring.is_empty() {
            return 0;
        }
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let key_hash = hasher.finish();
        match self.ring.binary_search_by_key(&key_hash, |&(h, _)| h) {
            Ok(i) | Err(i) => {
                if i >= self.ring.len() {
                    self.ring[0].1
                } else {
                    self.ring[i].1
                }
            }
        }
    }
}

impl ShardingStrategy for ConsistentHashSharding {
    fn get_shard_id(&self, key: u32, num_shards: usize) -> usize {
        if self.ring.is_empty() {
            return 0;
        }
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let key_hash = hasher.finish();
        let shard = match self.ring.binary_search_by_key(&key_hash, |&(h, _)| h) {
            Ok(i) | Err(i) => {
                if i >= self.ring.len() {
                    self.ring[0].1
                } else {
                    self.ring[i].1
                }
            }
        };
        shard % num_shards
    }
    fn get_all_shards(&self, num_shards: usize) -> Vec<usize> {
        (0..num_shards).collect()
    }
}

#[derive(Clone, Debug)]
pub struct ShardInfo {
    pub shard_id: usize,
    pub host: String,
    pub port: u16,
}

#[derive(Clone)]
pub struct ShardMap {
    shards: Arc<RwLock<HashMap<usize, ShardInfo>>>,
    strategy: Arc<dyn ShardingStrategy>,
    num_shards: usize,
}

impl ShardMap {
    pub fn new(num_shards: usize, strategy: Box<dyn ShardingStrategy>) -> Self {
        ShardMap {
            shards: Arc::new(RwLock::new(HashMap::new())),
            strategy: Arc::from(strategy),
            num_shards,
        }
    }

    pub fn register_shard(&self, info: ShardInfo) {
        self.shards.write().insert(info.shard_id, info);
    }

    pub fn unregister_shard(&self, shard_id: usize) -> Option<ShardInfo> {
        self.shards.write().remove(&shard_id)
    }

    pub fn locate_key(&self, key: u32) -> Option<ShardInfo> {
        let shard_id = self.strategy.get_shard_id(key, self.num_shards);
        self.shards.read().get(&shard_id).cloned()
    }

    pub fn get_all_shards(&self) -> Vec<ShardInfo> {
        self.strategy
            .get_all_shards(self.num_shards)
            .iter()
            .filter_map(|id| self.shards.read().get(id).cloned())
            .collect()
    }

    pub fn num_shards(&self) -> usize {
        self.num_shards
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    fn make_shard_map(n: usize) -> ShardMap {
        let map = ShardMap::new(n, Box::new(HashSharding));
        for i in 0..n {
            map.register_shard(ShardInfo {
                shard_id: i,
                host: "127.0.0.1".to_string(),
                port: 8000 + i as u16,
            });
        }
        map
    }

    #[test]
    fn test_hash_sharding_deterministic() {
        let strategy = HashSharding;
        let a = strategy.get_shard_id(42, 4);
        let b = strategy.get_shard_id(42, 4);
        assert_eq!(a, b);
    }

    #[test]
    fn test_range_sharding_boundaries() {
        let strategy = RangeSharding::new(vec![10, 20, 30]);
        assert_eq!(strategy.get_shard_id(0, 4), 0);
        assert_eq!(strategy.get_shard_id(15, 4), 1);
        assert_eq!(strategy.get_shard_id(25, 4), 2);
        assert_eq!(strategy.get_shard_id(35, 4), 3);
    }

    #[test]
    fn test_consistent_hash_basic() {
        let strategy = ConsistentHashSharding::new(4, 150);
        let shard = strategy.get_shard_id(42, 4);
        assert!(shard < 4);
    }

    #[test]
    fn test_shard_map_register_and_locate() {
        let map = make_shard_map(3);
        let info = map.locate_key(42);
        assert!(info.is_some());
        let info = info.unwrap();
        assert!(info.shard_id < 3);
        assert_eq!(info.host, "127.0.0.1");
    }

    #[test]
    fn test_shard_map_get_all() {
        let map = make_shard_map(4);
        let all = map.get_all_shards();
        assert_eq!(all.len(), 4);
    }

    #[test]
    fn test_shard_map_unregister() {
        let map = make_shard_map(3);
        let removed = map.unregister_shard(1);
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().shard_id, 1);
        assert!(map.unregister_shard(1).is_none());
    }

    #[test]
    fn test_shard_map_is_clone() {
        let map = make_shard_map(2);
        let map2 = map.clone();
        assert_eq!(map.num_shards(), map2.num_shards());
    }

    #[test]
    fn test_shard_map_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<ShardMap>();
    }

    #[test]
    fn test_consistent_hash_shard_for_key() {
        let strategy = ConsistentHashSharding::new(4, 100);
        let shard = strategy.get_shard_for_key(99);
        assert!(shard < 4);
    }

    #[test]
    fn test_hash_sharding_all_shards() {
        let strategy = HashSharding;
        let shards = strategy.get_all_shards(5);
        assert_eq!(shards, vec![0, 1, 2, 3, 4]);
    }
}
