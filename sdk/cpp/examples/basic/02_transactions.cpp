/**
 * KCM C++ SDK — Transaction Example.
 *
 * Demonstrates: begin, commit, and rollback scenarios with RAII transactions.
 * Compile: g++ -std=c++17 -Wall -Wextra -O2 -I../../include -o 02_transactions 02_transactions.cpp -lkcm
 */

#include "kcm.hpp"
#include <iostream>
#include <cassert>

int main() {
    std::cout << "=== KCM C++ SDK — Transaction Example ===" << std::endl << std::endl;

    try {
        kcm::Database db;

        // Insert baseline facts
        db.insert({1, 0, 2, 0.95, 0, 0, 0, 1, 0, 0});
        db.insert({2, 1, 3, 0.90, 1, 0, 1, 1, 0, 1});
        std::cout << "Initial: " << db.active_count() << " active facts" << std::endl << std::endl;

        // --- COMMITTED TRANSACTION ---
        std::cout << "--- Committed Transaction ---" << std::endl;
        {
            auto txn = db.begin_transaction();
            db.insert({3, 2, 4, 0.85, 2, 0, 2, 1, 0, 2});
            std::cout << "  Inserted fact in transaction" << std::endl;
            txn.commit();
            std::cout << "  Committed transaction" << std::endl;
        }
        std::cout << "  After commit: " << db.active_count() << " active facts" << std::endl;
        assert(db.active_count() == 3);

        // --- ROLLED BACK TRANSACTION ---
        std::cout << "\n--- Rolled Back Transaction ---" << std::endl;
        {
            auto txn = db.begin_transaction();
            db.insert({4, 3, 5, 0.80, 3, 0, 2, 1, 0, 3});
            std::cout << "  Inserted fact in transaction" << std::endl;
            txn.rollback();
            std::cout << "  Rolled back transaction" << std::endl;
        }
        std::cout << "  After rollback: " << db.active_count() << " active facts" << std::endl;
        assert(db.active_count() == 3);

        // --- AUTO-ROLLBACK ON EXCEPTION ---
        std::cout << "\n--- Auto-Rollback on Exception ---" << std::endl;
        uint64_t countBefore = db.active_count();
        {
            auto txn = db.begin_transaction();
            db.insert({5, 4, 6, 0.70, 0, 0, 0, 1, 0, 0});
            std::cout << "  Inserted fact, about to throw" << std::endl;
            throw std::runtime_error("simulated error");
        }
        // txn destructor calls rollback automatically
        std::cout << "  After exception: " << db.active_count() << " active facts" << std::endl;
        std::cout << "  Transaction auto-rolled back: " << (db.active_count() == countBefore) << std::endl;

        // --- MULTIPLE OPERATIONS ---
        std::cout << "\n--- Multiple Operations in Transaction ---" << std::endl;
        {
            auto txn = db.begin_transaction();
            db.insert({10, 0, 20, 0.99, 0, 0, 0, 1, 0, 0});
            db.insert({30, 1, 40, 0.88, 0, 0, 0, 1, 0, 0});
            db.insert({50, 2, 60, 0.77, 0, 0, 0, 1, 0, 0});
            std::cout << "  3 pending operations" << std::endl;
            txn.commit();
        }
        std::cout << "  After commit: " << db.active_count() << " active facts" << std::endl;

    } catch (const std::exception& e) {
        std::cerr << "Error: " << e.what() << std::endl;
        return 1;
    }

    std::cout << "\n=== All transaction operations completed ===" << std::endl;
    return 0;
}
