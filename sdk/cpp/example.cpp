/**
 * KCM C++ SDK Example
 *
 * Demonstrates RAII, exception safety, and modern C++ patterns.
 * Compile: g++ -std=c++17 -o kcm_example example.cpp -lkcm
 */
#include "kcm.hpp"
#include <iostream>
#include <cassert>

int main() {
    std::cout << "=== KCM C++ SDK Example ===" << std::endl << std::endl;

    try {
        // RAII: database automatically freed on scope exit
        kcm::Database db;

        // Insert facts
        db.insert({1, 0, 2, 0.95, 0, 0, 0, 0, 0, 0});
        db.insert({2, 1, 3, 0.90, 0, 0, 0, 0, 0, 0});
        db.insert({3, 2, 4, 0.85, 0, 0, 0, 0, 0, 0});
        std::cout << "Inserted 3 facts" << std::endl;

        // Query all
        auto results = db.query("*").collect();
        std::cout << "Query returned " << results.size() << " facts" << std::endl;

        // Transaction with RAII
        {
            auto txn = db.begin_transaction();
            db.insert({4, 3, 5, 0.80, 0, 0, 0, 0, 0, 0});
            txn.commit();  // auto-rollback if not committed
            std::cout << "Committed transaction" << std::endl;
        }

        // Delete
        db.remove(0);
        std::cout << "Deleted row 0" << std::endl;

        // Stats
        std::cout << "Fact count: " << db.fact_count() << std::endl;
        std::cout << "Active: " << db.active_count() << std::endl;

        // Error handling
        try {
            kcm::Database::verify("/nonexistent.kcm");
        } catch (const kcm::Error& e) {
            std::cout << "Expected error: " << e.what() << std::endl;
        }

    } catch (const std::exception& e) {
        std::cerr << "Unexpected error: " << e.what() << std::endl;
        return 1;
    }

    std::cout << std::endl << "All C++ SDK tests passed!" << std::endl;
    return 0;
}
