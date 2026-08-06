#![allow(clippy::unwrap_used, clippy::panic)]
use kcm_compliance::gdpr::*;
use kcm_core::types::*;
use kcm_security::audit::*;
use kcm_security::encryption::*;
use kcm_security::rbac::*;

#[test]
fn test_rbac_create_user_and_role() {
    let acl = ACLManager::new();
    acl.create_user("alice").unwrap();
    acl.create_role("admin").unwrap();
    acl.add_permission_to_role("admin", Permission::Read).unwrap();
    acl.add_permission_to_role("admin", Permission::Write).unwrap();
    acl.add_permission_to_role("admin", Permission::Admin).unwrap();
    acl.assign_role("alice", "admin").unwrap();
    assert!(acl.check_permission("alice", ContextID(1), Permission::Read));
    assert!(acl.check_permission("alice", ContextID(1), Permission::Write));
    assert!(!acl.check_permission("alice", ContextID(1), Permission::Delete));
}

#[test]
fn test_rbac_context_permission() {
    let acl = ACLManager::new();
    acl.create_user("bob").unwrap();
    acl.grant_context_permission("bob", ContextID(5), Permission::Read).unwrap();
    assert!(acl.check_permission("bob", ContextID(5), Permission::Read));
    assert!(!acl.check_permission("bob", ContextID(3), Permission::Read));
    assert!(!acl.check_permission("bob", ContextID(5), Permission::Write));
}

#[test]
fn test_rbac_nonexistent_user() {
    let acl = ACLManager::new();
    assert!(!acl.check_permission("nobody", ContextID(0), Permission::Read));
}

#[test]
fn test_encryption_key_from_password() {
    let key1 = EncryptionKey::from_password("securepassword123", &[42u8; 32]).unwrap();
    let key2 = EncryptionKey::from_password("securepassword123", &[42u8; 32]).unwrap();
    assert_eq!(key1.as_bytes(), key2.as_bytes());
    let key3 = EncryptionKey::from_password("differentpassword", &[42u8; 32]).unwrap();
    assert_ne!(key1.as_bytes(), key3.as_bytes());
}

#[test]
fn test_encryption_key_random() {
    let key1 = EncryptionKey::random().unwrap();
    let key2 = EncryptionKey::random().unwrap();
    assert_ne!(key1.as_bytes(), key2.as_bytes());
}

#[test]
fn test_encrypt_decrypt_roundtrip() {
    let key = EncryptionKey::random().unwrap();
    let plaintext = b"Hello, KCM Security!";
    let encrypted = EncryptedStorage::encrypt(plaintext, &key).unwrap();
    assert_ne!(&encrypted[..], plaintext);
    assert!(encrypted.len() > plaintext.len());
    let decrypted = EncryptedStorage::decrypt(&encrypted, &key).unwrap();
    assert_eq!(decrypted, plaintext);
}

#[test]
fn test_encrypt_decrypt_wrong_key() {
    let key1 = EncryptionKey::random().unwrap();
    let key2 = EncryptionKey::random().unwrap();
    let plaintext = b"secret data";
    let encrypted = EncryptedStorage::encrypt(plaintext, &key1).unwrap();
    let result = EncryptedStorage::decrypt(&encrypted, &key2);
    assert!(result.is_err());
}

#[test]
fn test_encrypt_decrypt_empty() {
    let key = EncryptionKey::random().unwrap();
    let result = EncryptedStorage::encrypt(b"", &key);
    assert!(result.is_err());
}

#[test]
fn test_encrypt_decrypt_large_data() {
    let key = EncryptionKey::random().unwrap();
    let plaintext: Vec<u8> = (0..10000).map(|i| (i % 256) as u8).collect();
    let encrypted = EncryptedStorage::encrypt(&plaintext, &key).unwrap();
    let decrypted = EncryptedStorage::decrypt(&encrypted, &key).unwrap();
    assert_eq!(decrypted, plaintext);
}

#[test]
fn test_audit_log() {
    let log = AuditLog::new();
    log.log_query("alice", "SELECT * FROM facts");
    log.log_insert("alice", 42);
    log.log_delete("alice", 42);
    log.log_permission_denied("bob", "facts/42");
    assert_eq!(log.event_count(), 4);
    let events = log.get_events();
    assert_eq!(events[0].user_id, "alice");
    assert_eq!(events[3].details, "Permission denied");
}

#[test]
fn test_audit_log_eviction() {
    let log = AuditLog::new();
    for i in 0..100_100 {
        log.log_query(&format!("user_{}", i % 10), "query");
    }
    assert!(log.event_count() <= 100_000);
}

#[test]
fn test_encrypt_file_roundtrip() {
    let dir = tempfile::tempdir().unwrap();
    let src_path = dir.path().join("source.bin");
    let enc_path = dir.path().join("encrypted.bin");
    let dec_path = dir.path().join("decrypted.bin");

    let data = vec![42u8; 10000];
    std::fs::write(&src_path, &data).unwrap();

    let key = EncryptionKey::random().unwrap();
    EncryptedStorage::encrypt_file(&src_path, &enc_path, &key).unwrap();
    assert!(enc_path.exists());

    EncryptedStorage::decrypt_file(&enc_path, &dec_path, &key).unwrap();
    let decrypted = std::fs::read(&dec_path).unwrap();
    assert_eq!(decrypted, data);
}

#[test]
fn test_encrypt_file_wrong_key() {
    let dir = tempfile::tempdir().unwrap();
    let src_path = dir.path().join("source.bin");
    let enc_path = dir.path().join("encrypted.bin");
    let dec_path = dir.path().join("decrypted.bin");

    std::fs::write(&src_path, b"secret data").unwrap();

    let key1 = EncryptionKey::random().unwrap();
    let key2 = EncryptionKey::random().unwrap();

    EncryptedStorage::encrypt_file(&src_path, &enc_path, &key1).unwrap();
    let result = EncryptedStorage::decrypt_file(&enc_path, &dec_path, &key2);
    assert!(result.is_err());
}

#[test]
fn test_random_keys_unique() {
    let key1 = EncryptionKey::random().unwrap();
    let key2 = EncryptionKey::random().unwrap();
    let key3 = EncryptionKey::random().unwrap();
    assert_ne!(key1.as_bytes(), key2.as_bytes());
    assert_ne!(key2.as_bytes(), key3.as_bytes());
    assert_ne!(key1.as_bytes(), key3.as_bytes());
}

#[test]
fn test_audit_log_capacity_overflow() {
    let log = AuditLog::new();
    for i in 0..150_000 {
        log.log_query("stress_user", &format!("query_{}", i)).unwrap();
    }
    assert!(log.event_count() <= 100_000);
    let events = log.get_events();
    let last_event = events.last().unwrap();
    assert!(last_event.timestamp > 0);
}

#[test]
fn test_rbac_multi_role() {
    let acl = ACLManager::new();
    acl.create_user("manager").unwrap();
    acl.create_role("reader").unwrap();
    acl.create_role("writer").unwrap();
    acl.add_permission_to_role("reader", Permission::Read).unwrap();
    acl.add_permission_to_role("writer", Permission::Write).unwrap();
    acl.assign_role("manager", "reader").unwrap();
    acl.assign_role("manager", "writer").unwrap();

    assert!(acl.check_permission("manager", ContextID(1), Permission::Read));
    assert!(acl.check_permission("manager", ContextID(1), Permission::Write));
    assert!(!acl.check_permission("manager", ContextID(1), Permission::Delete));
}

#[test]
fn test_encrypt_large_data() {
    let key = EncryptionKey::random().unwrap();
    let data: Vec<u8> = (0..100_000).map(|i| (i % 256) as u8).collect();
    let encrypted = EncryptedStorage::encrypt(&data, &key).unwrap();
    let decrypted = EncryptedStorage::decrypt(&encrypted, &key).unwrap();
    assert_eq!(data, decrypted);
    assert!(encrypted.len() > data.len());
}

#[test]
fn test_gdpr_concurrent_access() {
    use std::sync::Arc;
    let mgr = Arc::new(GDPRManager::new());
    let mut handles = Vec::new();

    for i in 0..4 {
        let mgr = mgr.clone();
        handles.push(std::thread::spawn(move || {
            for j in 0..100 {
                let subject = DataSubject {
                    subject_id: format!("user_{}_{}", i, j),
                    email: format!("u{}{}@test.com", i, j),
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
fn test_rbac_role_revocation() {
    let acl = ACLManager::new();
    acl.create_user("alice").unwrap();
    acl.create_role("admin").unwrap();
    acl.add_permission_to_role("admin", Permission::Read).unwrap();
    acl.add_permission_to_role("admin", Permission::Write).unwrap();
    acl.assign_role("alice", "admin").unwrap();

    assert!(acl.check_permission("alice", ContextID(1), Permission::Read));
    assert!(acl.check_permission("alice", ContextID(1), Permission::Write));

    // Remove role
    acl.remove_role("alice", "admin").unwrap();
    assert!(!acl.check_permission("alice", ContextID(1), Permission::Read));
    assert!(!acl.check_permission("alice", ContextID(1), Permission::Write));
}

#[test]
fn test_audit_verify_integrity_empty() {
    let log = AuditLog::new();
    assert!(log.verify_integrity().unwrap());
}

#[test]
fn test_audit_verify_integrity_sequential() {
    let log = AuditLog::new();
    log.log_query("alice", "SELECT * FROM facts").unwrap();
    log.log_insert("bob", 42).unwrap();
    log.log_delete("alice", 42).unwrap();
    assert_eq!(log.event_count(), 3);
    assert!(log.verify_integrity().unwrap());
}

#[test]
fn test_audit_verify_integrity_overflow() {
    let log = AuditLog::new();
    for i in 0..100_000 {
        log.log_query(&format!("user_{}", i % 10), &format!("query_{}", i)).unwrap();
    }
    assert_eq!(log.event_count(), 100_000);
    assert!(log.verify_integrity().unwrap());
}
