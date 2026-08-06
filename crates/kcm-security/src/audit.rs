use kcm_core::types::*;
use parking_lot::Mutex;
use std::collections::VecDeque;
use std::sync::Arc;

pub const MAX_EVENTS: usize = 100_000;
pub const MAX_USER_ID_LENGTH: usize = 256;
pub const MAX_CONTEXT_LENGTH: usize = 4096;
pub const MAX_DETAILS_LENGTH: usize = 8192;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AuditEventType {
    QueryExecuted,
    FactInserted,
    FactDeleted,
    RuleExecuted,
    PermissionDenied,
    EncryptionOperation,
    KeyDerivation,
    AccessControlCheck,
}

impl AuditEventType {
    pub fn name(&self) -> &'static str {
        match self {
            AuditEventType::QueryExecuted => "query_executed",
            AuditEventType::FactInserted => "fact_inserted",
            AuditEventType::FactDeleted => "fact_deleted",
            AuditEventType::RuleExecuted => "rule_executed",
            AuditEventType::PermissionDenied => "permission_denied",
            AuditEventType::EncryptionOperation => "encryption_operation",
            AuditEventType::KeyDerivation => "key_derivation",
            AuditEventType::AccessControlCheck => "access_control_check",
        }
    }
}

#[derive(Debug, Clone)]
pub struct AuditEvent {
    pub event_type: AuditEventType,
    pub user_id: String,
    pub context: String,
    pub timestamp: i64,
    pub details: String,
    pub prev_hash: [u8; 32],
    pub event_hash: [u8; 32],
}

impl AuditEvent {
    pub fn new(
        event_type: AuditEventType,
        user_id: &str,
        context: &str,
        details: &str,
    ) -> Result<Self, KcmError> {
        Self::validate_user_id(user_id)?;
        Self::validate_context(context)?;
        Self::validate_details(details)?;

        Ok(AuditEvent {
            event_type,
            user_id: user_id.to_string(),
            context: context.to_string(),
            timestamp: Self::now(),
            details: details.to_string(),
            prev_hash: [0u8; 32],
            event_hash: [0u8; 32],
        })
    }

    fn validate_user_id(user_id: &str) -> Result<(), KcmError> {
        if user_id.is_empty() {
            return Err(KcmError::InvalidArgument(
                "Audit event user_id cannot be empty".to_string(),
            ));
        }
        if user_id.len() > MAX_USER_ID_LENGTH {
            return Err(KcmError::InvalidArgument(format!(
                "Audit event user_id cannot exceed {} characters",
                MAX_USER_ID_LENGTH
            )));
        }
        Ok(())
    }

    fn validate_context(context: &str) -> Result<(), KcmError> {
        if context.len() > MAX_CONTEXT_LENGTH {
            return Err(KcmError::InvalidArgument(format!(
                "Audit event context cannot exceed {} characters",
                MAX_CONTEXT_LENGTH
            )));
        }
        Ok(())
    }

    fn validate_details(details: &str) -> Result<(), KcmError> {
        if details.len() > MAX_DETAILS_LENGTH {
            return Err(KcmError::InvalidArgument(format!(
                "Audit event details cannot exceed {} characters",
                MAX_DETAILS_LENGTH
            )));
        }
        Ok(())
    }

    fn now() -> i64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64
    }

    pub fn compute_hash(&self) -> [u8; 32] {
        blake3::Hasher::new()
            .update(&self.prev_hash)
            .update(
                format!(
                    "{:?}|{}|{}|{}|{}",
                    self.event_type, self.user_id, self.context, self.timestamp, self.details
                )
                .as_bytes(),
            )
            .finalize()
            .into()
    }
}

pub struct AuditLog {
    events: Arc<Mutex<VecDeque<AuditEvent>>>,
    max_events: usize,
}

impl AuditLog {
    pub fn new() -> Self {
        AuditLog {
            events: Arc::new(Mutex::new(VecDeque::new())),
            max_events: MAX_EVENTS,
        }
    }

    pub fn with_capacity(max_events: usize) -> Self {
        AuditLog {
            events: Arc::new(Mutex::new(VecDeque::with_capacity(max_events.min(1024)))),
            max_events,
        }
    }

    pub fn log(&self, mut event: AuditEvent) {
        let mut events = self.events.lock();

        if let Some(prev) = events.back() {
            event.prev_hash = prev.event_hash;
        } else {
            event.prev_hash = [0u8; 32];
        }

        event.event_hash = event.compute_hash();
        events.push_back(event);

        while events.len() > self.max_events {
            events.pop_front();
        }

        if let Some(first) = events.front_mut() {
            if first.prev_hash != [0u8; 32] {
                first.prev_hash = [0u8; 32];
                first.event_hash = first.compute_hash();
            }
        }
    }

    pub fn log_query(&self, user_id: &str, query: &str) -> Result<(), KcmError> {
        let event = AuditEvent::new(
            AuditEventType::QueryExecuted,
            user_id,
            query,
            "Query executed",
        )?;
        self.log(event);
        Ok(())
    }

    pub fn log_insert(&self, user_id: &str, row_id: u64) -> Result<(), KcmError> {
        let event = AuditEvent::new(
            AuditEventType::FactInserted,
            user_id,
            &format!("row_id={}", row_id),
            "Fact inserted",
        )?;
        self.log(event);
        Ok(())
    }

    pub fn log_delete(&self, user_id: &str, row_id: u64) -> Result<(), KcmError> {
        let event = AuditEvent::new(
            AuditEventType::FactDeleted,
            user_id,
            &format!("row_id={}", row_id),
            "Fact deleted",
        )?;
        self.log(event);
        Ok(())
    }

    pub fn log_permission_denied(&self, user_id: &str, resource: &str) -> Result<(), KcmError> {
        let event = AuditEvent::new(
            AuditEventType::PermissionDenied,
            user_id,
            resource,
            "Permission denied",
        )?;
        self.log(event);
        Ok(())
    }

    pub fn log_rule(&self, user_id: &str, rule_id: u32) -> Result<(), KcmError> {
        let event = AuditEvent::new(
            AuditEventType::RuleExecuted,
            user_id,
            &format!("rule_id={}", rule_id),
            "Rule executed",
        )?;
        self.log(event);
        Ok(())
    }

    pub fn log_encryption(&self, user_id: &str, operation: &str) -> Result<(), KcmError> {
        let event = AuditEvent::new(
            AuditEventType::EncryptionOperation,
            user_id,
            operation,
            "Encryption operation",
        )?;
        self.log(event);
        Ok(())
    }

    pub fn log_access_check(
        &self,
        user_id: &str,
        resource: &str,
        granted: bool,
    ) -> Result<(), KcmError> {
        let details = if granted {
            "Access granted"
        } else {
            "Access denied"
        };
        let event = AuditEvent::new(
            AuditEventType::AccessControlCheck,
            user_id,
            resource,
            details,
        )?;
        self.log(event);
        Ok(())
    }

    pub fn get_events(&self) -> Vec<AuditEvent> {
        self.events.lock().clone().into()
    }

    pub fn get_events_since(&self, timestamp: i64) -> Vec<AuditEvent> {
        self.events
            .lock()
            .iter()
            .filter(|e| e.timestamp >= timestamp)
            .cloned()
            .collect()
    }

    pub fn get_events_by_type(&self, event_type: AuditEventType) -> Vec<AuditEvent> {
        self.events
            .lock()
            .iter()
            .filter(|e| e.event_type == event_type)
            .cloned()
            .collect()
    }

    pub fn get_events_by_user(&self, user_id: &str) -> Vec<AuditEvent> {
        self.events
            .lock()
            .iter()
            .filter(|e| e.user_id == user_id)
            .cloned()
            .collect()
    }

    pub fn event_count(&self) -> usize {
        self.events.lock().len()
    }

    pub fn verify_integrity(&self) -> Result<bool, KcmError> {
        let events = self.events.lock();
        if events.is_empty() {
            return Ok(true);
        }

        if events[0].prev_hash != [0u8; 32] {
            return Ok(false);
        }

        for i in 1..events.len() {
            let expected_hash = events[i - 1].event_hash;
            if events[i].prev_hash != expected_hash {
                return Ok(false);
            }
        }

        for event in events.iter() {
            let computed = event.compute_hash();
            if event.event_hash != computed {
                return Ok(false);
            }
        }

        Ok(true)
    }

    pub fn clear(&self) {
        self.events.lock().clear();
    }

    pub fn is_empty(&self) -> bool {
        self.events.lock().is_empty()
    }

    pub fn capacity(&self) -> usize {
        self.max_events
    }
}

impl Default for AuditLog {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_log_basic() {
        let log = AuditLog::new();
        log.log_query("user1", "SELECT * FROM facts").unwrap();
        assert_eq!(log.event_count(), 1);
        assert!(log.verify_integrity().unwrap());
    }

    #[test]
    fn test_audit_log_integrity() {
        let log = AuditLog::new();
        log.log_query("user1", "query1").unwrap();
        log.log_insert("user1", 1).unwrap();
        log.log_delete("user1", 2).unwrap();
        assert_eq!(log.event_count(), 3);
        assert!(log.verify_integrity().unwrap());
    }

    #[test]
    fn test_audit_log_capacity() {
        let log = AuditLog::with_capacity(5);
        for i in 0..10 {
            log.log_query("user1", &format!("query_{}", i)).unwrap();
        }
        assert_eq!(log.event_count(), 5);
    }

    #[test]
    fn test_audit_log_filtering() {
        let log = AuditLog::new();
        log.log_query("user1", "query1").unwrap();
        log.log_insert("user1", 1).unwrap();
        log.log_delete("user2", 2).unwrap();

        let query_events = log.get_events_by_type(AuditEventType::QueryExecuted);
        assert_eq!(query_events.len(), 1);

        let user1_events = log.get_events_by_user("user1");
        assert_eq!(user1_events.len(), 2);
    }

    #[test]
    fn test_audit_log_clear() {
        let log = AuditLog::new();
        log.log_query("user1", "query1").unwrap();
        assert!(!log.is_empty());
        log.clear();
        assert!(log.is_empty());
    }

    #[test]
    fn test_audit_event_validation() {
        assert!(AuditEvent::new(AuditEventType::QueryExecuted, "", "ctx", "det").is_err());
        assert!(AuditEvent::new(
            AuditEventType::QueryExecuted,
            "user",
            &"c".repeat(4097),
            "det"
        )
        .is_err());
        assert!(AuditEvent::new(
            AuditEventType::QueryExecuted,
            "user",
            "ctx",
            &"d".repeat(8193)
        )
        .is_err());
    }

    #[test]
    fn test_audit_event_types() {
        assert_eq!(AuditEventType::QueryExecuted.name(), "query_executed");
        assert_eq!(AuditEventType::FactInserted.name(), "fact_inserted");
        assert_eq!(AuditEventType::PermissionDenied.name(), "permission_denied");
    }
}
