/**
 * KCM C++ SDK Basic Example
 *
 * Demonstrates RAII, exception safety, and modern C++ patterns.
 * Compile: g++ -std=c++17 -Wall -Wextra -O2 -I../../include -o basic basic.cpp -lkcm
 */

#include "kcm.hpp"
#include <iostream>
#include <cassert>

int main() {
    std::cout << "=== KCM C++ SDK Basic Example ===" << std::endl << std::endl;

    try {
        kcm::Database db;

        db.insert({1, 0, 2, 0.95, 0, 0, 0, 0, 0, 0});
        db.insert({2, 1, 3, 0.90, 0, 0, 0, 0, 0, 0});
        db.insert({3, 2, 4, 0.85, 0, 0, 0, 0, 0, 0});
        std::cout << "Inserted 3 facts" << std::endl;

        auto results = db.queryAll();
        std::cout << "Query returned " << results.size() << " facts" << std::endl;

        {
            auto txn = db.begin_transaction();
            db.insert({4, 3, 5, 0.80, 0, 0, 0, 0, 0, 0});
            txn.commit();
            std::cout << "Committed transaction" << std::endl;
        }

        db.remove(0);
        std::cout << "Deleted row 0" << std::endl;

        std::cout << "Fact count: " << db.fact_count() << std::endl;
        std::cout << "Active: " << db.active_count() << std::endl;

        try {
            kcm::Database::verify("/nonexistent.kcm");
        } catch (const kcm::Error& e) {
            std::cout << "Expected error: " << e.what() << std::endl;
        }

    } catch (const std::exception& e) {
        std::cerr << "Unexpected error: " << e.what() << std::endl;
        return 1;
    }

    std::cout << std::endl << "All C++ SDK examples passed!" << std::endl;
    return 0;
}
