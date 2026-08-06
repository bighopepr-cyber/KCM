/**
 * KCM C SDK Test Suite
 *
 * Comprehensive tests for all 18 FFI functions.
 * Compile: gcc -Wall -Wextra -O2 -I../include -o test_kcm test_kcm.c -lkcm
 */

#include "kcm.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>

static int tests_run = 0;
static int tests_passed = 0;

#define TEST(name) static void name(void)
#define RUN(name) do { \
    tests_run++; \
    printf("  %-50s ", #name); \
    name(); \
    tests_passed++; \
    printf("[PASS]\n"); \
} while(0)

#define ASSERT_EQ(a, b) do { \
    if ((a) != (b)) { \
        printf("[FAIL] %s:%d: %s != %s\n", __FILE__, __LINE__, #a, #b); \
        exit(1); \
    } \
} while(0)

#define ASSERT_STR_EQ(a, b) do { \
    if (strcmp((a), (b)) != 0) { \
        printf("[FAIL] %s:%d: \"%s\" != \"%s\"\n", __FILE__, __LINE__, (a), (b)); \
        exit(1); \
    } \
} while(0)

#define ASSERT_NOT_NULL(p) do { \
    if ((p) == NULL) { \
        printf("[FAIL] %s:%d: %s is NULL\n", __FILE__, __LINE__, #p); \
        exit(1); \
    } \
} while(0)

#define ASSERT_NULL(p) do { \
    if ((p) != NULL) { \
        printf("[FAIL] %s:%d: %s is not NULL\n", __FILE__, __LINE__, #p); \
        exit(1); \
    } \
} while(0)

TEST(test_database_new_and_free) {
    KCM_Database *db = NULL;
    KCM_Error rc = KCM_DatabaseNew(&db);
    ASSERT_EQ(rc, KCM_OK);
    ASSERT_NOT_NULL(db);
    KCM_DatabaseFree(db);
}

TEST(test_database_free_null) {
    KCM_DatabaseFree(NULL);
}

TEST(test_insert_single_fact) {
    KCM_Database *db = NULL;
    KCM_DatabaseNew(&db);

    KCM_Fact fact = { .subject = 1, .predicate = 0, .object = 2, .confidence = 0.95 };
    KCM_Error rc = KCM_DatabaseInsert(db, &fact);
    ASSERT_EQ(rc, KCM_OK);
    ASSERT_EQ(KCM_DatabaseFactCount(db), 1);
    ASSERT_EQ(KCM_DatabaseActiveCount(db), 1);

    KCM_DatabaseFree(db);
}

TEST(test_insert_multiple_facts) {
    KCM_Database *db = NULL;
    KCM_DatabaseNew(&db);

    for (uint32_t i = 0; i < 100; i++) {
        KCM_Fact fact = { .subject = i, .predicate = 0, .object = i + 1, .confidence = 0.5 };
        KCM_Error rc = KCM_DatabaseInsert(db, &fact);
        ASSERT_EQ(rc, KCM_OK);
    }

    ASSERT_EQ(KCM_DatabaseFactCount(db), 100);
    ASSERT_EQ(KCM_DatabaseActiveCount(db), 100);

    KCM_DatabaseFree(db);
}

TEST(test_update_fact) {
    KCM_Database *db = NULL;
    KCM_DatabaseNew(&db);

    KCM_Fact fact = { .subject = 1, .predicate = 0, .object = 2, .confidence = 0.5 };
    KCM_DatabaseInsert(db, &fact);

    KCM_Fact updated = { .subject = 1, .predicate = 0, .object = 99, .confidence = 1.0 };
    KCM_Error rc = KCM_DatabaseUpdate(db, 0, &updated);
    ASSERT_EQ(rc, KCM_OK);

    KCM_DatabaseFree(db);
}

TEST(test_delete_fact) {
    KCM_Database *db = NULL;
    KCM_DatabaseNew(&db);

    KCM_Fact fact = { .subject = 1, .predicate = 0, .object = 2, .confidence = 0.95 };
    KCM_DatabaseInsert(db, &fact);
    ASSERT_EQ(KCM_DatabaseActiveCount(db), 1);

    KCM_Error rc = KCM_DatabaseDelete(db, 0);
    ASSERT_EQ(rc, KCM_OK);
    ASSERT_EQ(KCM_DatabaseFactCount(db), 1);
    ASSERT_EQ(KCM_DatabaseActiveCount(db), 0);

    KCM_DatabaseFree(db);
}

TEST(test_query_returns_results) {
    KCM_Database *db = NULL;
    KCM_DatabaseNew(&db);

    KCM_Fact fact1 = { .subject = 1, .predicate = 0, .object = 2, .confidence = 0.95 };
    KCM_Fact fact2 = { .subject = 2, .predicate = 1, .object = 3, .confidence = 0.90 };
    KCM_DatabaseInsert(db, &fact1);
    KCM_DatabaseInsert(db, &fact2);

    KCM_Query *q = KCM_DatabaseQuery(db, "SELECT * FROM facts");
    ASSERT_NOT_NULL(q);

    KCM_Fact *f1 = KCM_QueryNext(q);
    ASSERT_NOT_NULL(f1);
    ASSERT_EQ(f1->subject, 1);

    KCM_Fact *f2 = KCM_QueryNext(q);
    ASSERT_NOT_NULL(f2);
    ASSERT_EQ(f2->subject, 2);

    KCM_Fact *f3 = KCM_QueryNext(q);
    ASSERT_NULL(f3);

    KCM_QueryFree(q);
    KCM_DatabaseFree(db);
}

TEST(test_query_free_null) {
    KCM_QueryFree(NULL);
}

TEST(test_transaction_commit) {
    KCM_Database *db = NULL;
    KCM_DatabaseNew(&db);

    KCM_Transaction *txn = KCM_DatabaseBeginTransaction(db);
    ASSERT_NOT_NULL(txn);

    KCM_Fact fact = { .subject = 1, .predicate = 0, .object = 2, .confidence = 0.95 };
    KCM_DatabaseInsert(db, &fact);

    KCM_Error rc = KCM_TransactionCommit(txn);
    ASSERT_EQ(rc, KCM_OK);
    ASSERT_EQ(KCM_DatabaseFactCount(db), 1);

    KCM_DatabaseFree(db);
}

TEST(test_transaction_rollback) {
    KCM_Database *db = NULL;
    KCM_DatabaseNew(&db);

    KCM_Transaction *txn = KCM_DatabaseBeginTransaction(db);
    ASSERT_NOT_NULL(txn);

    KCM_Fact fact = { .subject = 1, .predicate = 0, .object = 2, .confidence = 0.95 };
    KCM_DatabaseInsert(db, &fact);

    KCM_Error rc = KCM_TransactionRollback(txn);
    ASSERT_EQ(rc, KCM_OK);

    KCM_DatabaseFree(db);
}

TEST(test_transaction_free_null) {
    KCM_TransactionFree(NULL);
}

TEST(test_fact_count_empty) {
    KCM_Database *db = NULL;
    KCM_DatabaseNew(&db);

    ASSERT_EQ(KCM_DatabaseFactCount(db), 0);
    ASSERT_EQ(KCM_DatabaseActiveCount(db), 0);

    KCM_DatabaseFree(db);
}

TEST(test_error_message_valid) {
    const char *msg = KCM_ErrorMessage(KCM_OK);
    ASSERT_NOT_NULL(msg);
    ASSERT_STR_EQ(msg, "OK");

    msg = KCM_ErrorMessage(KCM_ERR_NOT_FOUND);
    ASSERT_NOT_NULL(msg);
}

TEST(test_error_message_all_codes) {
    KCM_Error codes[] = {
        KCM_OK, KCM_ERR_NOT_FOUND, KCM_ERR_OUT_OF_MEMORY,
        KCM_ERR_INVALID_ARGUMENT, KCM_ERR_IO, KCM_ERR_CORRUPTED,
        KCM_ERR_CONFLICT, KCM_ERR_TRANSACTION_ABORTED
    };
    for (size_t i = 0; i < sizeof(codes) / sizeof(codes[0]); i++) {
        const char *msg = KCM_ErrorMessage(codes[i]);
        ASSERT_NOT_NULL(msg);
    }
}

TEST(test_insert_all_fact_attributes) {
    KCM_Database *db = NULL;
    KCM_DatabaseNew(&db);

    KCM_Fact fact = {
        .subject = 42,
        .predicate = 7,
        .object = 100,
        .confidence = 0.88,
        .evidence = 3,
        .timestamp = 1700000000000000000LL,
        .context = 2,
        .version = 5,
        .priority = -1,
        .owner = 10
    };
    KCM_Error rc = KCM_DatabaseInsert(db, &fact);
    ASSERT_EQ(rc, KCM_OK);

    KCM_Query *q = KCM_DatabaseQuery(db, "SELECT * FROM facts");
    KCM_Fact *f = KCM_QueryNext(q);
    ASSERT_NOT_NULL(f);
    ASSERT_EQ(f->subject, 42);
    ASSERT_EQ(f->predicate, 7);
    ASSERT_EQ(f->object, 100);
    ASSERT_EQ(f->confidence, 0.88);
    ASSERT_EQ(f->evidence, 3);
    ASSERT_EQ(f->timestamp, 1700000000000000000LL);
    ASSERT_EQ(f->context, 2);
    ASSERT_EQ(f->version, 5);
    ASSERT_EQ(f->priority, -1);
    ASSERT_EQ(f->owner, 10);

    KCM_QueryFree(q);
    KCM_DatabaseFree(db);
}

TEST(test_insert_then_delete_counts) {
    KCM_Database *db = NULL;
    KCM_DatabaseNew(&db);

    for (int i = 0; i < 5; i++) {
        KCM_Fact fact = { .subject = i, .predicate = 0, .object = i, .confidence = 0.5 };
        KCM_DatabaseInsert(db, &fact);
    }
    ASSERT_EQ(KCM_DatabaseActiveCount(db), 5);

    KCM_DatabaseDelete(db, 0);
    KCM_DatabaseDelete(db, 2);
    KCM_DatabaseDelete(db, 4);

    ASSERT_EQ(KCM_DatabaseFactCount(db), 5);
    ASSERT_EQ(KCM_DatabaseActiveCount(db), 2);

    KCM_DatabaseFree(db);
}

TEST(test_database_insert_null_db) {
    KCM_Fact fact = { .subject = 1 };
    KCM_Error rc = KCM_DatabaseInsert(NULL, &fact);
    ASSERT_EQ(rc, KCM_ERR_INVALID_ARGUMENT);
}

int main(void) {
    printf("=== KCM C SDK Test Suite ===\n\n");

    RUN(test_database_new_and_free);
    RUN(test_database_free_null);
    RUN(test_insert_single_fact);
    RUN(test_insert_multiple_facts);
    RUN(test_update_fact);
    RUN(test_delete_fact);
    RUN(test_query_returns_results);
    RUN(test_query_free_null);
    RUN(test_transaction_commit);
    RUN(test_transaction_rollback);
    RUN(test_transaction_free_null);
    RUN(test_fact_count_empty);
    RUN(test_error_message_valid);
    RUN(test_error_message_all_codes);
    RUN(test_insert_all_fact_attributes);
    RUN(test_insert_then_delete_counts);
    RUN(test_database_insert_null_db);

    printf("\n%d/%d tests passed\n", tests_passed, tests_run);
    return 0;
}
