use kcm_compliance::data_classification::*;
use kcm_compliance::gdpr::*;
use kcm_core::types::{ContextID, EvidenceID, Fact, ObjectID, PredicateID, SubjectID};

#[test]
fn test_gdpr_register_and_consent() {
    let mgr = GDPRManager::new();
    let subject = DataSubject {
        subject_id: "user1".to_string(),
        email: "user1@example.com".to_string(),
        consent: ConsentStatus::NotProvided,
    };
    mgr.register_subject(subject).unwrap();
    assert!(!mgr.has_consent("user1"));
    mgr.grant_consent("user1").unwrap();
    assert!(mgr.has_consent("user1"));
}

#[test]
fn test_gdpr_withdraw_consent() {
    let mgr = GDPRManager::new();
    let subject = DataSubject {
        subject_id: "user1".to_string(),
        email: "test@example.com".to_string(),
        consent: ConsentStatus::Granted,
    };
    mgr.register_subject(subject).unwrap();
    assert!(mgr.has_consent("user1"));
    mgr.withdraw_consent("user1").unwrap();
    assert!(!mgr.has_consent("user1"));
}

#[test]
fn test_gdpr_export_delete() {
    let mgr = GDPRManager::new();
    let subject = DataSubject {
        subject_id: "user1".to_string(),
        email: "test@example.com".to_string(),
        consent: ConsentStatus::Granted,
    };
    mgr.register_subject(subject).unwrap();
    let data = mgr.export_data("user1").unwrap();
    assert!(data.contains("user1"));
    mgr.delete_data("user1").unwrap();
    assert!(mgr.export_data("user1").is_err());
}

#[test]
fn test_gdpr_errors() {
    let mgr = GDPRManager::new();
    assert!(mgr.grant_consent("nonexistent").is_err());
    assert!(mgr.withdraw_consent("nonexistent").is_err());
    assert!(mgr.export_data("nonexistent").is_err());
    let subject = DataSubject {
        subject_id: "u1".to_string(),
        email: "a@b.com".to_string(),
        consent: ConsentStatus::NotProvided,
    };
    mgr.register_subject(subject).unwrap();
    assert!(mgr
        .register_subject(DataSubject {
            subject_id: "u1".to_string(),
            email: "c@d.com".to_string(),
            consent: ConsentStatus::NotProvided,
        })
        .is_err());
}

#[test]
fn test_data_classification() {
    assert!(!DataClassification::Public.requires_encryption());
    assert!(!DataClassification::Internal.requires_encryption());
    assert!(DataClassification::Confidential.requires_encryption());
    assert!(DataClassification::Restricted.requires_encryption());
    assert!(!DataClassification::Public.requires_audit_log());
    assert!(DataClassification::Internal.requires_audit_log());
    assert!(DataClassification::Confidential.requires_audit_log());
    assert!(DataClassification::Restricted.requires_audit_log());
}

#[test]
fn test_data_classification_retention() {
    assert!(DataClassification::Public.max_retention_days().is_some());
    assert!(DataClassification::Restricted
        .max_retention_days()
        .is_some());
}

#[test]
fn test_classified_fact_retention() {
    let fact = ClassifiedFact {
        fact: Fact {
            subject: SubjectID(0),
            predicate: PredicateID(0),
            object: ObjectID(0),
            confidence: 1.0,
            evidence: EvidenceID::UNKNOWN,
            timestamp: 1_000_000,
            context: ContextID::NULL,
            version: 1,
            priority: 0,
            owner: 0,
        },
        classification: DataClassification::Confidential,
    };
    assert!(fact.should_retain(1_000_000 + 86400 * 300));
    assert!(!fact.should_retain(1_000_000 + 86400 * 1826));
}
