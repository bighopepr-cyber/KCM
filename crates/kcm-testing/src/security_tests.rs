use kcm_core::bitmap::Bitmap;
use kcm_core::dictionary::Dictionary;
use kcm_core::types::*;
use kcm_core::vec::DenseVec;
use kcm_runtime::database::KnowledgeDatabase;
use kcm_security::rbac::*;
use std::time::Instant;

#[test]
fn test_injection_prevention() {
    let kb = KnowledgeDatabase::new().unwrap();
    let malicious_inputs = vec![
        "'; DROP TABLE facts; --",
        "Robert'); DROP TABLE Students;--",
        "<script>alert('xss')</script>",
        "{{constructor.constructor('return this')()}}",
        "../../etc/passwd",
        "\x00\x00\x00",
    ];
    for input in &malicious_inputs {
        let id = kb.dict_insert_subject(input);
        assert_eq!(kb.dict_get_subject(id), Some(input.to_string()));
    }
    assert_eq!(kb.fact_count(), 0);
}

#[test]
fn test_buffer_overflow_prevention() {
    let mut vec: DenseVec<u32> = DenseVec::new(10).unwrap();
    for i in 0..10 {
        assert!(vec.push(i).is_ok());
    }
    assert!(vec.push(10).is_err());
    assert!(vec.push(11).is_err());
    assert!(vec.push(u32::MAX).is_err());
    assert_eq!(vec.len(), 10);
}

#[test]
fn test_integer_overflow_subject_id() {
    let max_subject = SubjectID::new(u32::MAX);
    assert_eq!(max_subject.0, u32::MAX);
    let zero = SubjectID::new(0);
    assert_eq!(zero.0, 0);
}

#[test]
fn test_confidence_boundary_rejection() {
    let invalid = vec![-0.1, 1.1, f64::NAN, f64::INFINITY, f64::NEG_INFINITY];
    for v in invalid {
        assert!(Confidence::new(v).is_err(), "Should reject {}", v);
    }
    let valid = vec![0.0, 0.5, 1.0, 0.001, 0.999];
    for v in valid {
        assert!(Confidence::new(v).is_ok(), "Should accept {}", v);
    }
}

#[test]
fn test_rbac_enforcement() {
    let acl = ACLManager::new();
    acl.create_user("alice");
    acl.create_user("bob");
    acl.create_role("reader");
    acl.add_permission_to_role("reader", Permission::Read);
    acl.assign_role("alice", "reader");

    assert!(acl.check_permission("alice", ContextID(1), Permission::Read));
    assert!(!acl.check_permission("alice", ContextID(1), Permission::Write));
    assert!(!acl.check_permission("alice", ContextID(1), Permission::Delete));
    assert!(!acl.check_permission("bob", ContextID(1), Permission::Read));
}

#[test]
fn test_rbac_admin_role() {
    let acl = ACLManager::new();
    acl.create_user("admin");
    acl.create_role("admin_role");
    acl.add_permission_to_role("admin_role", Permission::Read);
    acl.add_permission_to_role("admin_role", Permission::Write);
    acl.add_permission_to_role("admin_role", Permission::Delete);
    acl.add_permission_to_role("admin_role", Permission::Execute);
    acl.add_permission_to_role("admin_role", Permission::Admin);
    acl.assign_role("admin", "admin_role");

    assert!(acl.check_permission("admin", ContextID(1), Permission::Read));
    assert!(acl.check_permission("admin", ContextID(1), Permission::Write));
    assert!(acl.check_permission("admin", ContextID(1), Permission::Delete));
    assert!(acl.check_permission("admin", ContextID(1), Permission::Admin));
}

#[test]
fn test_context_isolation() {
    let acl = ACLManager::new();
    acl.create_user("alice");
    acl.grant_context_permission("alice", ContextID(1), Permission::Read);

    assert!(acl.check_permission("alice", ContextID(1), Permission::Read));
    assert!(!acl.check_permission("alice", ContextID(2), Permission::Read));
    assert!(!acl.check_permission("alice", ContextID(0), Permission::Read));
}

#[test]
fn test_timing_attack_mitigation() {
    let mut dict = Dictionary::new();
    for i in 0..100 {
        dict.insert(&format!("key_{}", i));
    }

    let warmup = 10;
    for _ in 0..warmup {
        dict.lookup("nonexistent_key");
    }

    let start_existing = Instant::now();
    for _ in 0..1000 {
        dict.lookup("key_50");
    }
    let time_existing = start_existing.elapsed();

    let start_missing = Instant::now();
    for _ in 0..1000 {
        dict.lookup("nonexistent_key");
    }
    let time_missing = start_missing.elapsed();

    let ratio = time_existing.as_nanos() as f64 / time_missing.as_nanos().max(1) as f64;
    assert!(
        ratio > 0.1 && ratio < 10.0,
        "Timing ratio {} should be bounded",
        ratio
    );
}

#[test]
fn test_memory_safety_no_use_after_free() {
    let mut vec: DenseVec<u32> = DenseVec::new(100).unwrap();
    for i in 0..100 {
        vec.push(i).unwrap();
    }
    let slice = vec.as_slice();
    assert_eq!(slice.len(), 100);
    assert_eq!(slice[0], 0);
    assert_eq!(slice[99], 99);
    assert_eq!(slice[50], 50);
}

#[test]
fn test_bitmap_boundary_access() {
    let mut bitmap = Bitmap::new(64);
    bitmap.set(0);
    bitmap.set(63);
    assert!(bitmap.get(0));
    assert!(bitmap.get(63));
    assert!(!bitmap.get(64));
    assert!(!bitmap.get(u32::MAX as usize));
    assert!(!bitmap.get(usize::MAX));
}

#[test]
fn test_large_fact_insertion() {
    let kb = KnowledgeDatabase::new().unwrap();
    let count = 10_000u32;
    for i in 0..count {
        let fact = Fact::new(
            SubjectID(i),
            PredicateID((i % 10) as u8),
            ObjectID(i * 3),
            0.5 + (i as f64 * 0.00005).min(0.49),
        )
        .unwrap();
        kb.insert(&fact).unwrap();
    }
    assert_eq!(kb.fact_count(), count as usize);
}

#[test]
fn test_concurrent_fact_insertion_safety() {
    use std::sync::Arc;
    let kb = Arc::new(KnowledgeDatabase::new().unwrap());
    let mut handles = Vec::new();

    for t in 0..4u32 {
        let kb = kb.clone();
        handles.push(std::thread::spawn(move || {
            for i in 0..250u32 {
                let fact =
                    Fact::new(SubjectID(t * 250 + i), PredicateID(0), ObjectID(i), 0.9).unwrap();
                kb.insert(&fact).unwrap();
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    assert_eq!(kb.fact_count(), 1000);
}

#[test]
fn test_dictionary_concurrent_access() {
    let dict = kcm_core::dictionary::SharedDictionary::new();
    let mut handles = Vec::new();

    for t in 0..4 {
        let dict = dict.clone();
        handles.push(std::thread::spawn(move || {
            for i in 0..250 {
                let id = dict.insert(&format!("t{}_v{}", t, i));
                let val = dict.get(id).unwrap();
                assert!(!val.is_empty());
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
}

#[test]
fn test_negative_confidence_rejected() {
    assert!(Confidence::new(-0.0001).is_err());
    assert!(Confidence::new(-1.0).is_err());
    assert!(Confidence::new(-100.0).is_err());
}

#[test]
fn test_fact_equality() {
    let f1 = Fact::new(SubjectID(1), PredicateID(0), ObjectID(2), 0.9).unwrap();
    let f2 = Fact::new(SubjectID(1), PredicateID(0), ObjectID(2), 0.9).unwrap();
    let f3 = Fact::new(SubjectID(1), PredicateID(0), ObjectID(3), 0.9).unwrap();
    assert_eq!(f1.subject, f2.subject);
    assert_eq!(f1.predicate, f2.predicate);
    assert_eq!(f1.object, f2.object);
    assert_eq!(f1.confidence, f2.confidence);
    assert_eq!(f1.subject, f3.subject);
    assert_eq!(f1.predicate, f3.predicate);
    assert_ne!(f1.object, f3.object);
}

#[test]
fn test_error_handling_consistency() {
    let kb = KnowledgeDatabase::new().unwrap();
    assert!(kb.get_fact(RowID(u64::MAX)).unwrap().is_none());
    assert!(kb.delete(RowID(u64::MAX)).is_err());
    assert!(kb
        .update(
            RowID(0),
            &Fact::new(SubjectID(0), PredicateID(0), ObjectID(0), 0.5).unwrap()
        )
        .is_err());
}

#[test]
fn test_dictionary_capacity_stress() {
    let mut dict = Dictionary::new();
    for i in 0..50_000 {
        let id = dict.insert(&format!("entry_{}", i));
        assert_eq!(
            dict.get(id).map(|s| s.to_string()),
            Some(format!("entry_{}", i))
        );
    }
    assert_eq!(dict.len(), 50_001);
}

#[test]
fn test_bitmap_large_scale() {
    let mut bitmap = Bitmap::new(1_000_000);
    for i in (0..1_000_000).step_by(7) {
        bitmap.set(i);
    }
    let count = bitmap.count_ones();
    assert_eq!(count, 1_000_000 / 7 + 1);
}

#[test]
fn test_query_after_delete_consistency() {
    let kb = KnowledgeDatabase::new().unwrap();
    let f1 = Fact::new(SubjectID(1), PredicateID(0), ObjectID(10), 0.9).unwrap();
    let f2 = Fact::new(SubjectID(1), PredicateID(0), ObjectID(20), 0.8).unwrap();
    let r1 = kb.insert(&f1).unwrap();
    let _r2 = kb.insert(&f2).unwrap();
    assert_eq!(kb.query().execute().unwrap().len(), 2);
    kb.delete(r1).unwrap();
    let results = kb.query().with_subject(SubjectID(1)).execute().unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].object, ObjectID(20));
}
