/**
 * KCM Knowledge Columnar Model - C++ SDK
 * 
 * RAII wrapper over the C FFI. Provides automatic resource management,
 * exception-safe error handling, and modern C++17 interface.
 */

#ifndef KCM_HPP
#define KCM_HPP

#include "kcm.h"
#include <string>
#include <vector>
#include <stdexcept>
#include <memory>
#include <optional>

namespace kcm {

class Error : public std::runtime_error {
public:
    explicit Error(KCM_Error code, const std::string& msg = "")
        : std::runtime_error(std::string(KCM_ErrorMessage(code)) + (msg.empty() ? "" : ": " + msg))
        , code_(code) {}
    KCM_Error code() const { return code_; }
private:
    KCM_Error code_;
};

struct Fact {
    uint32_t subject = 0;
    uint8_t  predicate = 0;
    uint32_t object = 0;
    double   confidence = 0.0;
    uint8_t  evidence = 0;
    int64_t  timestamp = 0;
    uint8_t  context = 0;
    int32_t  version = 0;
    int8_t   priority = 0;
    uint16_t owner = 0;

    KCM_Fact to_c() const {
        return KCM_Fact{subject, predicate, object, confidence,
                        evidence, timestamp, context, version, priority, owner};
    }

    static Fact from_c(const KCM_Fact& c) {
        return Fact{c.subject, c.predicate, c.object, c.confidence,
                    c.evidence, c.timestamp, c.context, c.version, c.priority, c.owner};
    }
};

class Query {
public:
    explicit Query(KCM_Query* q) : query_(q) {}
    ~Query() { if (query_) KCM_QueryFree(query_); }
    
    Query(const Query&) = delete;
    Query& operator=(const Query&) = delete;
    Query(Query&& o) noexcept : query_(o.query_) { o.query_ = nullptr; }
    Query& operator=(Query&& o) noexcept {
        if (this != &o) { if (query_) KCM_QueryFree(query_); query_ = o.query_; o.query_ = nullptr; }
        return *this;
    }

    std::optional<Fact> next() {
        if (!query_) return std::nullopt;
        auto* f = KCM_QueryNext(query_);
        if (!f) return std::nullopt;
        auto fact = Fact::from_c(*f);
        return fact;
    }

    std::vector<Fact> collect() {
        std::vector<Fact> results;
        while (auto f = next()) results.push_back(*f);
        return results;
    }

private:
    KCM_Query* query_;
};

class Transaction {
public:
    explicit Transaction(KCM_Transaction* t) : txn_(t) {}
    ~Transaction() { if (txn_) KCM_TransactionFree(txn_); }
    
    Transaction(const Transaction&) = delete;
    Transaction& operator=(const Transaction&) = delete;
    Transaction(Transaction&& o) noexcept : txn_(o.txn_) { o.txn_ = nullptr; }

    void commit() {
        if (!txn_) throw Error(KCM_ERR_INVALID_ARGUMENT, "null transaction");
        auto rc = KCM_TransactionCommit(txn_);
        if (rc != KCM_OK) throw Error(rc);
        txn_ = nullptr;
    }

    void rollback() {
        if (!txn_) return;
        KCM_TransactionRollback(txn_);
        txn_ = nullptr;
    }

private:
    KCM_Transaction* txn_;
};

class Database {
public:
    Database() {
        KCM_Database* db = nullptr;
        auto rc = KCM_DatabaseNew(&db);
        if (rc != KCM_OK) throw Error(rc);
        db_ = db;
    }

    ~Database() { if (db_) KCM_DatabaseFree(db_); }

    Database(const Database&) = delete;
    Database& operator=(const Database&) = delete;
    Database(Database&& o) noexcept : db_(o.db_) { o.db_ = nullptr; }

    void insert(const Fact& fact) {
        auto c = fact.to_c();
        auto rc = KCM_DatabaseInsert(db_, &c);
        if (rc != KCM_OK) throw Error(rc);
    }

    void update(uint64_t row_id, const Fact& fact) {
        auto c = fact.to_c();
        auto rc = KCM_DatabaseUpdate(db_, row_id, &c);
        if (rc != KCM_OK) throw Error(rc);
    }

    void remove(uint64_t row_id) {
        auto rc = KCM_DatabaseDelete(db_, row_id);
        if (rc != KCM_OK) throw Error(rc);
    }

    uint64_t fact_count() const { return KCM_DatabaseFactCount(db_); }
    uint64_t active_count() const { return KCM_DatabaseActiveCount(db_); }

    Query query(const std::string& kql) {
        auto* q = KCM_DatabaseQuery(db_, kql.c_str());
        return Query(q);
    }

    Transaction begin_transaction() {
        return Transaction(KCM_DatabaseBeginTransaction(db_));
    }

    void save(const std::string& path) {
        auto rc = KCM_DatabaseSave(db_, path.c_str());
        if (rc != KCM_OK) throw Error(rc);
    }

    void load(const std::string& path) {
        auto rc = KCM_DatabaseLoad(db_, path.c_str());
        if (rc != KCM_OK) throw Error(rc);
    }

    static void verify(const std::string& path) {
        auto rc = KCM_DatabaseVerify(path.c_str());
        if (rc != KCM_OK) throw Error(rc);
    }

    KCM_Database* raw() { return db_; }

private:
    KCM_Database* db_ = nullptr;
};

} // namespace kcm

#endif /* KCM_HPP */
