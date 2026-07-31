#![allow(non_camel_case_types)]

pub mod examples;
pub mod kql_parser;
pub mod python;
pub mod rest_api;

use kcm_core::types::*;
use kcm_runtime::database::KnowledgeDatabase;
use parking_lot::Mutex;
use std::os::raw::c_char;
use std::sync::Arc;

pub struct KCM_Database {
    inner: Arc<Mutex<KnowledgeDatabase>>,
}

use kcm_runtime::transaction::Transaction;

pub struct KCM_Transaction {
    inner: Option<Transaction>,
}

pub struct KCM_Query {
    inner: Vec<Fact>,
    position: usize,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

/// Create a new KCM database.
///
/// # Safety
/// - `db_out` must be a valid pointer to a `*mut KCM_Database` slot.
/// - Caller must free the returned pointer with `KCM_DatabaseFree`.
#[no_mangle]
pub unsafe extern "C" fn KCM_DatabaseNew(db_out: *mut *mut KCM_Database) -> KCM_Error {
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

/// Free a KCM database and all associated resources.
///
/// # Safety
/// - `db` must be a valid pointer previously returned by `KCM_DatabaseNew`.
/// - `db` must not be used after this call (use-after-free).
#[no_mangle]
pub unsafe extern "C" fn KCM_DatabaseFree(db: *mut KCM_Database) {
    if !db.is_null() {
        unsafe {
            drop(Box::from_raw(db));
        }
    }
}

/// Insert a fact into the database.
///
/// # Safety
/// - `db` must be a valid pointer previously returned by `KCM_DatabaseNew`.
/// - `fact` must be a valid pointer to a `KCM_Fact`.
#[no_mangle]
pub unsafe extern "C" fn KCM_DatabaseInsert(
    db: *mut KCM_Database,
    fact: *const KCM_Fact,
) -> KCM_Error {
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

/// Update an existing fact in the database.
///
/// # Safety
/// - `db` must be a valid pointer previously returned by `KCM_DatabaseNew`.
/// - `fact` must be a valid pointer to a `KCM_Fact`.
#[no_mangle]
pub unsafe extern "C" fn KCM_DatabaseUpdate(
    db: *mut KCM_Database,
    row_id: u64,
    fact: *const KCM_Fact,
) -> KCM_Error {
    if db.is_null() || fact.is_null() {
        return KCM_Error::KCM_ERR_INVALID_ARGUMENT;
    }
    unsafe {
        let db = &*db;
        let fact_ref = &*fact;
        let kcm_fact = Fact::from(fact_ref);
        match db.inner.lock().update(RowID(row_id), &kcm_fact) {
            Ok(_) => KCM_Error::KCM_OK,
            Err(e) => e.into(),
        }
    }
}

/// Delete a fact from the database by row ID.
///
/// # Safety
/// - `db` must be a valid pointer previously returned by `KCM_DatabaseNew`.
#[no_mangle]
pub unsafe extern "C" fn KCM_DatabaseDelete(db: *mut KCM_Database, row_id: u64) -> KCM_Error {
    if db.is_null() {
        return KCM_Error::KCM_ERR_INVALID_ARGUMENT;
    }
    unsafe {
        let db = &*db;
        match db.inner.lock().delete(RowID(row_id)) {
            Ok(_) => KCM_Error::KCM_OK,
            Err(e) => e.into(),
        }
    }
}

/// Get the total fact count (including deleted).
///
/// # Safety
/// - `db` must be a valid pointer previously returned by `KCM_DatabaseNew`.
#[no_mangle]
pub unsafe extern "C" fn KCM_DatabaseFactCount(db: *mut KCM_Database) -> u64 {
    if db.is_null() {
        return 0;
    }
    unsafe { (*db).inner.lock().fact_count() as u64 }
}

/// Get the active fact count (excluding deleted).
///
/// # Safety
/// - `db` must be a valid pointer previously returned by `KCM_DatabaseNew`.
#[no_mangle]
pub unsafe extern "C" fn KCM_DatabaseActiveCount(db: *mut KCM_Database) -> u64 {
    if db.is_null() {
        return 0;
    }
    unsafe { (*db).inner.lock().active_fact_count() as u64 }
}

/// Execute a query returning all facts.
///
/// # Safety
/// - `db` must be a valid pointer previously returned by `KCM_DatabaseNew`.
/// - `query_out` must be a valid pointer to a `*mut KCM_Query` slot.
/// - Caller must free the returned query with `KCM_QueryFree`.
#[no_mangle]
pub unsafe extern "C" fn KCM_DatabaseQuery(
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

/// Get the next fact from a query result.
///
/// # Safety
/// - `query` must be a valid pointer previously returned by `KCM_DatabaseQuery`.
/// - `fact_out` must be a valid pointer to a `KCM_Fact` slot.
/// - `has_next` must be a valid pointer to a `bool` slot.
#[no_mangle]
pub unsafe extern "C" fn KCM_QueryNext(
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

/// Free a query result.
///
/// # Safety
/// - `query` must be a valid pointer previously returned by `KCM_DatabaseQuery`.
/// - `query` must not be used after this call.
#[no_mangle]
pub unsafe extern "C" fn KCM_QueryFree(query: *mut KCM_Query) {
    if !query.is_null() {
        unsafe {
            drop(Box::from_raw(query));
        }
    }
}

/// Begin a new transaction.
///
/// # Safety
/// - `db` must be a valid pointer previously returned by `KCM_DatabaseNew`.
/// - `txn_out` must be a valid pointer to a `*mut KCM_Transaction` slot.
/// - Caller must free the returned transaction with `KCM_TransactionFree`.
#[no_mangle]
pub unsafe extern "C" fn KCM_DatabaseBeginTransaction(
    db: *mut KCM_Database,
    txn_out: *mut *mut KCM_Transaction,
) -> KCM_Error {
    if db.is_null() || txn_out.is_null() {
        return KCM_Error::KCM_ERR_INVALID_ARGUMENT;
    }
    unsafe {
        let db_ref = &*db;
        let txn = db_ref.inner.lock().begin_transaction();
        *txn_out = Box::into_raw(Box::new(KCM_Transaction { inner: Some(txn) }));
        KCM_Error::KCM_OK
    }
}

/// Free a transaction handle.
///
/// # Safety
/// - `txn` must be a valid pointer previously returned by `KCM_DatabaseBeginTransaction`.
/// - `txn` must not be used after this call.
#[no_mangle]
pub unsafe extern "C" fn KCM_TransactionFree(txn: *mut KCM_Transaction) {
    if !txn.is_null() {
        unsafe {
            drop(Box::from_raw(txn));
        }
    }
}

/// Commit a transaction, applying all buffered changes to the database.
///
/// # Safety
/// - `txn` must be a valid pointer previously returned by `KCM_DatabaseBeginTransaction`.
/// - `txn` must not be used after this call (it is consumed).
/// - `db` must be the same database pointer used to begin the transaction.
#[no_mangle]
pub unsafe extern "C" fn KCM_TransactionCommit(
    txn: *mut KCM_Transaction,
    db: *mut KCM_Database,
) -> KCM_Error {
    if txn.is_null() || db.is_null() {
        return KCM_Error::KCM_ERR_INVALID_ARGUMENT;
    }
    unsafe {
        let txn_ref = &mut *txn;
        if let Some(inner_txn) = txn_ref.inner.take() {
            let db_guard = (*db).inner.lock();
            let mut schema = db_guard.get_schema_mut();
            if let Err(e) = inner_txn.apply_to_schema(&mut schema) {
                return e.into();
            }
            drop(schema);
            drop(db_guard);
            match inner_txn.commit() {
                Ok(()) => KCM_Error::KCM_OK,
                Err(e) => e.into(),
            }
        } else {
            KCM_Error::KCM_ERR_TRANSACTION_ABORTED
        }
    }
}

/// Rollback a transaction, discarding all buffered changes.
///
/// # Safety
/// - `txn` must be a valid pointer previously returned by `KCM_DatabaseBeginTransaction`.
/// - `txn` must not be used after this call (it is consumed).
#[no_mangle]
pub unsafe extern "C" fn KCM_TransactionRollback(txn: *mut KCM_Transaction) -> KCM_Error {
    if txn.is_null() {
        return KCM_Error::KCM_ERR_INVALID_ARGUMENT;
    }
    unsafe {
        let txn_ref = &mut *txn;
        if let Some(inner_txn) = txn_ref.inner.take() {
            match inner_txn.rollback() {
                Ok(()) => KCM_Error::KCM_OK,
                Err(e) => e.into(),
            }
        } else {
            KCM_Error::KCM_ERR_TRANSACTION_ABORTED
        }
    }
}

/// Get error message string for an error code.
///
/// # Safety
/// - Returns a pointer to a static null-terminated string.
/// - The string is valid for the lifetime of the program.
/// - Caller must not free or modify the returned pointer.
#[no_mangle]
pub unsafe extern "C" fn KCM_ErrorMessage(err: KCM_Error) -> *const c_char {
    match err {
        KCM_Error::KCM_OK => c"OK".as_ptr(),
        KCM_Error::KCM_ERR_NOT_FOUND => c"Not found".as_ptr(),
        KCM_Error::KCM_ERR_OUT_OF_MEMORY => c"Out of memory".as_ptr(),
        KCM_Error::KCM_ERR_INVALID_ARGUMENT => c"Invalid argument".as_ptr(),
        KCM_Error::KCM_ERR_IO => c"I/O error".as_ptr(),
        KCM_Error::KCM_ERR_CORRUPTED => c"Data corrupted".as_ptr(),
        KCM_Error::KCM_ERR_CONFLICT => c"Conflict".as_ptr(),
        KCM_Error::KCM_ERR_TRANSACTION_ABORTED => c"Transaction aborted".as_ptr(),
    }
}
