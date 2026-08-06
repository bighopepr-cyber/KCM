/**
 * KCM C++ SDK — Persistence Example.
 *
 * Demonstrates: save, load, and verify database persistence.
 * Compile: g++ -std=c++17 -Wall -Wextra -O2 -I../../include -o 03_persistence 03_persistence.cpp -lkcm
 */

#include "kcm.hpp"
#include <iostream>
#include <cassert>
#include <cstdio>

int main() {
    std::cout << "=== KCM C++ SDK — Persistence Example ===" << std::endl << std::endl;

    const char* path = "/tmp/kcm_cpp_example.kcm";

    try {
        // --- SAVE DATABASE ---
        std::cout << "--- Save Database ---" << std::endl;
        {
            kcm::Database db;
            db.insert({1, 0, 2, 0.95, 1, 0, 1, 1, 0, 1});
            db.insert({2, 1, 3, 0.90, 2, 0, 1, 1, 0, 2});
            db.insert({3, 2, 4, 0.85, 3, 0, 2, 1, 0, 3});
            std::cout << "  Facts before save: " << db.fact_count()
                      << " total, " << db.active_count() << " active" << std::endl;
            db.save(path);
            std::cout << "  Saved to " << path << std::endl;
        }

        // --- VERIFY FILE ---
        std::cout << "\n--- Verify Database File ---" << std::endl;
        kcm::Database::verify(path);
        std::cout << "  Verification passed" << std::endl;

        // --- LOAD INTO NEW DATABASE ---
        std::cout << "\n--- Load Into New Database ---" << std::endl;
        {
            kcm::Database db2;
            db2.load(path);
            std::cout << "  Loaded: " << db2.fact_count() << " total, "
                      << db2.active_count() << " active" << std::endl;
            assert(db2.fact_count() == 3);
            assert(db2.active_count() == 3);

            // --- VERIFY DATA INTEGRITY ---
            std::cout << "\n--- Verify Data Integrity ---" << std::endl;
            auto all = db2.queryAll();
            for (const auto& f : all) {
                std::cout << "  Subject: " << f.subject << ", Predicate: " << f.predicate
                          << ", Object: " << f.object << ", Confidence: " << f.confidence << std::endl;
            }
            assert(all.size() == 3);

            // --- SAVE-LOAD ROUND TRIP ---
            std::cout << "\n--- Save-Load Round Trip ---" << std::endl;
            db2.insert({10, 0, 20, 0.99, 0, 0, 0, 1, 0, 0});
            db2.save(path);
        }

        {
            kcm::Database db3;
            db3.load(path);
            std::cout << "  Round-trip: " << db3.fact_count() << " total, "
                      << db3.active_count() << " active" << std::endl;
            assert(db3.fact_count() == 4);
            assert(db3.active_fact_count() == 4);
        }

    } catch (const std::exception& e) {
        std::cerr << "Error: " << e.what() << std::endl;
        std::remove(path);
        return 1;
    }

    std::remove(path);
    std::cout << "\n=== All persistence operations completed ===" << std::endl;
    return 0;
}
