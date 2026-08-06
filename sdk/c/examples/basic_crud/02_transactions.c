/**
 * KCM C SDK — Transaction Example.
 *
 * Demonstrates: begin, commit, and rollback scenarios with transactions.
 * Compile: gcc -Wall -Wextra -O2 -I../../include -o 02_transactions 02_transactions.c -lkcm
 */

#include "kcm.h"
#include <stdio.h>
#include <assert.h>

int main(void) {
    KCM_Database *db = NULL;
    KCM_Error rc;

    printf("=== KCM C SDK — Transaction Example ===\n\n");

    rc = KCM_DatabaseNew(&db);
    assert(rc == KCM_OK);

    /* Insert baseline facts */
    KCM_Fact f1 = { .subject = 1, .predicate = 0, .object = 2, .confidence = 0.95 };
    KCM_Fact f2 = { .subject = 2, .predicate = 1, .object = 3, .confidence = 0.90 };
    KCM_DatabaseInsert(db, &f1);
    KCM_DatabaseInsert(db, &f2);
    printf("Initial: %lu active facts\n\n", (unsigned long)KCM_DatabaseActiveCount(db));

    /* --- COMMITTED TRANSACTION --- */
    printf("--- Committed Transaction ---\n");
    KCM_Transaction *txn1 = KCM_DatabaseBeginTransaction(db);
    KCM_Fact f3 = { .subject = 3, .predicate = 2, .object = 4, .confidence = 0.85,
                    .evidence = 2, .context = 2, .version = 1, .owner = 2 };
    rc = KCM_DatabaseInsert(db, &f3);
    assert(rc == KCM_OK);
    printf("  Inserted fact in transaction\n");
    rc = KCM_TransactionCommit(txn1);
    assert(rc == KCM_OK);
    KCM_TransactionFree(txn1);
    printf("  Committed transaction\n");
    printf("  After commit: %lu active facts\n", (unsigned long)KCM_DatabaseActiveCount(db));
    assert(KCM_DatabaseActiveCount(db) == 3);

    /* --- ROLLED BACK TRANSACTION --- */
    printf("\n--- Rolled Back Transaction ---\n");
    KCM_Transaction *txn2 = KCM_DatabaseBeginTransaction(db);
    KCM_Fact f4 = { .subject = 4, .predicate = 3, .object = 5, .confidence = 0.80,
                    .evidence = 3, .context = 2, .version = 1, .owner = 3 };
    rc = KCM_DatabaseInsert(db, &f4);
    assert(rc == KCM_OK);
    printf("  Inserted fact in transaction\n");
    rc = KCM_TransactionRollback(txn2);
    assert(rc == KCM_OK);
    KCM_TransactionFree(txn2);
    printf("  Rolled back transaction\n");
    printf("  After rollback: %lu active facts\n", (unsigned long)KCM_DatabaseActiveCount(db));
    assert(KCM_DatabaseActiveCount(db) == 3);

    /* --- MULTIPLE OPERATIONS --- */
    printf("\n--- Multiple Operations in Transaction ---\n");
    KCM_Transaction *txn3 = KCM_DatabaseBeginTransaction(db);
    KCM_Fact fs[] = {
        { .subject = 10, .predicate = 0, .object = 20, .confidence = 0.99 },
        { .subject = 30, .predicate = 1, .object = 40, .confidence = 0.88 },
        { .subject = 50, .predicate = 2, .object = 60, .confidence = 0.77 },
    };
    for (int i = 0; i < 3; i++) {
        rc = KCM_DatabaseInsert(db, &fs[i]);
        assert(rc == KCM_OK);
    }
    printf("  3 pending operations\n");
    rc = KCM_TransactionCommit(txn3);
    assert(rc == KCM_OK);
    KCM_TransactionFree(txn3);
    printf("  After commit: %lu active facts\n", (unsigned long)KCM_DatabaseActiveCount(db));

    KCM_DatabaseFree(db);
    printf("\n=== All transaction operations completed ===\n");
    return 0;
}
