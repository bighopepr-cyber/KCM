/**
 * KCM C++ SDK — Basic CRUD Example.
 *
 * Demonstrates: insert, query, update, delete with RAII and exception safety.
 * Compile: g++ -std=c++17 -Wall -Wextra -O2 -I../../include -o 01_basic_crud 01_basic_crud.cpp -lkcm
 */

#include "kcm.hpp"
#include <iostream>
#include <cassert>

int main() {
    std::cout << "=== KCM C++ SDK — Basic CRUD Example ===" << std::endl << std::endl;

    try {
        kcm::Database db;

        // --- INSERT ---
        std::cout << "--- Insert Facts ---" << std::endl;
        db.insert({1, 0, 2, 0.95, 0, 0, 0, 1, 0, 0});
        db.insert({2, 1, 3, 0.90, 1, 0, 1, 1, 0, 1});
        db.insert({3, 2, 4, 0.85, 2, 0, 2, 1, 0, 2});
        db.insert({1, 3, 5, 0.80, 3, 0, 2, 1, -1, 7});
        std::cout << "  Inserted 4 facts" << std::endl;
        std::cout << "  Total: " << db.fact_count() << ", Active: " << db.active_count() << std::endl;

        // --- QUERY ALL ---
        std::cout << "\n--- Query All Facts ---" << std::endl;
        auto all = db.queryAll();
        std::cout << "  Returned " << all.size() << " facts:" << std::endl;
        for (const auto& f : all) {
            std::cout << "  Subject: " << f.subject << ", Predicate: " << f.predicate
                      << ", Object: " << f.object << ", Confidence: " << f.confidence << std::endl;
        }

        // --- QUERY WITH FILTER ---
        std::cout << "\n--- KQL Query: SELECT * FROM facts WHERE subject = 1 ---" << std::endl;
        auto query = db.query("SELECT * FROM facts WHERE subject = 1");
        auto filtered = query.collect();
        std::cout << "  Returned " << filtered.size() << " facts" << std::endl;
        for (const auto& f : filtered) {
            std::cout << "  Subject: " << f.subject << ", Predicate: " << f.predicate
                      << ", Object: " << f.object << std::endl;
        }

        // --- UPDATE ---
        std::cout << "\n--- Update Fact ---" << std::endl;
        db.update(0, {10, 0, 20, 0.99, 5, 0, 3, 2, 2, 10});
        std::cout << "  Updated row 0: subject changed to 10" << std::endl;

        // --- DELETE ---
        std::cout << "\n--- Delete Fact ---" << std::endl;
        db.remove(3);
        std::cout << "  Deleted row 3" << std::endl;
        std::cout << "  Total: " << db.fact_count() << ", Active: " << db.active_count() << std::endl;

        // --- VERIFY COUNTS ---
        std::cout << "\n--- Verify Counts ---" << std::endl;
        assert(db.fact_count() == 4);
        assert(db.active_count() == 3);
        std::cout << "  Counts verified: 4 total, 3 active" << std::endl;

    } catch (const std::exception& e) {
        std::cerr << "Unexpected error: " << e.what() << std::endl;
        return 1;
    }

    std::cout << "\n=== All operations completed ===" << std::endl;
    return 0;
}
