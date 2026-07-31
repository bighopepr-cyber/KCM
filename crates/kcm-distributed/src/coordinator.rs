use parking_lot::Mutex;
use std::collections::HashMap;
use std::sync::Arc;

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
}

impl TransactionCoordinator {
    pub fn new() -> Self {
        TransactionCoordinator {
            transactions: Arc::new(Mutex::new(HashMap::new())),
            next_id: std::sync::atomic::AtomicU64::new(1),
        }
    }

    pub fn begin_transaction(&self, participants: Vec<usize>) -> String {
        let id = self
            .next_id
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let txn_id = format!("txn-{}", id);
        let txn = DistributedTransaction::new(txn_id.clone(), participants);
        self.transactions.lock().insert(txn_id.clone(), txn);
        txn_id
    }

    pub fn two_phase_commit(&self, txn_id: &str) -> Result<(), String> {
        let mut txns = self.transactions.lock();
        let txn = txns
            .get_mut(txn_id)
            .ok_or_else(|| "Transaction not found".to_string())?;

        for _shard in &txn.participants {
            txn.record_vote(true);
        }

        if txn.all_voted() {
            txn.status = TransactionStatus::Committed;
            Ok(())
        } else {
            txn.status = TransactionStatus::Aborted;
            Err("Not all shards voted to commit".to_string())
        }
    }

    pub fn abort(&self, txn_id: &str) -> Result<(), String> {
        let mut txns = self.transactions.lock();
        if let Some(txn) = txns.get_mut(txn_id) {
            txn.status = TransactionStatus::Aborted;
            Ok(())
        } else {
            Err("Transaction not found".to_string())
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
