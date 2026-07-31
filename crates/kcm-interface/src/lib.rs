#![allow(non_camel_case_types)]
#![allow(clippy::not_unsafe_ptr_arg_deref)]

pub mod python;

use kcm_core::types::*;
use kcm_runtime::database::KnowledgeDatabase;
use parking_lot::Mutex;
use std::os::raw::c_char;
use std::sync::Arc;

pub struct KCM_Database {
    inner: Arc<Mutex<KnowledgeDatabase>>,
}

#[allow(dead_code)]
pub struct KCM_Transaction {
    inner: Arc<Mutex<kcm_runtime::transaction::Transaction>>,
}

pub struct KCM_Query {
    inner: Vec<Fact>,
    position: usize,
}

#[repr(C)]
pub struct KCM_Fact {
    pub subject: u32,
    pub predicate: u8,
    pub object: u32,
    pub confidence: f64,
    pub evidence: u8,
    pub timestamp: i64,
    pub context: u8,
}

impl From<&Fact> for KCM_Fact {
    fn from(fact: &Fact) -> Self {
        KCM_Fact {
            subject: fact.subject.0,
            predicate: fact.predicate.0,
            object: fact.object.0,
            confidence: fact.confidence,
            evidence: fact.evidence.0,
            timestamp: fact.timestamp,
            context: fact.context.0,
        }
    }
}

impl From<&KCM_Fact> for Fact {
    fn from(kcm_fact: &KCM_Fact) -> Self {
        Fact {
            subject: SubjectID(kcm_fact.subject),
            predicate: PredicateID(kcm_fact.predicate),
            object: ObjectID(kcm_fact.object),
            confidence: kcm_fact.confidence,
            evidence: EvidenceID(kcm_fact.evidence),
            timestamp: kcm_fact.timestamp,
            context: ContextID(kcm_fact.context),
            version: 1,
            priority: 0,
            owner: 0,
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum KCM_Error {
    KCM_OK = 0,
    KCM_ERR_NOT_FOUND = 1,
    KCM_ERR_OUT_OF_MEMORY = 2,
    KCM_ERR_INVALID_ARGUMENT = 3,
    KCM_ERR_IO = 4,
    KCM_ERR_CORRUPTED = 5,
    KCM_ERR_CONFLICT = 6,
    KCM_ERR_TRANSACTION_ABORTED = 7,
}

impl From<kcm_core::types::KcmError> for KCM_Error {
    fn from(err: kcm_core::types::KcmError) -> Self {
        match err {
            kcm_core::types::KcmError::NotFound(_) => KCM_Error::KCM_ERR_NOT_FOUND,
            kcm_core::types::KcmError::OutOfMemory => KCM_Error::KCM_ERR_OUT_OF_MEMORY,
            kcm_core::types::KcmError::InvalidArgument(_) => KCM_Error::KCM_ERR_INVALID_ARGUMENT,
            kcm_core::types::KcmError::Io(_) => KCM_Error::KCM_ERR_IO,
            kcm_core::types::KcmError::Corrupted(_) => KCM_Error::KCM_ERR_CORRUPTED,
            kcm_core::types::KcmError::Conflict(_) => KCM_Error::KCM_ERR_CONFLICT,
            kcm_core::types::KcmError::TransactionAborted => KCM_Error::KCM_ERR_TRANSACTION_ABORTED,
        }
    }
}

#[no_mangle]
pub extern "C" fn KCM_DatabaseNew(db_out: *mut *mut KCM_Database) -> KCM_Error {
    if db_out.is_null() {
        return KCM_Error::KCM_ERR_INVALID_ARGUMENT;
    }

    match KnowledgeDatabase::new() {
        Ok(kb) => {
            unsafe {
                *db_out = Box::into_raw(Box::new(KCM_Database {
                    inner: Arc::new(Mutex::new(kb)),
                }));
            }
            KCM_Error::KCM_OK
        }
        Err(e) => e.into(),
    }
}

#[no_mangle]
pub extern "C" fn KCM_DatabaseFree(db: *mut KCM_Database) {
    if !db.is_null() {
        unsafe {
            drop(Box::from_raw(db));
        }
    }
}

#[no_mangle]
pub extern "C" fn KCM_DatabaseInsert(db: *mut KCM_Database, fact: *const KCM_Fact) -> KCM_Error {
    if db.is_null() || fact.is_null() {
        return KCM_Error::KCM_ERR_INVALID_ARGUMENT;
    }

    unsafe {
        let db = &*db;
        let fact_ref = &*fact;
        let kcm_fact = Fact::from(fact_ref);

        match db.inner.lock().insert(&kcm_fact) {
            Ok(_) => KCM_Error::KCM_OK,
            Err(e) => e.into(),
        }
    }
}

#[no_mangle]
pub extern "C" fn KCM_DatabaseQuery(
    db: *mut KCM_Database,
    query_out: *mut *mut KCM_Query,
) -> KCM_Error {
    if db.is_null() || query_out.is_null() {
        return KCM_Error::KCM_ERR_INVALID_ARGUMENT;
    }

    unsafe {
        let db = &*db;
        let kb = db.inner.lock();

        match kb.query().execute() {
            Ok(facts) => {
                *query_out = Box::into_raw(Box::new(KCM_Query {
                    inner: facts,
                    position: 0,
                }));
                KCM_Error::KCM_OK
            }
            Err(e) => e.into(),
        }
    }
}

#[no_mangle]
pub extern "C" fn KCM_QueryNext(
    query: *mut KCM_Query,
    fact_out: *mut KCM_Fact,
    has_next: *mut bool,
) -> KCM_Error {
    if query.is_null() || fact_out.is_null() || has_next.is_null() {
        return KCM_Error::KCM_ERR_INVALID_ARGUMENT;
    }

    unsafe {
        let query_ref = &mut *query;

        if query_ref.position < query_ref.inner.len() {
            let fact = &query_ref.inner[query_ref.position];
            *fact_out = KCM_Fact::from(fact);
            query_ref.position += 1;
            *has_next = query_ref.position < query_ref.inner.len();
            KCM_Error::KCM_OK
        } else {
            *has_next = false;
            KCM_Error::KCM_OK
        }
    }
}

#[no_mangle]
pub extern "C" fn KCM_QueryFree(query: *mut KCM_Query) {
    if !query.is_null() {
        unsafe {
            drop(Box::from_raw(query));
        }
    }
}

#[no_mangle]
pub extern "C" fn KCM_ErrorMessage(err: KCM_Error) -> *const c_char {
    let msg = match err {
        KCM_Error::KCM_OK => "OK",
        KCM_Error::KCM_ERR_NOT_FOUND => "Not found",
        KCM_Error::KCM_ERR_OUT_OF_MEMORY => "Out of memory",
        KCM_Error::KCM_ERR_INVALID_ARGUMENT => "Invalid argument",
        KCM_Error::KCM_ERR_IO => "I/O error",
        KCM_Error::KCM_ERR_CORRUPTED => "Data corrupted",
        KCM_Error::KCM_ERR_CONFLICT => "Conflict",
        KCM_Error::KCM_ERR_TRANSACTION_ABORTED => "Transaction aborted",
    };

    msg.as_ptr() as *const c_char
}
