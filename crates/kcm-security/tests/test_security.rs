use kcm_core::types::*;
use kcm_security::audit::*;
use kcm_security::encryption::*;
use kcm_security::rbac::*;

#[test]
fn test_rbac_create_user_and_role() {
    let acl = ACLManager::new();
    acl.create_user("alice");
    acl.create_role("admin");
    acl.add_permission_to_role("admin", Permission::Read);
    acl.add_permission_to_role("admin", Permission::Write);
    acl.add_permission_to_role("admin", Permission::Admin);
    acl.assign_role("alice", "admin");
    assert!(acl.check_permission("alice", ContextID(1), Permission::Read));
    assert!(acl.check_permission("alice", ContextID(1), Permission::Write));
    assert!(!acl.check_permission("alice", ContextID(1), Permission::Delete));
}

#[test]
fn test_rbac_context_permission() {
    let acl = ACLManager::new();
    acl.create_user("bob");
    acl.grant_context_permission("bob", ContextID(5), Permission::Read);
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
    let key1 = EncryptionKey::from_password("hello", &[42u8; 32]);
    let key2 = EncryptionKey::from_password("hello", &[42u8; 32]);
    assert_eq!(key1.as_bytes(), key2.as_bytes());
    let key3 = EncryptionKey::from_password("world", &[42u8; 32]);
    assert_ne!(key1.as_bytes(), key3.as_bytes());
}

#[test]
fn test_encryption_key_random() {
    let key1 = EncryptionKey::random();
    let key2 = EncryptionKey::random();
    assert_ne!(key1.as_bytes(), key2.as_bytes());
}

#[test]
fn test_encrypt_decrypt_roundtrip() {
    let key = EncryptionKey::random();
    let plaintext = b"Hello, KCM Security!";
    let encrypted = EncryptedStorage::encrypt(plaintext, &key).unwrap();
    assert_ne!(&encrypted[..], plaintext);
    assert!(encrypted.len() > plaintext.len());
    let decrypted = EncryptedStorage::decrypt(&encrypted, &key).unwrap();
    assert_eq!(decrypted, plaintext);
}

#[test]
fn test_encrypt_decrypt_wrong_key() {
    let key1 = EncryptionKey::random();
    let key2 = EncryptionKey::random();
    let plaintext = b"secret data";
    let encrypted = EncryptedStorage::encrypt(plaintext, &key1).unwrap();
    let result = EncryptedStorage::decrypt(&encrypted, &key2);
    assert!(result.is_err());
}

#[test]
fn test_encrypt_decrypt_empty() {
    let key = EncryptionKey::random();
    let encrypted = EncryptedStorage::encrypt(b"", &key).unwrap();
    let decrypted = EncryptedStorage::decrypt(&encrypted, &key).unwrap();
    assert!(decrypted.is_empty());
}

#[test]
fn test_encrypt_decrypt_large_data() {
    let key = EncryptionKey::random();
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
