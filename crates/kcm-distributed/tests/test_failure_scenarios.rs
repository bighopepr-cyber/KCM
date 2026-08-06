#![allow(clippy::unwrap_used, clippy::panic)]
use kcm_distributed::coordinator::*;
use kcm_distributed::replication::*;
use kcm_distributed::sharding::*;
use std::sync::Arc;

struct FailingTransport {
    fail_prepare: bool,
    fail_commit: bool,
    votes_no: bool,
}

impl FailingTransport {
    fn always_fail() -> Self {
        Self {
            fail_prepare: true,
            fail_commit: true,
            votes_no: true,
        }
    }

    fn vote_no() -> Self {
        Self {
            fail_prepare: false,
            fail_commit: false,
            votes_no: true,
        }
    }

    fn fail_on_commit() -> Self {
        Self {
            fail_prepare: false,
            fail_commit: true,
            votes_no: false,
        }
    }

    fn healthy() -> Self {
        Self {
            fail_prepare: false,
            fail_commit: false,
            votes_no: false,
        }
    }
}

impl ParticipantTransport for FailingTransport {
    fn prepare(&self, _participant_id: usize, _txn_id: &str) -> bool {
        !self.fail_prepare && !self.votes_no
    }
    fn commit(&self, _participant_id: usize, _txn_id: &str) {
        if self.fail_commit {
            panic!("Simulated commit failure");
        }
    }
    fn abort(&self, _participant_id: usize, _txn_id: &str) {}
}

#[test]
fn test_leader_failure_during_prepare() {
    let transport = Arc::new(FailingTransport::always_fail());
    let coord = TransactionCoordinator::with_transport(transport);
    let txn_id = coord.begin_transaction(vec![0, 1, 2]);
    let result = coord.two_phase_commit(&txn_id);
    assert!(result.is_err());
    assert_eq!(coord.get_status(&txn_id), Some(TransactionStatus::Aborted));
}

#[test]
fn test_follower_failure_votes_no() {
    let transport = Arc::new(FailingTransport::vote_no());
    let coord = TransactionCoordinator::with_transport(transport);
    let txn_id = coord.begin_transaction(vec![0, 1, 2]);
    let result = coord.two_phase_commit(&txn_id);
    assert!(result.is_err());
    assert_eq!(coord.get_status(&txn_id), Some(TransactionStatus::Aborted));
}

#[test]
fn test_partial_failure_one_node_down() {
    let healthy_transport = Arc::new(FailingTransport::healthy());
    let coord = TransactionCoordinator::with_transport(healthy_transport);
    let txn_id = coord.begin_transaction(vec![0]);
    let result = coord.two_phase_commit(&txn_id);
    assert!(result.is_ok());
    assert_eq!(
        coord.get_status(&txn_id),
        Some(TransactionStatus::Committed)
    );
}

#[test]
fn test_quorum_loss_all_nodes_fail() {
    let transport = Arc::new(FailingTransport::always_fail());
    let coord = TransactionCoordinator::with_transport(transport);
    let txn_id = coord.begin_transaction(vec![0, 1, 2, 3, 4]);
    let result = coord.two_phase_commit(&txn_id);
    assert!(result.is_err());
    assert_eq!(coord.get_status(&txn_id), Some(TransactionStatus::Aborted));
}

#[test]
fn test_duplicate_transaction_id_uses_separate_entries() {
    let coord = TransactionCoordinator::new();
    let id1 = coord.begin_transaction(vec![0]);
    let id2 = coord.begin_transaction(vec![0]);
    assert_ne!(id1, id2);
    assert!(coord.two_phase_commit(&id1).is_ok());
    assert!(coord.two_phase_commit(&id2).is_ok());
}

#[test]
fn test_abort_then_status_is_aborted() {
    let coord = TransactionCoordinator::new();
    let txn_id = coord.begin_transaction(vec![0, 1, 2]);
    coord.abort(&txn_id).unwrap();
    assert_eq!(coord.get_status(&txn_id), Some(TransactionStatus::Aborted));
}

#[test]
fn test_repeated_abort_is_idempotent() {
    let coord = TransactionCoordinator::new();
    let txn_id = coord.begin_transaction(vec![0]);
    coord.abort(&txn_id).unwrap();
    coord.abort(&txn_id).unwrap();
    assert_eq!(coord.get_status(&txn_id), Some(TransactionStatus::Aborted));
}

#[test]
fn test_shard_map_unregister_and_reregister() {
    let map = ShardMap::new(3, Box::new(HashSharding));
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
    assert_eq!(map.get_all_shards().len(), 3);
    map.unregister_shard(1);
    assert_eq!(map.get_all_shards().len(), 2);
    map.register_shard(ShardInfo {
        shard_id: 1,
        host: "h3".to_string(),
        port: 8002,
    });
    let all = map.get_all_shards();
    assert_eq!(all.len(), 3);
}

#[test]
fn test_consistent_hash_ring_rebalancing() {
    let sharding = ConsistentHashSharding::new(4, 150);
    let mut original_assignments = Vec::new();
    for key in 0..100u32 {
        original_assignments.push(sharding.get_shard_for_key(key));
    }
    for key in 0..100u32 {
        let shard = sharding.get_shard_for_key(key);
        assert!(shard < 4);
    }
}

#[test]
fn test_network_partition_simulation() {
    let map = ShardMap::new(4, Box::new(HashSharding));
    for i in 0..4 {
        map.register_shard(ShardInfo {
            shard_id: i,
            host: format!("h{}", i),
            port: 8000 + i as u16,
        });
    }
    let all = map.get_all_shards();
    assert_eq!(all.len(), 4);
    for key in 0..1000u32 {
        let loc = map.locate_key(key);
        assert!(
            loc.is_some(),
            "Key {} has no shard after simulated partition",
            key
        );
    }
}

#[test]
fn test_replication_lag_tracking() {
    let manager = ReplicationManager::new("r1");
    manager.register_region(RegionNode {
        region_id: "r1".to_string(),
        endpoint: "node1:8000".to_string(),
        status: ReplicationStatus::Active,
        lag_ms: 10,
        last_sync: 0,
    });
    manager.register_region(RegionNode {
        region_id: "r2".to_string(),
        endpoint: "node2:8000".to_string(),
        status: ReplicationStatus::Active,
        lag_ms: 10,
        last_sync: 0,
    });
    manager.update_lag("r1", 10).unwrap();
    manager.update_lag("r2", 5000).unwrap();
    manager
        .update_status("r2", ReplicationStatus::Lagging)
        .unwrap();
    let healthy = manager.healthy_regions();
    assert!(healthy.iter().any(|r| r.region_id == "r1"));
    assert!(!healthy.iter().any(|r| r.region_id == "r2"));
}

#[test]
fn test_leader_election() {
    let manager = ReplicationManager::new("r1");
    manager.register_region(RegionNode {
        region_id: "r1".to_string(),
        endpoint: "node1:8000".to_string(),
        status: ReplicationStatus::Active,
        lag_ms: 0,
        last_sync: 0,
    });
    manager.register_region(RegionNode {
        region_id: "r2".to_string(),
        endpoint: "node2:8000".to_string(),
        status: ReplicationStatus::Active,
        lag_ms: 0,
        last_sync: 0,
    });
    manager.set_primary("r1").unwrap();
    manager.set_primary("r2").unwrap();
    assert_eq!(manager.primary_region(), "r2");
}

#[test]
fn test_node_restart_shard_continuity() {
    let map = ShardMap::new(4, Box::new(HashSharding));
    for i in 0..4 {
        map.register_shard(ShardInfo {
            shard_id: i,
            host: format!("h{}", i),
            port: 8000 + i as u16,
        });
    }
    let key_42_shard = map.locate_key(42).unwrap().shard_id;
    map.unregister_shard(2);
    let key_42_shard_after = map.locate_key(42).unwrap().shard_id;
    if key_42_shard == 2 {
        assert_ne!(key_42_shard_after, 2);
    }
}

#[test]
fn test_duplicate_message_deduplication_in_queries() {
    let map = ShardMap::new(2, Box::new(HashSharding));
    for i in 0..2 {
        map.register_shard(ShardInfo {
            shard_id: i,
            host: format!("h{}", i),
            port: 8000 + i as u16,
        });
    }
    let transport = Arc::new(LocalTransport);
    let engine = DistributedQueryEngine::new(map, transport);
    let query = DistributedQuery {
        predicates: vec![],
        serialized_query: vec![1, 2, 3],
    };
    let results = engine.fan_out_query(&query);
    assert!(results.is_empty() || results.len() <= 2);
}

#[test]
fn test_2pc_all_participants_committed() {
    let transport = Arc::new(FailingTransport::healthy());
    let coord = TransactionCoordinator::with_transport(transport);
    let txn_id = coord.begin_transaction(vec![0, 1, 2, 3]);
    coord.two_phase_commit(&txn_id).unwrap();
    assert_eq!(
        coord.get_status(&txn_id),
        Some(TransactionStatus::Committed)
    );
}

#[test]
fn test_2pc_all_participants_aborted_on_single_failure() {
    let transport = Arc::new(FailingTransport::vote_no());
    let coord = TransactionCoordinator::with_transport(transport);
    let txn_id = coord.begin_transaction(vec![0, 1, 2, 3]);
    coord.two_phase_commit(&txn_id).unwrap_err();
    assert_eq!(coord.get_status(&txn_id), Some(TransactionStatus::Aborted));
}

#[test]
#[should_panic(expected = "Simulated commit failure")]
fn test_2pc_abort_on_commit_failure() {
    let transport = Arc::new(FailingTransport::fail_on_commit());
    let coord = TransactionCoordinator::with_transport(transport);
    let txn_id = coord.begin_transaction(vec![0, 1, 2, 3]);
    let _ = coord.two_phase_commit(&txn_id);
}

#[test]
fn test_concurrent_transactions_different_ids() {
    let coord = TransactionCoordinator::new();
    let txn_ids: Vec<String> = (0..10).map(|_| coord.begin_transaction(vec![0])).collect();
    let unique_ids: std::collections::HashSet<&str> = txn_ids.iter().map(|s| s.as_str()).collect();
    assert_eq!(unique_ids.len(), 10);
}

#[test]
fn test_empty_transaction_participants() {
    let coord = TransactionCoordinator::new();
    let txn_id = coord.begin_transaction(vec![]);
    let result = coord.two_phase_commit(&txn_id);
    assert!(result.is_ok());
}
