/**
 * KCM C++ SDK — Query Patterns Example.
 *
 * Demonstrates: different KQL query patterns and filtering options.
 * Compile: g++ -std=c++17 -Wall -Wextra -O2 -I../../include -o 04_query_patterns 04_query_patterns.cpp -lkcm
 */

#include "kcm.hpp"
#include <iostream>
#include <cassert>

int main() {
    std::cout << "=== KCM C++ SDK — Query Patterns Example ===" << std::endl << std::endl;

    try {
        kcm::Database db;

        // Insert test data
        db.insert({1, 0, 2, 0.95, 1, 0, 1, 1, 0, 1});
        db.insert({2, 1, 3, 0.90, 2, 0, 1, 1, 0, 2});
        db.insert({3, 2, 4, 0.85, 3, 0, 2, 1, 0, 3});
        db.insert({1, 3, 5, 0.80, 1, 0, 2, 1, 0, 1});
        db.insert({4, 0, 6, 0.75, 2, 0, 1, 1, 0, 2});
        std::cout << "Inserted 5 facts" << std::endl << std::endl;

        // --- SELECT ALL ---
        std::cout << "--- SELECT * FROM facts ---" << std::endl;
        auto all = db.query("SELECT * FROM facts").collect();
        std::cout << "  Returned " << all.size() << " facts" << std::endl;

        // --- FILTER BY SUBJECT ---
        std::cout << "\n--- Filter by Subject = 1 ---" << std::endl;
        auto bySubject = db.query("SELECT * FROM facts WHERE subject = 1").collect();
        std::cout << "  Returned " << bySubject.size() << " facts" << std::endl;
        for (const auto& f : bySubject) {
            std::cout << "  Subject: " << f.subject << ", Predicate: " << f.predicate
                      << ", Object: " << f.object << std::endl;
        }
        assert(bySubject.size() == 2);

        // --- FILTER BY PREDICATE ---
        std::cout << "\n--- Filter by Predicate = 0 ---" << std::endl;
        auto byPred = db.query("SELECT * FROM facts WHERE predicate = 0").collect();
        std::cout << "  Returned " << byPred.size() << " facts" << std::endl;
        for (const auto& f : byPred) {
            std::cout << "  Subject: " << f.subject << ", Predicate: " << f.predicate
                      << ", Object: " << f.object << std::endl;
        }
        assert(byPred.size() == 2);

        // --- FILTER BY OBJECT ---
        std::cout << "\n--- Filter by Object = 4 ---" << std::endl;
        auto byObj = db.query("SELECT * FROM facts WHERE object = 4").collect();
        std::cout << "  Returned " << byObj.size() << " facts" << std::endl;
        assert(byObj.size() == 1);

        // --- MULTI-CONDITION FILTER ---
        std::cout << "\n--- Filter: subject=1 AND predicate=3 ---" << std::endl;
        auto multi = db.query("SELECT * FROM facts WHERE subject = 1 AND predicate = 3").collect();
        std::cout << "  Returned " << multi.size() << " facts" << std::endl;
        assert(multi.size() == 1);

        // --- QUERY ALL CONVENIENCE ---
        std::cout << "\n--- queryAll() convenience method ---" << std::endl;
        auto allFacts = db.queryAll();
        std::cout << "  Returned " << allFacts.size() << " facts" << std::endl;
        assert(allFacts.size() == 5);

        // --- ITERATOR PATTERN ---
        std::cout << "\n--- Iterator Pattern ---" << std::endl;
        auto q = db.query("SELECT * FROM facts WHERE subject = 1");
        while (auto fact = q.next()) {
            std::cout << "  Subject: " << fact->subject << ", Predicate: " << fact->predicate
                      << ", Object: " << fact->object
                      << ", Confidence: " << fact->confidence << std::endl;
        }

    } catch (const std::exception& e) {
        std::cerr << "Error: " << e.what() << std::endl;
        return 1;
    }

    std::cout << "\n=== All query patterns completed ===" << std::endl;
    return 0;
}
