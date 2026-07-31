use kcm_compliance::data_classification::*;
use kcm_compliance::gdpr::*;

#[test]
fn test_gdpr_full_lifecycle() {
    let mgr = GDPRManager::new();

    let subject = DataSubject {
        subject_id: "user123".to_string(),
        email: "user123@example.com".to_string(),
        consent: ConsentStatus::NotProvided,
    };
    mgr.register_subject(subject).unwrap();
    assert!(!mgr.has_consent("user123"));

    mgr.grant_consent("user123").unwrap();
    assert!(mgr.has_consent("user123"));

    let exported = mgr.export_data("user123").unwrap();
    assert!(exported.contains("user123"));

    mgr.withdraw_consent("user123").unwrap();
    assert!(!mgr.has_consent("user123"));

    mgr.delete_data("user123").unwrap();
    assert!(mgr.export_data("user123").is_err());
}

#[test]
fn test_gdpr_duplicate_registration() {
    let mgr = GDPRManager::new();
    let subject = DataSubject {
        subject_id: "user1".to_string(),
        email: "u1@test.com".to_string(),
        consent: ConsentStatus::NotProvided,
    };
    mgr.register_subject(subject).unwrap();
    let subject2 = DataSubject {
        subject_id: "user1".to_string(),
        email: "u1b@test.com".to_string(),
        consent: ConsentStatus::NotProvided,
    };
    assert!(mgr.register_subject(subject2).is_err());
}

#[test]
fn test_gdpr_delete_nonexistent() {
    let mgr = GDPRManager::new();
    assert!(mgr.delete_data("nonexistent").is_err());
}

#[test]
fn test_gdpr_concurrent_operations() {
    use std::sync::Arc;
    let mgr = Arc::new(GDPRManager::new());
    let mut handles = Vec::new();

    for i in 0..8 {
        let mgr = mgr.clone();
        handles.push(std::thread::spawn(move || {
            for j in 0..50 {
                let subject = DataSubject {
                    subject_id: format!("user_{}_{}", i, j),
                    email: format!("u{}{}@t.com", i, j),
                    consent: ConsentStatus::NotProvided,
                };
                mgr.register_subject(subject).ok();
                mgr.grant_consent(&format!("user_{}_{}", i, j)).ok();
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
}

#[test]
fn test_data_classification_enforcement() {
    assert!(!DataClassification::Public.requires_encryption());
    assert!(!DataClassification::Public.requires_audit_log());
    assert_eq!(DataClassification::Public.max_retention_days(), Some(2555));

    assert!(!DataClassification::Internal.requires_encryption());
    assert!(!DataClassification::Internal.requires_audit_log());
    assert_eq!(
        DataClassification::Internal.max_retention_days(),
        Some(1095)
    );

    assert!(DataClassification::Confidential.requires_encryption());
    assert!(!DataClassification::Confidential.requires_audit_log());
    assert_eq!(
        DataClassification::Confidential.max_retention_days(),
        Some(365)
    );

    assert!(DataClassification::Restricted.requires_encryption());
    assert!(DataClassification::Restricted.requires_audit_log());
    assert_eq!(
        DataClassification::Restricted.max_retention_days(),
        Some(180)
    );
}

#[test]
fn test_classified_fact_retention() {
    let now = 1_000_000_000i64;
    let fact_public = ClassifiedFact {
        fact_id: 1,
        classification: DataClassification::Public,
        owner: "admin".to_string(),
        created_at: now,
    };
    assert!(fact_public.should_retain(now + 86400 * 365));

    let fact_restricted = ClassifiedFact {
        fact_id: 2,
        classification: DataClassification::Restricted,
        owner: "admin".to_string(),
        created_at: now,
    };
    assert!(fact_restricted.should_retain(now + 86400 * 100));
    assert!(!fact_restricted.should_retain(now + 86400 * 200));
}
