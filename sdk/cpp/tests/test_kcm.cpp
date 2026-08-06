/**
 * KCM C++ SDK Test Suite
 *
 * Comprehensive tests for RAII wrapper classes.
 * Compile: g++ -std=c++17 -Wall -Wextra -O2 -I../include -o test_kcm test_kcm.cpp -lkcm
 */

#include "kcm.hpp"
#include <iostream>
#include <cassert>
#include <string>

static int tests_run = 0;
static int tests_passed = 0;

#define TEST(name) static void name()
#define RUN(name) do { \
    tests_run++; \
    std::cout << "  " << #name << " ... "; \
    name(); \
    tests_passed++; \
    std::cout << "[PASS]" << std::endl; \
} while(0)

#define ASSERT_EQ(a, b) do { \
    if ((a) != (b)) { \
        std::cerr << "\n[FAIL] " << __FILE__ << ":" << __LINE__ << ": " << #a << " != " << #b << std::endl; \
        std::abort(); \
    } \
} while(0)

#define ASSERT_THROWS(expr, exc) do { \
    try { expr; } \
    catch (const exc&) { break; } \
    catch (...) { \
        std::cerr << "\n[FAIL] " << __FILE__ << ":" << __LINE__ << ": expected " << #exc << std::endl; \
        std::abort(); \
    } \
    std::cerr << "\n[FAIL] " << __FILE__ << ":" << __LINE__ << ": no exception thrown" << std::endl; \
    std::abort(); \
} while(0)

TEST(test_database_construction) {
    kcm::Database db;
    ASSERT_EQ(db.fact_count(), 0);
    ASSERT_EQ(db.active_count(), 0);
}

TEST(test_database_move_semantics) {
    kcm::Database db1;
    db1.insert({1, 0, 2, 0.95});

    kcm::Database db2 = std::move(db1);
    ASSERT_EQ(db2.fact_count(), 1);
}

TEST(test_insert_single) {
    kcm::Database db;
    db.insert({1, 0, 2, 0.95});
    ASSERT_EQ(db.fact_count(), 1);
    ASSERT_EQ(db.active_count(), 1);
}

TEST(test_insert_multiple) {
    kcm::Database db;
    for (uint32_t i = 0; i < 50; i++) {
        db.insert({i, 0, i + 1, 0.5});
    }
    ASSERT_EQ(db.fact_count(), 50);
    ASSERT_EQ(db.active_count(), 50);
}

TEST(test_update_fact) {
    kcm::Database db;
    db.insert({1, 0, 2, 0.5});
    db.update(0, {1, 0, 99, 1.0});
    ASSERT_EQ(db.fact_count(), 1);
}

TEST(test_remove_fact) {
    kcm::Database db;
    db.insert({1, 0, 2, 0.95});
    ASSERT_EQ(db.active_count(), 1);

    db.remove(0);
    ASSERT_EQ(db.fact_count(), 1);
    ASSERT_EQ(db.active_count(), 0);
}

TEST(test_query_returns_results) {
    kcm::Database db;
    db.insert({1, 0, 2, 0.95});
    db.insert({2, 1, 3, 0.90});

    auto results = db.query("SELECT * FROM facts").collect();
    ASSERT_EQ(results.size(), 2);
    ASSERT_EQ(results[0].subject, 1);
    ASSERT_EQ(results[1].subject, 2);
}

TEST(test_query_all) {
    kcm::Database db;
    db.insert({1, 0, 2, 0.95});
    db.insert({2, 1, 3, 0.90});
    db.insert({3, 2, 4, 0.85});

    auto results = db.queryAll();
    ASSERT_EQ(results.size(), 3);
}

TEST(test_query_empty) {
    kcm::Database db;
    auto results = db.query("SELECT * FROM facts").collect();
    ASSERT_EQ(results.size(), 0);
}

TEST(test_query_move_semantics) {
    kcm::Database db;
    db.insert({1, 0, 2, 0.95});

    auto q1 = db.query("SELECT * FROM facts");
    auto q2 = std::move(q1);

    auto fact = q2.next();
    assert(fact.has_value());
    ASSERT_EQ(fact->subject, 1);
}

TEST(test_transaction_commit) {
    kcm::Database db;
    {
        auto txn = db.begin_transaction();
        db.insert({1, 0, 2, 0.95});
        txn.commit();
    }
    ASSERT_EQ(db.fact_count(), 1);
}

TEST(test_transaction_rollback) {
    kcm::Database db;
    {
        auto txn = db.begin_transaction();
        db.insert({1, 0, 2, 0.95});
        txn.rollback();
    }
    ASSERT_EQ(db.fact_count(), 0);
}

TEST(test_transaction_destructor_rollback) {
    kcm::Database db;
    {
        auto txn = db.begin_transaction();
        db.insert({1, 0, 2, 0.95});
    }
    ASSERT_EQ(db.fact_count(), 0);
}

TEST(test_fact_attributes) {
    kcm::Database db;
    kcm::Fact f{
        .subject = 42, .predicate = 7, .object = 100,
        .confidence = 0.88, .evidence = 3, .timestamp = 1700000000000000000LL,
        .context = 2, .version = 5, .priority = -1, .owner = 10
    };
    db.insert(f);

    auto results = db.queryAll();
    ASSERT_EQ(results.size(), 1);
    auto& r = results[0];
    ASSERT_EQ(r.subject, 42u);
    ASSERT_EQ(r.predicate, 7u);
    ASSERT_EQ(r.object, 100u);
    ASSERT_EQ(r.confidence, 0.88);
    ASSERT_EQ(r.evidence, 3u);
    ASSERT_EQ(r.timestamp, 1700000000000000000LL);
    ASSERT_EQ(r.context, 2u);
    ASSERT_EQ(r.version, 5);
    ASSERT_EQ(r.priority, -1);
    ASSERT_EQ(r.owner, 10u);
}

TEST(test_error_code) {
    kcm::Error err(KCM_ERR_NOT_FOUND, "test");
    ASSERT_EQ(err.code(), KCM_ERR_NOT_FOUND);
}

TEST(test_insert_invalid_throws) {
    kcm::Database db;
    ASSERT_THROWS(db.insert({0, 0, 0, -1.0}), kcm::Error);
}

TEST(test_fact_conversions) {
    kcm::Fact original{1, 2, 3, 0.75, 4, 5, 6, 7, 8, 9};
    KCM_Fact c_fact = original.to_c();
    kcm::Fact restored = kcm::Fact::from_c(c_fact);

    ASSERT_EQ(restored.subject, original.subject);
    ASSERT_EQ(restored.predicate, original.predicate);
    ASSERT_EQ(restored.object, original.object);
    ASSERT_EQ(restored.confidence, original.confidence);
}

int main() {
    std::cout << "=== KCM C++ SDK Test Suite ===" << std::endl << std::endl;

    RUN(test_database_construction);
    RUN(test_database_move_semantics);
    RUN(test_insert_single);
    RUN(test_insert_multiple);
    RUN(test_update_fact);
    RUN(test_remove_fact);
    RUN(test_query_returns_results);
    RUN(test_query_all);
    RUN(test_query_empty);
    RUN(test_query_move_semantics);
    RUN(test_transaction_commit);
    RUN(test_transaction_rollback);
    RUN(test_transaction_destructor_rollback);
    RUN(test_fact_attributes);
    RUN(test_error_code);
    RUN(test_insert_invalid_throws);
    RUN(test_fact_conversions);

    std::cout << std::endl << tests_passed << "/" << tests_run << " tests passed" << std::endl;
    return 0;
}
