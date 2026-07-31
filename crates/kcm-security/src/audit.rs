use parking_lot::Mutex;
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
}

pub struct AuditLog {
    events: Arc<Mutex<Vec<AuditEvent>>>,
    max_events: usize,
}

impl AuditLog {
    pub fn new() -> Self {
        AuditLog {
            events: Arc::new(Mutex::new(Vec::new())),
            max_events: 100_000,
        }
    }

    pub fn log(&self, event: AuditEvent) {
        let mut events = self.events.lock();
        events.push(event);
        if events.len() > self.max_events {
            events.remove(0);
        }
    }

    pub fn log_query(&self, user_id: &str, query: &str) {
        self.log(AuditEvent {
            event_type: AuditEventType::QueryExecuted,
            user_id: user_id.to_string(),
            context: query.to_string(),
            timestamp: Self::now(),
            details: "Query executed".to_string(),
        });
    }

    pub fn log_insert(&self, user_id: &str, row_id: u64) {
        self.log(AuditEvent {
            event_type: AuditEventType::FactInserted,
            user_id: user_id.to_string(),
            context: format!("row_id={}", row_id),
            timestamp: Self::now(),
            details: "Fact inserted".to_string(),
        });
    }

    pub fn log_delete(&self, user_id: &str, row_id: u64) {
        self.log(AuditEvent {
            event_type: AuditEventType::FactDeleted,
            user_id: user_id.to_string(),
            context: format!("row_id={}", row_id),
            timestamp: Self::now(),
            details: "Fact deleted".to_string(),
        });
    }

    pub fn log_permission_denied(&self, user_id: &str, resource: &str) {
        self.log(AuditEvent {
            event_type: AuditEventType::PermissionDenied,
            user_id: user_id.to_string(),
            context: resource.to_string(),
            timestamp: Self::now(),
            details: "Permission denied".to_string(),
        });
    }

    pub fn get_events(&self) -> Vec<AuditEvent> {
        self.events.lock().clone()
    }

    pub fn event_count(&self) -> usize {
        self.events.lock().len()
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
