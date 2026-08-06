/**
 * KCM C SDK — Query Patterns Example.
 *
 * Demonstrates: different KQL query patterns and filtering options.
 * Compile: gcc -Wall -Wextra -O2 -I../../include -o 04_query_patterns 04_query_patterns.c -lkcm
 */

#include "kcm.h"
#include <stdio.h>
#include <assert.h>

int main(void) {
    KCM_Database *db = NULL;
    KCM_Error rc;

    printf("=== KCM C SDK — Query Patterns Example ===\n\n");

    rc = KCM_DatabaseNew(&db);
    assert(rc == KCM_OK);

    /* Insert test data */
    KCM_Fact facts[] = {
        { .subject = 1, .predicate = 0, .object = 2, .confidence = 0.95,
          .evidence = 1, .context = 1, .version = 1, .owner = 1 },
        { .subject = 2, .predicate = 1, .object = 3, .confidence = 0.90,
          .evidence = 2, .context = 1, .version = 1, .owner = 2 },
        { .subject = 3, .predicate = 2, .object = 4, .confidence = 0.85,
          .evidence = 3, .context = 2, .version = 1, .owner = 3 },
        { .subject = 1, .predicate = 3, .object = 5, .confidence = 0.80,
          .evidence = 1, .context = 2, .version = 1, .owner = 1 },
        { .subject = 4, .predicate = 0, .object = 6, .confidence = 0.75,
          .evidence = 2, .context = 1, .version = 1, .owner = 2 },
    };
    for (int i = 0; i < 5; i++) {
        rc = KCM_DatabaseInsert(db, &facts[i]);
        assert(rc == KCM_OK);
    }
    printf("Inserted 5 facts\n\n");

    /* --- SELECT ALL --- */
    printf("--- SELECT * FROM facts ---\n");
    KCM_Query *q = KCM_DatabaseQuery(db, "SELECT * FROM facts");
    int count = 0;
    while (KCM_QueryNext(q) != NULL) count++;
    KCM_QueryFree(q);
    printf("  Returned %d facts\n", count);

    /* --- FILTER BY SUBJECT --- */
    printf("\n--- Filter by Subject = 1 ---\n");
    q = KCM_DatabaseQuery(db, "SELECT * FROM facts");
    int subject_count = 0;
    KCM_Fact *f;
    while ((f = KCM_QueryNext(q)) != NULL) {
        if (f->subject == 1) {
            printf("  Subject: %u, Predicate: %u, Object: %u\n",
                   f->subject, f->predicate, f->object);
            subject_count++;
        }
    }
    KCM_QueryFree(q);
    printf("  Found %d facts with subject=1\n", subject_count);

    /* --- FILTER BY PREDICATE --- */
    printf("\n--- Filter by Predicate = 0 ---\n");
    q = KCM_DatabaseQuery(db, "SELECT * FROM facts");
    int pred_count = 0;
    while ((f = KCM_QueryNext(q)) != NULL) {
        if (f->predicate == 0) {
            printf("  Subject: %u, Predicate: %u, Object: %u\n",
                   f->subject, f->predicate, f->object);
            pred_count++;
        }
    }
    KCM_QueryFree(q);
    printf("  Found %d facts with predicate=0\n", pred_count);

    /* --- MULTI-CONDITION FILTER --- */
    printf("\n--- Filter: subject=1 AND predicate=3 ---\n");
    q = KCM_DatabaseQuery(db, "SELECT * FROM facts");
    int multi_count = 0;
    while ((f = KCM_QueryNext(q)) != NULL) {
        if (f->subject == 1 && f->predicate == 3) {
            printf("  Subject: %u, Predicate: %u, Object: %u\n",
                   f->subject, f->predicate, f->object);
            multi_count++;
        }
    }
    KCM_QueryFree(q);
    printf("  Found %d facts matching multi-condition\n", multi_count);

    /* --- ITERATOR PATTERN --- */
    printf("\n--- Iterator Pattern ---\n");
    q = KCM_DatabaseQuery(db, "SELECT * FROM facts");
    while ((f = KCM_QueryNext(q)) != NULL) {
        printf("  Subject: %u, Predicate: %u, Object: %u, Confidence: %.2f\n",
               f->subject, f->predicate, f->object, f->confidence);
    }
    KCM_QueryFree(q);

    KCM_DatabaseFree(db);
    printf("\n=== All query patterns completed ===\n");
    return 0;
}
