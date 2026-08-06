/**
 * KCM Knowledge Columnar Model - C SDK Header
 * 
 * This header wraps the 18 FFI functions implemented in kcm-interface.
 * Link against libkcm (produced by building kcm-interface with --release).
 */

#ifndef KCM_H
#define KCM_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

/** Error codes returned by KCM functions. */
typedef enum {
    KCM_OK = 0,
    KCM_ERR_NOT_FOUND = 1,
    KCM_ERR_OUT_OF_MEMORY = 2,
    KCM_ERR_INVALID_ARGUMENT = 3,
    KCM_ERR_IO = 4,
    KCM_ERR_CORRUPTED = 5,
    KCM_ERR_CONFLICT = 6,
    KCM_ERR_TRANSACTION_ABORTED = 7,
} KCM_Error;

/** Opaque handle to a KCM database. */
typedef struct KCM_Database KCM_Database;

/** Opaque handle to a KCM query result. */
typedef struct KCM_Query KCM_Query;

/** Opaque handle to a KCM transaction. */
typedef struct KCM_Transaction KCM_Transaction;

/** A knowledge fact with 10 attributes. */
typedef struct {
    uint32_t subject;
    uint8_t  predicate;
    uint32_t object;
    double   confidence;
    uint8_t  evidence;
    int64_t  timestamp;
    uint8_t  context;
    int32_t  version;
    int8_t   priority;
    uint16_t owner;
} KCM_Fact;

/**
 * Create a new in-memory KCM database.
 * @param db_out  Output pointer to the created database.
 * @return KCM_OK on success.
 */
KCM_Error KCM_DatabaseNew(KCM_Database **db_out);

/**
 * Free a KCM database and release all resources.
 * @param db  Database handle. Safe to call with NULL.
 */
void KCM_DatabaseFree(KCM_Database *db);

/**
 * Insert a fact into the database.
 * @param db    Database handle.
 * @param fact  Pointer to the fact to insert.
 * @return KCM_OK on success.
 */
KCM_Error KCM_DatabaseInsert(KCM_Database *db, const KCM_Fact *fact);

/**
 * Update an existing fact by row ID.
 * @param db      Database handle.
 * @param row_id  Row ID of the fact to update.
 * @param fact    New fact data.
 * @return KCM_OK on success.
 */
KCM_Error KCM_DatabaseUpdate(KCM_Database *db, uint64_t row_id, const KCM_Fact *fact);

/**
 * Delete a fact by row ID.
 * @param db      Database handle.
 * @param row_id  Row ID of the fact to delete.
 * @return KCM_OK on success.
 */
KCM_Error KCM_DatabaseDelete(KCM_Database *db, uint64_t row_id);

/**
 * Get total fact count (including deleted).
 * @param db  Database handle.
 * @return Total fact count.
 */
uint64_t KCM_DatabaseFactCount(KCM_Database *db);

/**
 * Get active (non-deleted) fact count.
 * @param db  Database handle.
 * @return Active fact count.
 */
uint64_t KCM_DatabaseActiveCount(KCM_Database *db);

/**
 * Execute a KQL query and return results.
 * @param db       Database handle.
 * @param query    Null-terminated KQL query string.
 * @return Query handle (must be freed with KCM_QueryFree).
 */
KCM_Query *KCM_DatabaseQuery(KCM_Database *db, const char *query);

/**
 * Get the next fact from a query result.
 * @param query  Query handle.
 * @return Next fact, or NULL if no more results.
 */
KCM_Fact *KCM_QueryNext(KCM_Query *query);

/**
 * Free a query result handle.
 * @param query  Query handle. Safe to call with NULL.
 */
void KCM_QueryFree(KCM_Query *query);

/**
 * Begin a new transaction.
 * @param db    Database handle.
 * @return Transaction handle.
 */
KCM_Transaction *KCM_DatabaseBeginTransaction(KCM_Database *db);

/**
 * Commit a transaction.
 * @param txn  Transaction handle.
 * @return KCM_OK on success.
 */
KCM_Error KCM_TransactionCommit(KCM_Transaction *txn);

/**
 * Rollback a transaction.
 * @param txn  Transaction handle.
 * @return KCM_OK on success.
 */
KCM_Error KCM_TransactionRollback(KCM_Transaction *txn);

/**
 * Free a transaction handle.
 * @param txn  Transaction handle. Safe to call with NULL.
 */
void KCM_TransactionFree(KCM_Transaction *txn);

/**
 * Save the database to a file.
 * @param db    Database handle.
 * @param path  File path.
 * @return KCM_OK on success.
 */
KCM_Error KCM_DatabaseSave(KCM_Database *db, const char *path);

/**
 * Load a database from a file.
 * @param db    Database handle.
 * @param path  File path.
 * @return KCM_OK on success.
 */
KCM_Error KCM_DatabaseLoad(KCM_Database *db, const char *path);

/**
 * Verify database integrity.
 * @param path  File path to verify.
 * @return KCM_OK if valid.
 */
KCM_Error KCM_DatabaseVerify(const char *path);

/**
 * Get human-readable error message.
 * @param err  Error code.
 * @return Static string describing the error.
 */
const char *KCM_ErrorMessage(KCM_Error err);

#ifdef __cplusplus
}
#endif

#endif /* KCM_H */
