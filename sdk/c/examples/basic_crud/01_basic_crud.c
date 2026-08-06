/**
 * KCM C SDK — Basic CRUD Example.
 *
 * Demonstrates: insert, query, update, delete operations on facts.
 * Compile: gcc -Wall -Wextra -O2 -I../../include -o 01_basic_crud 01_basic_crud.c -lkcm
 */

#include "kcm.h"
#include <stdio.h>
#include <assert.h>

int main(void) {
    KCM_Database *db = NULL;
    KCM_Error rc;

    printf("=== KCM C SDK — Basic CRUD Example ===\n\n");

    rc = KCM_DatabaseNew(&db);
    assert(rc == KCM_OK);

    /* --- INSERT --- */
    printf("--- Insert Facts ---\n");
    KCM_Fact facts[] = {
        { .subject = 1, .predicate = 0, .object = 2, .confidence = 0.95,
          .evidence = 0, .timestamp = 0, .context = 0, .version = 1, .priority = 0, .owner = 0 },
        { .subject = 2, .predicate = 1, .object = 3, .confidence = 0.90,
          .evidence = 1, .timestamp = 0, .context = 1, .version = 1, .priority = 0, .owner = 1 },
        { .subject = 3, .predicate = 2, .object = 4, .confidence = 0.85,
          .evidence = 2, .timestamp = 0, .context = 2, .version = 1, .priority = 0, .owner = 2 },
        { .subject = 1, .predicate = 3, .object = 5, .confidence = 0.80,
          .evidence = 3, .timestamp = 0, .context = 2, .version = 1, .priority = -1, .owner = 7 },
    };
    for (int i = 0; i < 4; i++) {
        rc = KCM_DatabaseInsert(db, &facts[i]);
        assert(rc == KCM_OK);
    }
    printf("  Inserted 4 facts\n");
    printf("  Total: %lu, Active: %lu\n",
           (unsigned long)KCM_DatabaseFactCount(db),
           (unsigned long)KCM_DatabaseActiveCount(db));

    /* --- QUERY ALL --- */
    printf("\n--- Query All Facts ---\n");
    KCM_Query *q = KCM_DatabaseQuery(db, "SELECT * FROM facts");
    KCM_Fact *f;
    int count = 0;
    while ((f = KCM_QueryNext(q)) != NULL) {
        printf("  Subject: %u, Predicate: %u, Object: %u, Confidence: %.2f\n",
               f->subject, f->predicate, f->object, f->confidence);
        count++;
    }
    KCM_QueryFree(q);
    printf("  Returned %d facts\n", count);

    /* --- UPDATE --- */
    printf("\n--- Update Fact ---\n");
    KCM_Fact updated = { .subject = 10, .predicate = 0, .object = 20, .confidence = 0.99,
                         .evidence = 5, .timestamp = 0, .context = 3, .version = 2,
                         .priority = 2, .owner = 10 };
    rc = KCM_DatabaseUpdate(db, 0, &updated);
    assert(rc == KCM_OK);
    printf("  Updated row 0: subject changed to 10\n");

    /* --- DELETE --- */
    printf("\n--- Delete Fact ---\n");
    rc = KCM_DatabaseDelete(db, 3);
    assert(rc == KCM_OK);
    printf("  Deleted row 3\n");
    printf("  Total: %lu, Active: %lu\n",
           (unsigned long)KCM_DatabaseFactCount(db),
           (unsigned long)KCM_DatabaseActiveCount(db));

    /* --- VERIFY COUNTS --- */
    printf("\n--- Verify Counts ---\n");
    assert(KCM_DatabaseFactCount(db) == 4);
    assert(KCM_DatabaseActiveCount(db) == 3);
    printf("  Counts verified: 4 total, 3 active\n");

    KCM_DatabaseFree(db);
    printf("\n=== All operations completed ===\n");
    return 0;
}
