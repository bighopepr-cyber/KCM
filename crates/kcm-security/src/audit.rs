use parking_lot::Mutex;
use std::collections::VecDeque;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum AuditEventType {
    QueryExecuted,
    FactInserted,
    FactDeleted,
    RuleExecuted,
    PermissionDenied,
}

#[derive(Debug, Clone)]
pub struct AuditEvent {
    pub event_type: AuditEventType,
    pub user_id: String,
    pub context: String,
    pub timestamp: i64,
    pub details: String,
    pub prev_hash: [u8; 32],
}

pub struct AuditLog {
    events: Arc<Mutex<VecDeque<AuditEvent>>>,
    max_events: usize,
}

impl AuditLog {
    pub fn new() -> Self {
        AuditLog {
            events: Arc::new(Mutex::new(VecDeque::new())),
            max_events: 100_000,
        }
    }

    pub fn log(&self, event: AuditEvent) {
        let mut events = self.events.lock();
        let mut chained = event;
        if let Some(prev) = events.back() {
            chained.prev_hash = Self::hash_event(prev);
        } else {
            chained.prev_hash = [0u8; 32];
        }
        events.push_back(chained);
        if events.len() > self.max_events {
            events.pop_front();
        }
    }

    pub fn log_query(&self, user_id: &str, query: &str) {
        self.log(AuditEvent {
            event_type: AuditEventType::QueryExecuted,
            user_id: user_id.to_string(),
            context: query.to_string(),
            timestamp: Self::now(),
            details: "Query executed".to_string(),
            prev_hash: [0u8; 32],
        });
    }

    pub fn log_insert(&self, user_id: &str, row_id: u64) {
        self.log(AuditEvent {
            event_type: AuditEventType::FactInserted,
            user_id: user_id.to_string(),
            context: format!("row_id={}", row_id),
            timestamp: Self::now(),
            details: "Fact inserted".to_string(),
            prev_hash: [0u8; 32],
        });
    }

    pub fn log_delete(&self, user_id: &str, row_id: u64) {
        self.log(AuditEvent {
            event_type: AuditEventType::FactDeleted,
            user_id: user_id.to_string(),
            context: format!("row_id={}", row_id),
            timestamp: Self::now(),
            details: "Fact deleted".to_string(),
            prev_hash: [0u8; 32],
        });
    }

    pub fn log_permission_denied(&self, user_id: &str, resource: &str) {
        self.log(AuditEvent {
            event_type: AuditEventType::PermissionDenied,
            user_id: user_id.to_string(),
            context: resource.to_string(),
            timestamp: Self::now(),
            details: "Permission denied".to_string(),
            prev_hash: [0u8; 32],
        });
    }

    pub fn get_events(&self) -> Vec<AuditEvent> {
        self.events.lock().clone().into()
    }

    pub fn event_count(&self) -> usize {
        self.events.lock().len()
    }

    pub fn verify_integrity(&self) -> bool {
        let events = self.events.lock();
        if events.is_empty() {
            return true;
        }
        if events[0].prev_hash != [0u8; 32] {
            return false;
        }
        for i in 1..events.len() {
            let expected = Self::hash_event(&events[i - 1]);
            if events[i].prev_hash != expected {
                return false;
            }
        }
        true
    }

    fn hash_event(event: &AuditEvent) -> [u8; 32] {
        blake3::Hasher::new()
            .update(
                format!(
                    "{:?}|{}|{}|{}|{}",
                    event.event_type, event.user_id, event.context, event.timestamp, event.details
                )
                .as_bytes(),
            )
            .finalize()
            .into()
    }

    fn now() -> i64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64
    }
}

impl Default for AuditLog {
    fn default() -> Self {
        Self::new()
    }
}
