use kcm_distributed::coordinator::*;
use kcm_distributed::sharding::*;

#[test]
fn test_hash_sharding() {
    let sharding = HashSharding;
    let id1 = sharding.get_shard_id(42, 4);
    let id2 = sharding.get_shard_id(42, 4);
    assert_eq!(id1, id2);
    assert!(id1 < 4);
}

#[test]
fn test_hash_sharding_uniform_distribution() {
    let sharding = HashSharding;
    let mut counts = vec![0usize; 4];
    for key in 0..10000u32 {
        counts[sharding.get_shard_id(key, 4)] += 1;
    }
    for count in &counts {
        assert!(*count > 1000, "Shard distribution too skewed: {:?}", counts);
    }
}

#[test]
fn test_range_sharding() {
    let sharding = RangeSharding::new(vec![100, 200, 300]);
    assert_eq!(sharding.get_shard_id(50, 4), 0);
    assert_eq!(sharding.get_shard_id(150, 4), 1);
    assert_eq!(sharding.get_shard_id(250, 4), 2);
    assert_eq!(sharding.get_shard_id(350, 4), 3);
}

#[test]
fn test_consistent_hash_sharding() {
    let sharding = ConsistentHashSharding::new(4, 150);
    let shard1 = sharding.get_shard_for_key(42);
    let shard2 = sharding.get_shard_for_key(42);
    assert_eq!(shard1, shard2);
    assert!(shard1 < 4);
}

#[test]
fn test_consistent_hash_stability() {
    let sharding = ConsistentHashSharding::new(4, 150);
    let mut same_count = 0;
    for key in 0..1000u32 {
        let s1 = sharding.get_shard_for_key(key);
        let s2 = sharding.get_shard_for_key(key);
        if s1 == s2 {
            same_count += 1;
        }
    }
    assert_eq!(same_count, 1000);
}

#[test]
fn test_shard_map() {
    let mut map = ShardMap::new(3, Box::new(HashSharding));
    map.register_shard(ShardInfo {
        shard_id: 0,
        host: "h1".to_string(),
        port: 8000,
    });
    map.register_shard(ShardInfo {
        shard_id: 1,
        host: "h2".to_string(),
        port: 8001,
    });
    map.register_shard(ShardInfo {
        shard_id: 2,
        host: "h3".to_string(),
        port: 8002,
    });
    assert_eq!(map.num_shards(), 3);
    let all = map.get_all_shards();
    assert_eq!(all.len(), 3);
    let loc = map.locate_key(42);
    assert!(loc.is_some());
}

#[test]
fn test_transaction_coordinator() {
    let coord = TransactionCoordinator::new();
    let txn_id = coord.begin_transaction(vec![0, 1, 2]);
    assert_eq!(coord.get_status(&txn_id), Some(TransactionStatus::Pending));
    let result = coord.two_phase_commit(&txn_id);
    assert!(result.is_ok());
    assert_eq!(
        coord.get_status(&txn_id),
        Some(TransactionStatus::Committed)
    );
}

#[test]
fn test_transaction_abort() {
    let coord = TransactionCoordinator::new();
    let txn_id = coord.begin_transaction(vec![0, 1]);
    coord.abort(&txn_id).unwrap();
    assert_eq!(coord.get_status(&txn_id), Some(TransactionStatus::Aborted));
}

#[test]
fn test_transaction_not_found() {
    let coord = TransactionCoordinator::new();
    assert!(coord.two_phase_commit("nonexistent").is_err());
    assert!(coord.abort("nonexistent").is_err());
}

#[test]
fn test_transaction_auto_id() {
    let coord = TransactionCoordinator::new();
    let id1 = coord.begin_transaction(vec![0]);
    let id2 = coord.begin_transaction(vec![0]);
    assert_ne!(id1, id2);
}
