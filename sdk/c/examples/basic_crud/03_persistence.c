/**
 * KCM C SDK — Persistence Example.
 *
 * Demonstrates: save, load, and verify database persistence.
 * Compile: gcc -Wall -Wextra -O2 -I../../include -o 03_persistence 03_persistence.c -lkcm
 */

#include "kcm.h"
#include <stdio.h>
#include <assert.h>
#include <stdlib.h>
#include <string.h>

int main(void) {
    KCM_Database *db = NULL;
    KCM_Error rc;

    printf("=== KCM C SDK — Persistence Example ===\n\n");

    const char *path = "/tmp/kcm_c_example.kcm";

    rc = KCM_DatabaseNew(&db);
    assert(rc == KCM_OK);

    /* --- SAVE DATABASE --- */
    printf("--- Save Database ---\n");
    KCM_Fact facts[] = {
        { .subject = 1, .predicate = 0, .object = 2, .confidence = 0.95,
          .evidence = 1, .context = 1, .version = 1, .owner = 1 },
        { .subject = 2, .predicate = 1, .object = 3, .confidence = 0.90,
          .evidence = 2, .context = 1, .version = 1, .owner = 2 },
        { .subject = 3, .predicate = 2, .object = 4, .confidence = 0.85,
          .evidence = 3, .context = 2, .version = 1, .owner = 3 },
    };
    for (int i = 0; i < 3; i++) {
        rc = KCM_DatabaseInsert(db, &facts[i]);
        assert(rc == KCM_OK);
    }
    printf("  Facts before save: %lu total, %lu active\n",
           (unsigned long)KCM_DatabaseFactCount(db),
           (unsigned long)KCM_DatabaseActiveCount(db));
    rc = KCM_DatabaseSave(db, path);
    assert(rc == KCM_OK);
    printf("  Saved to %s\n", path);

    /* --- VERIFY FILE --- */
    printf("\n--- Verify Database File ---\n");
    rc = KCM_DatabaseVerify(path);
    assert(rc == KCM_OK);
    printf("  Verification passed\n");

    /* --- LOAD INTO NEW DATABASE --- */
    printf("\n--- Load Into New Database ---\n");
    KCM_Database *db2 = NULL;
    rc = KCM_DatabaseNew(&db2);
    assert(rc == KCM_OK);
    rc = KCM_DatabaseLoad(db2, path);
    assert(rc == KCM_OK);
    printf("  Loaded: %lu total, %lu active\n",
           (unsigned long)KCM_DatabaseFactCount(db2),
           (unsigned long)KCM_DatabaseActiveCount(db2));
    assert(KCM_DatabaseFactCount(db2) == 3);
    assert(KCM_DatabaseActiveCount(db2) == 3);

    /* --- VERIFY DATA INTEGRITY --- */
    printf("\n--- Verify Data Integrity ---\n");
    KCM_Query *q = KCM_DatabaseQuery(db2, "SELECT * FROM facts");
    KCM_Fact *f;
    int count = 0;
    while ((f = KCM_QueryNext(q)) != NULL) {
        printf("  Subject: %u, Predicate: %u, Object: %u, Confidence: %.2f\n",
               f->subject, f->predicate, f->object, f->confidence);
        count++;
    }
    KCM_QueryFree(q);
    assert(count == 3);

    /* --- SAVE-LOAD ROUND TRIP --- */
    printf("\n--- Save-Load Round Trip ---\n");
    KCM_Fact new_fact = { .subject = 10, .predicate = 0, .object = 20, .confidence = 0.99 };
    rc = KCM_DatabaseInsert(db2, &new_fact);
    assert(rc == KCM_OK);
    rc = KCM_DatabaseSave(db2, path);
    assert(rc == KCM_OK);
    KCM_DatabaseFree(db2);

    KCM_Database *db3 = NULL;
    rc = KCM_DatabaseNew(&db3);
    assert(rc == KCM_OK);
    rc = KCM_DatabaseLoad(db3, path);
    assert(rc == KCM_OK);
    printf("  Round-trip: %lu total, %lu active\n",
           (unsigned long)KCM_DatabaseFactCount(db3),
           (unsigned long)KCM_DatabaseActiveCount(db3));
    assert(KCM_DatabaseFactCount(db3) == 4);
    assert(KCM_DatabaseActiveCount(db3) == 4);

    KCM_DatabaseFree(db3);
    KCM_DatabaseFree(db);
    remove(path);
    printf("\n=== All persistence operations completed ===\n");
    return 0;
}
