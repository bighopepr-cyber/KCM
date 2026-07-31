use kcm_core::types::KcmError;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConsentStatus {
    Granted,
    Withdrawn,
    NotProvided,
}

#[derive(Debug, Clone)]
pub struct DataSubject {
    pub subject_id: String,
    pub email: String,
    pub consent: ConsentStatus,
}

pub struct GDPRManager {
    subjects: Arc<RwLock<HashMap<String, DataSubject>>>,
}

impl GDPRManager {
    pub fn new() -> Self {
        GDPRManager {
            subjects: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn register_subject(&self, subject: DataSubject) -> Result<(), KcmError> {
        let mut subjects = self.subjects.write();
        if subjects.contains_key(&subject.subject_id) {
            return Err(KcmError::InvalidArgument(
                "Subject already exists".to_string(),
            ));
        }
        subjects.insert(subject.subject_id.clone(), subject);
        Ok(())
    }

    pub fn grant_consent(&self, subject_id: &str) -> Result<(), KcmError> {
        let mut subjects = self.subjects.write();
        match subjects.get_mut(subject_id) {
            Some(s) => {
                s.consent = ConsentStatus::Granted;
                Ok(())
            }
            None => Err(KcmError::NotFound("Subject not found".to_string())),
        }
    }

    pub fn withdraw_consent(&self, subject_id: &str) -> Result<(), KcmError> {
        let mut subjects = self.subjects.write();
        match subjects.get_mut(subject_id) {
            Some(s) => {
                s.consent = ConsentStatus::Withdrawn;
                Ok(())
            }
            None => Err(KcmError::NotFound("Subject not found".to_string())),
        }
    }

    pub fn has_consent(&self, subject_id: &str) -> bool {
        self.subjects
            .read()
            .get(subject_id)
            .map(|s| s.consent == ConsentStatus::Granted)
            .unwrap_or(false)
    }

    pub fn export_data(&self, subject_id: &str) -> Result<String, KcmError> {
        self.subjects
            .read()
            .get(subject_id)
            .map(|s| format!("{:?}", s))
            .ok_or_else(|| KcmError::NotFound("Subject not found".to_string()))
    }

    pub fn delete_data(&self, subject_id: &str) -> Result<(), KcmError> {
        if self.subjects.write().remove(subject_id).is_none() {
            return Err(KcmError::NotFound(format!(
                "Subject {} not found",
                subject_id
            )));
        }
        Ok(())
    }
}

impl Default for GDPRManager {
    fn default() -> Self {
        Self::new()
    }
}
