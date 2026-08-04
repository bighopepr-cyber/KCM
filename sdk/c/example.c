/**
 * KCM C SDK Example
 * 
 * Demonstrates: create database, insert facts, query, transaction, verify.
 * Compile: gcc -o kcm_example example.c -lkcm
 */
#include "kcm.h"
#include <stdio.h>
#include <assert.h>

int main(void) {
    KCM_Database *db = NULL;
    KCM_Error rc;

    printf("=== KCM C SDK Example ===\n\n");

    // Create database
    rc = KCM_DatabaseNew(&db);
    assert(rc == KCM_OK);
    printf("Created database\n");

    // Insert facts
    KCM_Fact facts[] = {
        { .subject = 1, .predicate = 0, .object = 2, .confidence = 0.95 },
        { .subject = 2, .predicate = 1, .object = 3, .confidence = 0.90 },
        { .subject = 3, .predicate = 2, .object = 4, .confidence = 0.85 },
    };
    for (int i = 0; i < 3; i++) {
        rc = KCM_DatabaseInsert(db, &facts[i]);
        assert(rc == KCM_OK);
    }
    printf("Inserted 3 facts\n");

    // Query
    uint64_t count = KCM_DatabaseFactCount(db);
    printf("Fact count: %lu\n", (unsigned long)count);
    assert(count == 3);

    // Active count
    uint64_t active = KCM_DatabaseActiveCount(db);
    printf("Active: %lu\n", (unsigned long)active);
    assert(active == 3);

    // Transaction
    KCM_Transaction *txn = KCM_DatabaseBeginTransaction(db);
    KCM_Fact new_fact = { .subject = 4, .predicate = 3, .object = 5, .confidence = 0.80 };
    rc = KCM_DatabaseInsert(db, &new_fact);
    assert(rc == KCM_OK);
    rc = KCM_TransactionCommit(txn);
    assert(rc == KCM_OK);
    printf("Committed transaction\n");

    // Delete
    rc = KCM_DatabaseDelete(db, 0);
    assert(rc == KCM_OK);
    printf("Deleted row 0\n");

    // Verify counts
    printf("Fact count: %lu\n", (unsigned long)KCM_DatabaseFactCount(db));
    printf("Active: %lu\n", (unsigned long)KCM_DatabaseActiveCount(db));

    // Error handling
    const char *msg = KCM_ErrorMessage(KCM_ERR_NOT_FOUND);
    printf("Error message: %s\n", msg);

    // Cleanup
    KCM_DatabaseFree(db);
    printf("\nAll C SDK tests passed!\n");
    return 0;
}
