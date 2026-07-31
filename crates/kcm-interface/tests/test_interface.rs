use kcm_core::types::*;
use kcm_interface::*;

#[test]
fn test_kcm_database_new_and_free() {
    let mut db: *mut KCM_Database = std::ptr::null_mut();
    let result = unsafe { KCM_DatabaseNew(&mut db) };
    assert_eq!(result, KCM_Error::KCM_OK);
    assert!(!db.is_null());
    unsafe { KCM_DatabaseFree(db) };
}

#[test]
fn test_kcm_database_insert() {
    let mut db: *mut KCM_Database = std::ptr::null_mut();
    unsafe { KCM_DatabaseNew(&mut db) };

    let fact = KCM_Fact {
        subject: 1,
        predicate: 0,
        object: 2,
        confidence: 0.9,
        evidence: 0,
        timestamp: 1234567890,
        context: 0,
    };

    let result = unsafe { KCM_DatabaseInsert(db, &fact) };
    assert_eq!(result, KCM_Error::KCM_OK);

    unsafe { KCM_DatabaseFree(db) };
}

#[test]
fn test_kcm_database_insert_null_args() {
    let fact = KCM_Fact {
        subject: 1,
        predicate: 0,
        object: 2,
        confidence: 0.9,
        evidence: 0,
        timestamp: 1234567890,
        context: 0,
    };

    let result = unsafe { KCM_DatabaseInsert(std::ptr::null_mut(), &fact) };
    assert_eq!(result, KCM_Error::KCM_ERR_INVALID_ARGUMENT);

    let mut db: *mut KCM_Database = std::ptr::null_mut();
    unsafe { KCM_DatabaseNew(&mut db) };
    let result = unsafe { KCM_DatabaseInsert(db, std::ptr::null()) };
    assert_eq!(result, KCM_Error::KCM_ERR_INVALID_ARGUMENT);

    unsafe { KCM_DatabaseFree(db) };
}

#[test]
fn test_kcm_query_next() {
    let mut db: *mut KCM_Database = std::ptr::null_mut();
    unsafe { KCM_DatabaseNew(&mut db) };

    for i in 0..5u32 {
        let fact = KCM_Fact {
            subject: i,
            predicate: 0,
            object: i * 10,
            confidence: 0.9,
            evidence: 0,
            timestamp: 1234567890,
            context: 0,
        };
        unsafe { KCM_DatabaseInsert(db, &fact) };
    }

    let mut query: *mut KCM_Query = std::ptr::null_mut();
    let result = unsafe { KCM_DatabaseQuery(db, &mut query) };
    assert_eq!(result, KCM_Error::KCM_OK);
    assert!(!query.is_null());

    let mut fact_out = KCM_Fact {
        subject: 0,
        predicate: 0,
        object: 0,
        confidence: 0.0,
        evidence: 0,
        timestamp: 0,
        context: 0,
    };
    let mut has_next = false;

    let mut count = 0;
    loop {
        let result = unsafe { KCM_QueryNext(query, &mut fact_out, &mut has_next) };
        assert_eq!(result, KCM_Error::KCM_OK);
        count += 1;
        if !has_next {
            break;
        }
    }

    assert_eq!(count, 5);

    unsafe { KCM_QueryFree(query) };
    unsafe { KCM_DatabaseFree(db) };
}

#[test]
fn test_kcm_error_message() {
    let msg = unsafe { KCM_ErrorMessage(KCM_Error::KCM_OK) };
    assert!(!msg.is_null());

    let msg = unsafe { KCM_ErrorMessage(KCM_Error::KCM_ERR_NOT_FOUND) };
    assert!(!msg.is_null());

    let msg = unsafe { KCM_ErrorMessage(KCM_Error::KCM_ERR_IO) };
    assert!(!msg.is_null());
}

#[test]
fn test_kcm_database_new_null() {
    let result = unsafe { KCM_DatabaseNew(std::ptr::null_mut()) };
    assert_eq!(result, KCM_Error::KCM_ERR_INVALID_ARGUMENT);
}

#[test]
fn test_kcm_query_next_null() {
    let mut fact_out = KCM_Fact {
        subject: 0,
        predicate: 0,
        object: 0,
        confidence: 0.0,
        evidence: 0,
        timestamp: 0,
        context: 0,
    };
    let mut has_next = false;

    let result = unsafe { KCM_QueryNext(std::ptr::null_mut(), &mut fact_out, &mut has_next) };
    assert_eq!(result, KCM_Error::KCM_ERR_INVALID_ARGUMENT);
}

#[test]
fn test_kcm_fact_conversion() {
    let kcm_fact = KCM_Fact {
        subject: 42,
        predicate: 7,
        object: 99,
        confidence: 0.77,
        evidence: 3,
        timestamp: 9876543210,
        context: 2,
    };

    let fact = Fact::from(&kcm_fact);
    assert_eq!(fact.subject, SubjectID(42));
    assert_eq!(fact.predicate, PredicateID(7));
    assert_eq!(fact.object, ObjectID(99));
    assert_eq!(fact.confidence, 0.77);
    assert_eq!(fact.evidence, EvidenceID(3));
    assert_eq!(fact.timestamp, 9876543210);
    assert_eq!(fact.context, ContextID(2));
}

#[test]
fn test_kcm_roundtrip_conversion() {
    let original = Fact::new(SubjectID(10), PredicateID(5), ObjectID(20), 0.85).unwrap();

    let kcm_fact = KCM_Fact::from(&original);
    let converted = Fact::from(&kcm_fact);

    assert_eq!(original.subject, converted.subject);
    assert_eq!(original.predicate, converted.predicate);
    assert_eq!(original.object, converted.object);
    assert_eq!(original.confidence, converted.confidence);
}

#[test]
fn test_kcm_database_lifecycle() {
    let mut db: *mut KCM_Database = std::ptr::null_mut();
    let result = unsafe { KCM_DatabaseNew(&mut db) };
    assert_eq!(result, KCM_Error::KCM_OK);
    assert!(!db.is_null());

    let count = unsafe { KCM_DatabaseFactCount(db) };
    assert_eq!(count, 0);

    let active = unsafe { KCM_DatabaseActiveCount(db) };
    assert_eq!(active, 0);

    unsafe { KCM_DatabaseFree(db) };
}

#[test]
fn test_kcm_insert_query_roundtrip() {
    let mut db: *mut KCM_Database = std::ptr::null_mut();
    unsafe { KCM_DatabaseNew(&mut db) };

    let fact = KCM_Fact {
        subject: 42,
        predicate: 7,
        object: 99,
        confidence: 0.77,
        evidence: 3,
        timestamp: 9876543210,
        context: 2,
    };
    let result = unsafe { KCM_DatabaseInsert(db, &fact) };
    assert_eq!(result, KCM_Error::KCM_OK);

    let count = unsafe { KCM_DatabaseFactCount(db) };
    assert_eq!(count, 1);

    let mut query: *mut KCM_Query = std::ptr::null_mut();
    let result = unsafe { KCM_DatabaseQuery(db, &mut query) };
    assert_eq!(result, KCM_Error::KCM_OK);

    let mut fact_out = KCM_Fact {
        subject: 0,
        predicate: 0,
        object: 0,
        confidence: 0.0,
        evidence: 0,
        timestamp: 0,
        context: 0,
    };
    let mut has_next = false;
    let result = unsafe { KCM_QueryNext(query, &mut fact_out, &mut has_next) };
    assert_eq!(result, KCM_Error::KCM_OK);
    assert!(!has_next);
    assert_eq!(fact_out.subject, 42);
    assert_eq!(fact_out.predicate, 7);
    assert_eq!(fact_out.object, 99);

    unsafe { KCM_QueryFree(query) };
    unsafe { KCM_DatabaseFree(db) };
}

#[test]
fn test_kcm_query_iterator_exhaustion() {
    let mut db: *mut KCM_Database = std::ptr::null_mut();
    unsafe { KCM_DatabaseNew(&mut db) };

    for i in 0..10u32 {
        let fact = KCM_Fact {
            subject: i,
            predicate: 0,
            object: i * 10,
            confidence: 0.9,
            evidence: 0,
            timestamp: 0,
            context: 0,
        };
        unsafe { KCM_DatabaseInsert(db, &fact) };
    }

    let mut query: *mut KCM_Query = std::ptr::null_mut();
    unsafe { KCM_DatabaseQuery(db, &mut query) };

    let mut fact_out = KCM_Fact {
        subject: 0,
        predicate: 0,
        object: 0,
        confidence: 0.0,
        evidence: 0,
        timestamp: 0,
        context: 0,
    };
    let mut has_next = true;
    let mut count = 0;

    while has_next {
        let result = unsafe { KCM_QueryNext(query, &mut fact_out, &mut has_next) };
        assert_eq!(result, KCM_Error::KCM_OK);
        if !has_next && count < 10 {
            count += 1;
        } else if has_next {
            count += 1;
        }
    }

    assert_eq!(count, 10);

    let mut fact_out2 = KCM_Fact {
        subject: 0,
        predicate: 0,
        object: 0,
        confidence: 0.0,
        evidence: 0,
        timestamp: 0,
        context: 0,
    };
    let mut has_next2 = false;
    let result = unsafe { KCM_QueryNext(query, &mut fact_out2, &mut has_next2) };
    assert_eq!(result, KCM_Error::KCM_OK);
    assert!(!has_next2);

    unsafe { KCM_QueryFree(query) };
    unsafe { KCM_DatabaseFree(db) };
}

#[test]
fn test_kcm_delete_and_count() {
    let mut db: *mut KCM_Database = std::ptr::null_mut();
    unsafe { KCM_DatabaseNew(&mut db) };

    for i in 0..5u32 {
        let fact = KCM_Fact {
            subject: i,
            predicate: 0,
            object: i,
            confidence: 0.9,
            evidence: 0,
            timestamp: 0,
            context: 0,
        };
        unsafe { KCM_DatabaseInsert(db, &fact) };
    }

    assert_eq!(unsafe { KCM_DatabaseFactCount(db) }, 5);
    assert_eq!(unsafe { KCM_DatabaseActiveCount(db) }, 5);

    let result = unsafe { KCM_DatabaseDelete(db, 2) };
    assert_eq!(result, KCM_Error::KCM_OK);

    assert_eq!(unsafe { KCM_DatabaseFactCount(db) }, 5);
    assert_eq!(unsafe { KCM_DatabaseActiveCount(db) }, 4);

    unsafe { KCM_DatabaseFree(db) };
}

#[test]
fn test_kcm_update_and_retrieve() {
    let mut db: *mut KCM_Database = std::ptr::null_mut();
    unsafe { KCM_DatabaseNew(&mut db) };

    let fact1 = KCM_Fact {
        subject: 1,
        predicate: 0,
        object: 10,
        confidence: 0.9,
        evidence: 0,
        timestamp: 0,
        context: 0,
    };
    unsafe { KCM_DatabaseInsert(db, &fact1) };

    let fact2 = KCM_Fact {
        subject: 2,
        predicate: 1,
        object: 20,
        confidence: 0.8,
        evidence: 0,
        timestamp: 0,
        context: 0,
    };
    let result = unsafe { KCM_DatabaseUpdate(db, 0, &fact2) };
    assert_eq!(result, KCM_Error::KCM_OK);

    let mut query: *mut KCM_Query = std::ptr::null_mut();
    unsafe { KCM_DatabaseQuery(db, &mut query) };

    let mut fact_out = KCM_Fact {
        subject: 0,
        predicate: 0,
        object: 0,
        confidence: 0.0,
        evidence: 0,
        timestamp: 0,
        context: 0,
    };
    let mut has_next = true;
    unsafe { KCM_QueryNext(query, &mut fact_out, &mut has_next) };
    assert_eq!(fact_out.subject, 2);
    assert_eq!(fact_out.predicate, 1);

    unsafe { KCM_QueryFree(query) };
    unsafe { KCM_DatabaseFree(db) };
}

#[test]
fn test_kcm_error_messages_all_variants() {
    let variants = vec![
        KCM_Error::KCM_OK,
        KCM_Error::KCM_ERR_NOT_FOUND,
        KCM_Error::KCM_ERR_OUT_OF_MEMORY,
        KCM_Error::KCM_ERR_INVALID_ARGUMENT,
        KCM_Error::KCM_ERR_IO,
        KCM_Error::KCM_ERR_CORRUPTED,
        KCM_Error::KCM_ERR_CONFLICT,
        KCM_Error::KCM_ERR_TRANSACTION_ABORTED,
    ];

    for variant in variants {
        let msg = unsafe { KCM_ErrorMessage(variant) };
        assert!(!msg.is_null());
        let c_str = unsafe { std::ffi::CStr::from_ptr(msg) };
        let s = c_str.to_str().unwrap();
        assert!(!s.is_empty());
    }
}
