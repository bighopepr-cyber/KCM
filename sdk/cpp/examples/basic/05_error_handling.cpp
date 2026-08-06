/**
 * KCM C++ SDK — Error Handling Example.
 *
 * Demonstrates: proper error handling with exceptions and error codes.
 * Compile: g++ -std=c++17 -Wall -Wextra -O2 -I../../include -o 05_error_handling 05_error_handling.cpp -lkcm
 */

#include "kcm.hpp"
#include <iostream>
#include <cassert>

int main() {
    std::cout << "=== KCM C++ SDK — Error Handling Example ===" << std::endl << std::endl;

    try {
        kcm::Database db;

        // --- INVALID CONFIDENCE ---
        std::cout << "--- Invalid Confidence (out of range) ---" << std::endl;
        try {
            db.insert({1, 0, 2, 1.5, 0, 0, 0, 1, 0, 0});
            std::cout << "  FAIL: Should have thrown" << std::endl;
        } catch (const kcm::Error& e) {
            std::cout << "  Caught kcm::Error: " << e.what() << std::endl;
            std::cout << "  Error code: " << static_cast<int>(e.code()) << std::endl;
            assert(e.code() == KCM_ERR_INVALID_ARGUMENT);
        }

        // --- NOT FOUND (update non-existent row) ---
        std::cout << "\n--- Not Found (update non-existent row) ---" << std::endl;
        try {
            db.update(99999, {1, 0, 2, 0.5, 0, 0, 0, 1, 0, 0});
            std::cout << "  FAIL: Should have thrown" << std::endl;
        } catch (const kcm::Error& e) {
            std::cout << "  Caught kcm::Error: " << e.what() << std::endl;
            std::cout << "  Error code: " << static_cast<int>(e.code()) << std::endl;
        }

        // --- NOT FOUND (delete non-existent row) ---
        std::cout << "\n--- Not Found (delete non-existent row) ---" << std::endl;
        try {
            db.remove(99999);
            std::cout << "  Delete succeeded (no exception)" << std::endl;
        } catch (const kcm::Error& e) {
            std::cout << "  Caught kcm::Error: " << e.what() << std::endl;
        }

        // --- ALL ERROR CODES ---
        std::cout << "\n--- All Error Codes ---" << std::endl;
        KCM_Error codes[] = {
            KCM_OK, KCM_ERR_NOT_FOUND, KCM_ERR_OUT_OF_MEMORY,
            KCM_ERR_INVALID_ARGUMENT, KCM_ERR_IO, KCM_ERR_CORRUPTED,
            KCM_ERR_CONFLICT, KCM_ERR_TRANSACTION_ABORTED
        };
        for (auto code : codes) {
            std::cout << "  Code " << static_cast<int>(code) << ": "
                      << KCM_ErrorMessage(code) << std::endl;
        }

        // --- FILE NOT FOUND ---
        std::cout << "\n--- Verify Non-Existent File ---" << std::endl;
        try {
            kcm::Database::verify("/nonexistent/path/db.kcm");
            std::cout << "  FAIL: Should have thrown" << std::endl;
        } catch (const kcm::Error& e) {
            std::cout << "  Caught kcm::Error: " << e.what() << std::endl;
        }

        // --- TRY-CATCH PATTERN ---
        std::cout << "\n--- Try-Catch Pattern ---" << std::endl;
        try {
            db.insert({1, 0, 2, 0.95, 0, 0, 0, 1, 0, 0});
            db.insert({2, 1, 3, 0.90, 0, 0, 0, 1, 0, 0});
            auto results = db.query("SELECT * FROM facts WHERE subject = 1").collect();
            std::cout << "  Query returned " << results.size() << " results" << std::endl;
        } catch (const kcm::Error& e) {
            std::cout << "  Database error: " << e.what() << std::endl;
        } catch (const std::exception& e) {
            std::cout << "  Unexpected error: " << e.what() << std::endl;
        }

        // --- MOVE SEMANTICS ---
        std::cout << "\n--- Move Semantics ---" << std::endl;
        kcm::Database db2;
        db2.insert({10, 0, 20, 0.99, 0, 0, 0, 1, 0, 0});
        kcm::Database db3 = std::move(db2);
        std::cout << "  Moved database: " << db3.fact_count() << " facts" << std::endl;

    } catch (const std::exception& e) {
        std::cerr << "Unexpected error: " << e.what() << std::endl;
        return 1;
    }

    std::cout << "\n=== All error handling patterns completed ===" << std::endl;
    return 0;
}
