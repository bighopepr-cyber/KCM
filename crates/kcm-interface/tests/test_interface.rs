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
