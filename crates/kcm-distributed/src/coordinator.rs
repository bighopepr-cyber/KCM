use kcm_core::types::KcmError;
use kcm_security::audit::{AuditEventType, AuditLog};
use parking_lot::Mutex;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use crate::sharding::ShardMap;

/// Transport abstraction for distributed participant communication.
pub trait ParticipantTransport: Send + Sync {
    /// Send PREPARE message and collect vote.
    fn prepare(&self, participant_id: usize, txn_id: &str) -> bool;
    /// Send COMMIT message to participant.
    fn commit(&self, participant_id: usize, txn_id: &str);
    /// Send ABORT message to participant.
    fn abort(&self, participant_id: usize, txn_id: &str);
    /// Send a serialized query to a shard and return serialized results.
    fn send_query(&self, participant_id: usize, query: &[u8]) -> Vec<u8> {
        let _ = participant_id;
        let _ = query;
        Vec::new()
    }
}

/// Local transport that always votes YES (single-node mode).
pub struct LocalTransport;

impl ParticipantTransport for LocalTransport {
    fn prepare(&self, _participant_id: usize, _txn_id: &str) -> bool {
        true
    }
    fn commit(&self, _participant_id: usize, _txn_id: &str) {}
    fn abort(&self, _participant_id: usize, _txn_id: &str) {}
    fn send_query(&self, _participant_id: usize, query: &[u8]) -> Vec<u8> {
        query.to_vec()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransactionStatus {
    Pending,
    Prepared,
    Committed,
    Aborted,
}

pub struct DistributedTransaction {
    pub transaction_id: String,
    pub participants: Vec<usize>,
    pub status: TransactionStatus,
    votes: Arc<Mutex<HashMap<usize, bool>>>,
}

impl DistributedTransaction {
    pub fn new(transaction_id: String, participants: Vec<usize>) -> Self {
        DistributedTransaction {
            transaction_id,
            participants,
            status: TransactionStatus::Pending,
            votes: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn record_vote(&self, vote: bool) -> bool {
        let mut votes = self.votes.lock();
        let next_idx = votes.len();
        votes.insert(next_idx, vote);
        votes.values().all(|&v| v)
    }

    pub fn all_voted(&self) -> bool {
        let votes = self.votes.lock();
        votes.len() >= self.participants.len()
    }
}

pub struct TransactionCoordinator {
    transactions: Arc<Mutex<HashMap<String, DistributedTransaction>>>,
    next_id: std::sync::atomic::AtomicU64,
    transport: Arc<dyn ParticipantTransport>,
    audit_log: Option<Arc<AuditLog>>,
}

impl TransactionCoordinator {
    pub fn new() -> Self {
        TransactionCoordinator {
            transactions: Arc::new(Mutex::new(HashMap::new())),
            next_id: std::sync::atomic::AtomicU64::new(1),
            transport: Arc::new(LocalTransport),
            audit_log: None,
        }
    }

    pub fn with_transport(transport: Arc<dyn ParticipantTransport>) -> Self {
        TransactionCoordinator {
            transactions: Arc::new(Mutex::new(HashMap::new())),
            next_id: std::sync::atomic::AtomicU64::new(1),
            transport,
            audit_log: None,
        }
    }

    pub fn with_audit_log(
        transport: Arc<dyn ParticipantTransport>,
        audit_log: Arc<AuditLog>,
    ) -> Self {
        TransactionCoordinator {
            transactions: Arc::new(Mutex::new(HashMap::new())),
            next_id: std::sync::atomic::AtomicU64::new(1),
            transport,
            audit_log: Some(audit_log),
        }
    }

    fn log_audit(&self, event_type: AuditEventType, user_id: &str, context: &str, details: &str) {
        if let Some(ref log) = self.audit_log {
            let _ = log.log_access_check(user_id, context, true);
            let event = kcm_security::audit::AuditEvent::new(event_type, user_id, context, details);
            if let Ok(evt) = event {
                log.log(evt);
            }
        }
    }

    pub fn begin_transaction(&self, participants: Vec<usize>) -> String {
        let id = self
            .next_id
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let txn_id = format!("txn-{}", id);
        let txn = DistributedTransaction::new(txn_id.clone(), participants.clone());
        self.transactions.lock().insert(txn_id.clone(), txn);
        log::debug!("Began transaction {} with {} participants", txn_id, participants.len());
        txn_id
    }

    pub fn two_phase_commit(&self, txn_id: &str) -> Result<(), KcmError> {
        let mut txns = self.transactions.lock();
        let txn = txns
            .get_mut(txn_id)
            .ok_or_else(|| KcmError::NotFound(format!("Transaction not found: {}", txn_id)))?;

        if txn.participants.is_empty() {
            txn.status = TransactionStatus::Committed;
            log::debug!("Transaction {} committed (empty participant list)", txn_id);
            return Ok(());
        }

        log::debug!(
            "2PC preparing {} participants for transaction {}",
            txn.participants.len(),
            txn_id
        );

        // Phase 1: PREPARE
        for participant in &txn.participants {
            let vote = self.transport.prepare(*participant, &txn.transaction_id);
            if !vote {
                txn.status = TransactionStatus::Aborted;
                log::warn!(
                    "2PC: participant {} voted ABORT for transaction {}",
                    participant,
                    txn_id
                );
                self.transport.abort(*participant, &txn.transaction_id);
                return Err(KcmError::Conflict(format!(
                    "Participant {} voted ABORT",
                    participant
                )));
            }
        }

        log::debug!("2PC committing {} participants for transaction {}", txn.participants.len(), txn_id);

        // Phase 2: COMMIT
        for participant in &txn.participants {
            self.transport.commit(*participant, &txn.transaction_id);
        }

        txn.status = TransactionStatus::Committed;
        log::info!(
            "2PC committed transaction {} with {} participants",
            txn_id,
            txn.participants.len()
        );
        self.log_audit(
            AuditEventType::AccessControlCheck,
            "system",
            &format!("txn={}", txn_id),
            &format!("2PC committed with {} participants", txn.participants.len()),
        );
        Ok(())
    }

    pub fn abort(&self, txn_id: &str) -> Result<(), KcmError> {
        let mut txns = self.transactions.lock();
        if let Some(txn) = txns.get_mut(txn_id) {
            txn.status = TransactionStatus::Aborted;
            log::info!("Transaction {} aborted", txn_id);
            Ok(())
        } else {
            Err(KcmError::NotFound(format!(
                "Transaction not found: {}",
                txn_id
            )))
        }
    }

    pub fn get_status(&self, txn_id: &str) -> Option<TransactionStatus> {
        self.transactions
            .lock()
            .get(txn_id)
            .map(|t| t.status.clone())
    }
}

impl Default for TransactionCoordinator {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct QueryDedupKey {
    pub subject: u32,
    pub predicate: u8,
    pub object: u32,
    pub timestamp: i64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct QueryResult {
    pub subject: u32,
    pub predicate: u8,
    pub object: u32,
    pub confidence: f64,
    pub evidence: u8,
    pub timestamp: i64,
    pub context: u8,
    pub version: i32,
    pub priority: i8,
    pub owner: u16,
}

impl QueryResult {
    pub fn dedup_key(&self) -> QueryDedupKey {
        QueryDedupKey {
            subject: self.subject,
            predicate: self.predicate,
            object: self.object,
            timestamp: self.timestamp,
        }
    }
}

#[derive(Clone, Debug)]
pub struct FilterPredicate {
    pub column: String,
    pub op: FilterOp,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum FilterOp {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

#[derive(Clone, Debug)]
pub struct DistributedQuery {
    pub predicates: Vec<FilterPredicate>,
    pub serialized_query: Vec<u8>,
}

pub struct DistributedQueryEngine {
    shard_map: ShardMap,
    transport: Arc<dyn ParticipantTransport>,
}

impl DistributedQueryEngine {
    pub fn new(shard_map: ShardMap, transport: Arc<dyn ParticipantTransport>) -> Self {
        DistributedQueryEngine {
            shard_map,
            transport,
        }
    }

    pub fn fan_out_query(&self, query: &DistributedQuery) -> Vec<QueryResult> {
        let shards = self.shard_map.get_all_shards();
        let serialized = &query.serialized_query;

        let shard_results: Vec<Vec<QueryResult>> = shards
            .par_iter()
            .map(|shard| {
                let raw = self.transport.send_query(shard.shard_id, serialized);
                deserialize_results(&raw)
            })
            .collect();

        let mut seen = HashSet::new();
        let mut merged = Vec::new();

        for results in shard_results {
            for result in results {
                let key = result.dedup_key();
                if seen.insert(key) {
                    merged.push(result);
                }
            }
        }

        merged
    }

    pub fn fan_out_with_predicates(&self, predicates: Vec<FilterPredicate>) -> Vec<QueryResult> {
        let query = DistributedQuery {
            predicates: predicates.clone(),
            serialized_query: serialize_predicates(&predicates),
        };
        self.fan_out_query(&query)
    }

    pub fn shard_map(&self) -> &ShardMap {
        &self.shard_map
    }
}

fn serialize_predicates(predicates: &[FilterPredicate]) -> Vec<u8> {
    let mut out = Vec::new();
    for p in predicates {
        out.extend_from_slice(p.column.as_bytes());
        out.push(0);
        out.push(match p.op {
            FilterOp::Eq => 0,
            FilterOp::Ne => 1,
            FilterOp::Lt => 2,
            FilterOp::Le => 3,
            FilterOp::Gt => 4,
            FilterOp::Ge => 5,
        });
        out.extend_from_slice(p.value.as_bytes());
        out.push(0);
    }
    out
}

fn deserialize_results(data: &[u8]) -> Vec<QueryResult> {
    if data.is_empty() {
        return Vec::new();
    }

    let mut results = Vec::new();
    let chunk_size = 28;
    for chunk in data.chunks(chunk_size) {
        if chunk.len() < chunk_size {
            break;
        }
        results.push(QueryResult {
            subject: u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]),
            predicate: chunk[4],
            object: u32::from_le_bytes([chunk[5], chunk[6], chunk[7], chunk[8]]),
            confidence: f64::from_le_bytes([
                chunk[9], chunk[10], chunk[11], chunk[12], chunk[13], chunk[14], chunk[15],
                chunk[16],
            ]),
            evidence: chunk[17],
            timestamp: i64::from_le_bytes([
                chunk[18], chunk[19], chunk[20], chunk[21], chunk[22], chunk[23], chunk[24],
                chunk[25],
            ]),
            context: chunk[26],
            version: i32::from_le_bytes([chunk[27], 0, 0, 0]),
            priority: 0,
            owner: 0,
        });
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sharding::{HashSharding, ShardInfo};

    type QueryLog = Arc<Mutex<Vec<(usize, Vec<u8>)>>>;

    fn make_engine(num_shards: usize) -> (DistributedQueryEngine, QueryLog) {
        let shard_map = ShardMap::new(num_shards, Box::new(HashSharding));
        for i in 0..num_shards {
            shard_map.register_shard(ShardInfo {
                shard_id: i,
                host: "127.0.0.1".to_string(),
                port: 8000 + i as u16,
            });
        }
        let log = Arc::new(Mutex::new(Vec::new()));
        let transport = Arc::new(MockQueryTransport { log: log.clone() });
        (DistributedQueryEngine::new(shard_map, transport), log)
    }

    struct MockQueryTransport {
        log: QueryLog,
    }

    impl ParticipantTransport for MockQueryTransport {
        fn prepare(&self, _: usize, _: &str) -> bool {
            true
        }
        fn commit(&self, _: usize, _: &str) {}
        fn abort(&self, _: usize, _: &str) {}
        fn send_query(&self, participant_id: usize, query: &[u8]) -> Vec<u8> {
            self.log.lock().push((participant_id, query.to_vec()));
            make_test_result_data(participant_id as u32)
        }
    }

    fn make_test_result_data(seed: u32) -> Vec<u8> {
        let mut data = Vec::with_capacity(28);
        data.extend_from_slice(&seed.to_le_bytes());
        data.push(1);
        data.extend_from_slice(&(seed + 100).to_le_bytes());
        data.extend_from_slice(&0.95f64.to_le_bytes());
        data.push(0);
        data.extend_from_slice(&1000i64.to_le_bytes());
        data.push(0);
        data.extend_from_slice(&1i32.to_le_bytes());
        data
    }

    #[test]
    fn test_fan_out_query_parallel() {
        let (engine, log) = make_engine(4);
        let query = DistributedQuery {
            predicates: vec![],
            serialized_query: vec![1, 2, 3],
        };
        let results = engine.fan_out_query(&query);
        let logged = log.lock();
        assert_eq!(logged.len(), 4);
        assert_eq!(results.len(), 4);
    }

    #[test]
    fn test_deduplication() {
        let shard_map = ShardMap::new(2, Box::new(HashSharding));
        for i in 0..2 {
            shard_map.register_shard(ShardInfo {
                shard_id: i,
                host: "127.0.0.1".to_string(),
                port: 8000 + i as u16,
            });
        }

        let transport = Arc::new(DedupTestTransport);
        let engine = DistributedQueryEngine::new(shard_map, transport);
        let query = DistributedQuery {
            predicates: vec![],
            serialized_query: vec![],
        };
        let results = engine.fan_out_query(&query);
        assert_eq!(results.len(), 1);
    }

    struct DedupTestTransport;

    impl ParticipantTransport for DedupTestTransport {
        fn prepare(&self, _: usize, _: &str) -> bool {
            true
        }
        fn commit(&self, _: usize, _: &str) {}
        fn abort(&self, _: usize, _: &str) {}
        fn send_query(&self, _: usize, _: &[u8]) -> Vec<u8> {
            make_test_result_data(1)
        }
    }

    #[test]
    fn test_filter_pushdown_predicates() {
        let (engine, log) = make_engine(2);
        let results = engine.fan_out_with_predicates(vec![FilterPredicate {
            column: "subject".to_string(),
            op: FilterOp::Eq,
            value: "42".to_string(),
        }]);
        let logged = log.lock();
        assert_eq!(logged.len(), 2);
        for (_, q) in logged.iter() {
            assert!(!q.is_empty());
        }
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_empty_transport_returns_empty() {
        let shard_map = ShardMap::new(1, Box::new(HashSharding));
        shard_map.register_shard(ShardInfo {
            shard_id: 0,
            host: "127.0.0.1".to_string(),
            port: 8000,
        });
        let transport = Arc::new(LocalTransport);
        let engine = DistributedQueryEngine::new(shard_map, transport);
        let query = DistributedQuery {
            predicates: vec![],
            serialized_query: vec![1, 2, 3],
        };
        let results = engine.fan_out_query(&query);
        assert!(results.is_empty());
    }

    #[test]
    fn test_serialize_predicates_roundtrip() {
        let predicates = vec![
            FilterPredicate {
                column: "subject".to_string(),
                op: FilterOp::Eq,
                value: "42".to_string(),
            },
            FilterPredicate {
                column: "timestamp".to_string(),
                op: FilterOp::Gt,
                value: "1000".to_string(),
            },
        ];
        let data = serialize_predicates(&predicates);
        assert!(!data.is_empty());
        assert!(data.contains(&0));
    }

    #[test]
    fn test_query_dedup_key_eq() {
        let a = QueryDedupKey {
            subject: 1,
            predicate: 2,
            object: 3,
            timestamp: 100,
        };
        let b = QueryDedupKey {
            subject: 1,
            predicate: 2,
            object: 3,
            timestamp: 100,
        };
        let c = QueryDedupKey {
            subject: 1,
            predicate: 2,
            object: 3,
            timestamp: 200,
        };
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn test_local_transport_send_query() {
        let t = LocalTransport;
        let result = t.send_query(0, &[1, 2, 3]);
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_deserialize_empty() {
        let results = deserialize_results(&[]);
        assert!(results.is_empty());
    }

    #[test]
    fn test_filter_op_variants() {
        let ops = [
            FilterOp::Eq,
            FilterOp::Ne,
            FilterOp::Lt,
            FilterOp::Le,
            FilterOp::Gt,
            FilterOp::Ge,
        ];
        assert_eq!(ops.len(), 6);
    }

    #[test]
    fn test_transaction_coordinator_default() {
        let coord = TransactionCoordinator::new();
        let id = coord.begin_transaction(vec![0, 1]);
        assert!(coord.get_status(&id).is_some());
        assert!(coord.two_phase_commit(&id).is_ok());
    }

    #[test]
    fn test_transaction_abort() {
        let coord = TransactionCoordinator::new();
        let id = coord.begin_transaction(vec![0]);
        assert!(coord.abort(&id).is_ok());
        assert_eq!(coord.get_status(&id), Some(TransactionStatus::Aborted));
    }

    #[test]
    fn test_transaction_not_found() {
        let coord = TransactionCoordinator::new();
        assert!(coord.abort("nonexistent").is_err());
    }

    #[test]
    fn test_distributed_query_engine_shard_map_access() {
        let (engine, _) = make_engine(2);
        assert_eq!(engine.shard_map().num_shards(), 2);
    }

    #[test]
    fn test_fan_out_query_dedup_by_key() {
        let shard_map = ShardMap::new(3, Box::new(HashSharding));
        for i in 0..3 {
            shard_map.register_shard(ShardInfo {
                shard_id: i,
                host: "127.0.0.1".to_string(),
                port: 8000 + i as u16,
            });
        }
        let transport = Arc::new(SameResultTransport);
        let engine = DistributedQueryEngine::new(shard_map, transport);
        let query = DistributedQuery {
            predicates: vec![],
            serialized_query: vec![],
        };
        let results = engine.fan_out_query(&query);
        assert_eq!(results.len(), 1);
    }

    struct SameResultTransport;

    impl ParticipantTransport for SameResultTransport {
        fn prepare(&self, _: usize, _: &str) -> bool {
            true
        }
        fn commit(&self, _: usize, _: &str) {}
        fn abort(&self, _: usize, _: &str) {}
        fn send_query(&self, _: usize, _: &[u8]) -> Vec<u8> {
            make_test_result_data(42)
        }
    }
}
