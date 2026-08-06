package main

import (
	"fmt"
	"log"

	kcm "github.com/kcm-project/go-sdk"
)

// Transactions: begin, commit, and rollback scenarios.
func main() {
	db, err := kcm.NewDatabase()
	if err != nil {
		log.Fatal(err)
	}
	defer db.Close()

	// Insert baseline facts
	if err := db.Insert(kcm.Fact{Subject: 1, Predicate: 0, Object: 2, Confidence: 0.95}); err != nil {
		log.Fatal(err)
	}
	if err := db.Insert(kcm.Fact{Subject: 2, Predicate: 1, Object: 3, Confidence: 0.90}); err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Initial: %d active facts\n\n", db.ActiveFactCount())

	// --- COMMITTED TRANSACTION ---
	fmt.Println("--- Committed Transaction ---")
	txn1 := db.BeginTransaction()
	if err := db.Insert(kcm.Fact{Subject: 3, Predicate: 2, Object: 4, Confidence: 0.85,
		Evidence: 2, Context: 2, Version: 1, Owner: 2}); err != nil {
		log.Fatal(err)
	}
	fmt.Println("  Inserted fact in transaction")
	if err := txn1.Commit(); err != nil {
		log.Fatal(err)
	}
	fmt.Printf("  After commit: %d active facts\n", db.ActiveFactCount())
	if db.ActiveFactCount() != 3 {
		log.Fatal("Expected 3 active facts after commit")
	}

	// --- ROLLED BACK TRANSACTION ---
	fmt.Println("\n--- Rolled Back Transaction ---")
	txn2 := db.BeginTransaction()
	if err := db.Insert(kcm.Fact{Subject: 4, Predicate: 3, Object: 5, Confidence: 0.80,
		Evidence: 3, Context: 2, Version: 1, Owner: 3}); err != nil {
		log.Fatal(err)
	}
	fmt.Println("  Inserted fact in transaction")
	if err := txn2.Rollback(); err != nil {
		log.Fatal(err)
	}
	fmt.Printf("  After rollback: %d active facts\n", db.ActiveFactCount())
	if db.ActiveFactCount() != 3 {
		log.Fatal("Expected 3 active facts after rollback")
	}

	// --- TRANSACTION WITH UPDATE ---
	fmt.Println("\n--- Transaction with Update ---")
	txn3 := db.BeginTransaction()
	if err := db.Update(0, kcm.Fact{
		Subject: 10, Predicate: 0, Object: 20, Confidence: 0.99,
		Evidence: 5, Context: 3, Version: 2, Owner: 1,
	}); err != nil {
		log.Fatal(err)
	}
	fmt.Println("  Updated fact in transaction")
	if err := txn3.Commit(); err != nil {
		log.Fatal(err)
	}
	fmt.Println("  Transaction committed successfully")

	// --- MULTIPLE OPERATIONS ---
	fmt.Println("\n--- Multiple Operations in Transaction ---")
	txn4 := db.BeginTransaction()
	if err := db.Insert(kcm.Fact{Subject: 10, Predicate: 0, Object: 20, Confidence: 0.99}); err != nil {
		log.Fatal(err)
	}
	if err := db.Insert(kcm.Fact{Subject: 30, Predicate: 1, Object: 40, Confidence: 0.88}); err != nil {
		log.Fatal(err)
	}
	if err := db.Insert(kcm.Fact{Subject: 50, Predicate: 2, Object: 60, Confidence: 0.77}); err != nil {
		log.Fatal(err)
	}
	fmt.Println("  3 pending operations")
	if err := txn4.Commit(); err != nil {
		log.Fatal(err)
	}
	fmt.Printf("  After commit: %d active facts\n", db.ActiveFactCount())

	fmt.Println("\n=== All transaction operations completed ===")
}
