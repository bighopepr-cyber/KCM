use kcm_core::types::*;
use kcm_distributed::coordinator::*;
use kcm_distributed::sharding::*;

#[test]
fn test_hash_sharding_consistency() {
    let sharding = HashSharding;
    let shard1 = sharding.get_shard_id(42, 4);
    let shard2 = sharding.get_shard_id(42, 4);
    assert_eq!(shard1, shard2);
}

#[test]
fn test_hash_sharding_distribution() {
    let sharding = HashSharding;
    let mut counts = vec![0usize; 4];
    for key in 0..10_000u32 {
        counts[sharding.get_shard_id(key, 4)] += 1;
    }
    for count in &counts {
        assert!(*count > 1000, "Distribution too skewed: {:?}", counts);
    }
}

#[test]
fn test_range_sharding_boundaries() {
    let sharding = RangeSharding::new(vec![100, 200, 300]);
    assert_eq!(sharding.get_shard_id(50, 4), 0);
    assert_eq!(sharding.get_shard_id(150, 4), 1);
    assert_eq!(sharding.get_shard_id(250, 4), 2);
    assert_eq!(sharding.get_shard_id(350, 4), 3);
}

#[test]
fn test_consistent_hash_stability() {
    let sharding = ConsistentHashSharding::new(4, 150);
    for key in 0..1000u32 {
        assert_eq!(
            sharding.get_shard_for_key(key),
            sharding.get_shard_for_key(key)
        );
    }
}

#[test]
fn test_shard_map_routing() {
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
    assert_eq!(map.get_all_shards().len(), 3);
    assert!(map.locate_key(42).is_some());
}

#[test]
fn test_2pc_commit() {
    let coord = TransactionCoordinator::new();
    let txn_id = coord.begin_transaction(vec![0, 1, 2]);
    assert_eq!(coord.get_status(&txn_id), Some(TransactionStatus::Pending));
    coord.two_phase_commit(&txn_id).unwrap();
    assert_eq!(
        coord.get_status(&txn_id),
        Some(TransactionStatus::Committed)
    );
}

#[test]
fn test_2pc_abort() {
    let coord = TransactionCoordinator::new();
    let txn_id = coord.begin_transaction(vec![0, 1]);
    coord.abort(&txn_id).unwrap();
    assert_eq!(coord.get_status(&txn_id), Some(TransactionStatus::Aborted));
}

#[test]
fn test_2pc_not_found() {
    let coord = TransactionCoordinator::new();
    assert!(coord.two_phase_commit("nonexistent").is_err());
    assert!(coord.abort("nonexistent").is_err());
}

#[test]
fn test_concurrent_inserts_across_shards() {
    let mut handles = Vec::new();
    for t in 0..4 {
        let kb_clone = kcm_runtime::database::KnowledgeDatabase::new().unwrap();
        handles.push(std::thread::spawn(move || {
            for i in 0..100 {
                let fact =
                    Fact::new(SubjectID(t * 100 + i), PredicateID(0), ObjectID(i), 0.9).unwrap();
                kb_clone.insert(&fact).unwrap();
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
}
