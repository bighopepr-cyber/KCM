/**
 * KCM C SDK — Error Handling Example.
 *
 * Demonstrates: proper error handling patterns with KCM_Error codes.
 * Compile: gcc -Wall -Wextra -O2 -I../../include -o 05_error_handling 05_error_handling.c -lkcm
 */

#include "kcm.h"
#include <stdio.h>
#include <assert.h>

int main(void) {
    KCM_Database *db = NULL;
    KCM_Error rc;

    printf("=== KCM C SDK — Error Handling Example ===\n\n");

    rc = KCM_DatabaseNew(&db);
    assert(rc == KCM_OK);

    /* --- NOT FOUND (update non-existent row) --- */
    printf("--- Not Found (update non-existent row) ---\n");
    KCM_Fact bad_fact = { .subject = 1, .predicate = 0, .object = 2, .confidence = 0.5 };
    rc = KCM_DatabaseUpdate(db, 99999, &bad_fact);
    printf("  Error code: %d\n", rc);
    printf("  Error message: %s\n", KCM_ErrorMessage(rc));
    assert(rc == KCM_ERR_NOT_FOUND);

    /* --- NOT FOUND (delete non-existent row) --- */
    printf("\n--- Not Found (delete non-existent row) ---\n");
    rc = KCM_DatabaseDelete(db, 99999);
    printf("  Error code: %d\n", rc);
    printf("  Error message: %s\n", KCM_ErrorMessage(rc));

    /* --- ALL ERROR CODES --- */
    printf("\n--- All Error Codes ---\n");
    KCM_Error codes[] = {
        KCM_OK, KCM_ERR_NOT_FOUND, KCM_ERR_OUT_OF_MEMORY,
        KCM_ERR_INVALID_ARGUMENT, KCM_ERR_IO, KCM_ERR_CORRUPTED,
        KCM_ERR_CONFLICT, KCM_ERR_TRANSACTION_ABORTED
    };
    const char *names[] = {
        "OK", "NOT_FOUND", "OUT_OF_MEMORY", "INVALID_ARGUMENT",
        "IO", "CORRUPTED", "CONFLICT", "TRANSACTION_ABORTED"
    };
    for (int i = 0; i < 8; i++) {
        printf("  %s (%d): %s\n", names[i], codes[i], KCM_ErrorMessage(codes[i]));
    }

    /* --- FILE NOT FOUND (save to bad path) --- */
    printf("\n--- Save to Invalid Path ---\n");
    rc = KCM_DatabaseSave(db, "/nonexistent/dir/db.kcm");
    printf("  Error code: %d\n", rc);
    printf("  Error message: %s\n", KCM_ErrorMessage(rc));

    /* --- VERIFY NON-EXISTENT FILE --- */
    printf("\n--- Verify Non-Existent File ---\n");
    rc = KCM_DatabaseVerify("/nonexistent/path/db.kcm");
    printf("  Error code: %d\n", rc);
    printf("  Error message: %s\n", KCM_ErrorMessage(rc));

    /* --- NULL POINTER SAFETY --- */
    printf("\n--- NULL Pointer Safety ---\n");
    KCM_DatabaseFree(NULL);
    KCM_QueryFree(NULL);
    KCM_TransactionFree(NULL);
    printf("  Freeing NULL handles is safe\n");

    /* --- SUCCESS PATTERN --- */
    printf("\n--- Success Pattern ---\n");
    rc = KCM_DatabaseInsert(db, &(KCM_Fact){ .subject = 1, .predicate = 0, .object = 2, .confidence = 0.95 });
    if (rc != KCM_OK) {
        printf("  Database error: %s\n", KCM_ErrorMessage(rc));
    } else {
        printf("  Insert succeeded\n");
    }

    /* --- QUERY ITERATION --- */
    printf("\n--- Query Iteration ---\n");
    KCM_Query *q = KCM_DatabaseQuery(db, "SELECT * FROM facts");
    KCM_Fact *f;
    int count = 0;
    while ((f = KCM_QueryNext(q)) != NULL) {
        count++;
    }
    KCM_QueryFree(q);
    printf("  Query returned %d results\n", count);

    KCM_DatabaseFree(db);
    printf("\n=== All error handling patterns completed ===\n");
    return 0;
}
