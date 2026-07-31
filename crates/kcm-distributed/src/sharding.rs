use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

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

pub struct ShardMap {
    shards: HashMap<usize, ShardInfo>,
    strategy: Box<dyn ShardingStrategy>,
    num_shards: usize,
}

impl ShardMap {
    pub fn new(num_shards: usize, strategy: Box<dyn ShardingStrategy>) -> Self {
        ShardMap {
            shards: HashMap::new(),
            strategy,
            num_shards,
        }
    }

    pub fn register_shard(&mut self, info: ShardInfo) {
        self.shards.insert(info.shard_id, info);
    }

    pub fn locate_key(&self, key: u32) -> Option<&ShardInfo> {
        let shard_id = self.strategy.get_shard_id(key, self.num_shards);
        self.shards.get(&shard_id)
    }

    pub fn get_all_shards(&self) -> Vec<&ShardInfo> {
        self.strategy
            .get_all_shards(self.num_shards)
            .iter()
            .filter_map(|id| self.shards.get(id))
            .collect()
    }

    pub fn num_shards(&self) -> usize {
        self.num_shards
    }
}
