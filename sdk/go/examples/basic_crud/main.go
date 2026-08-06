package main

import (
	"fmt"
	"log"

	kcm "github.com/kcm-project/go-sdk"
)

// Basic CRUD: insert, query, update, delete operations on facts.
func main() {
	db, err := kcm.NewDatabase()
	if err != nil {
		log.Fatal(err)
	}
	defer db.Close()

	// --- INSERT ---
	fmt.Println("--- Insert Facts ---")
	facts := []kcm.Fact{
		{Subject: 1, Predicate: 0, Object: 2, Confidence: 0.95, Evidence: 0, Context: 0, Version: 1, Owner: 0},
		{Subject: 2, Predicate: 1, Object: 3, Confidence: 0.90, Evidence: 1, Context: 1, Version: 1, Owner: 1},
		{Subject: 3, Predicate: 2, Object: 4, Confidence: 0.85, Evidence: 2, Context: 2, Version: 1, Owner: 2},
		{Subject: 1, Predicate: 3, Object: 5, Confidence: 0.80, Evidence: 3, Context: 2, Version: 1, Owner: 7},
	}
	for _, f := range facts {
		if err := db.Insert(f); err != nil {
			log.Fatal(err)
		}
	}
	fmt.Printf("  Inserted %d facts\n", len(facts))
	fmt.Printf("  Total: %d, Active: %d\n", db.FactCount(), db.ActiveFactCount())

	// --- QUERY ALL ---
	fmt.Println("\n--- Query All Facts ---")
	allFacts, err := db.QueryAll()
	if err != nil {
		log.Fatal(err)
	}
	for _, f := range allFacts {
		fmt.Printf("  Subject: %d, Predicate: %d, Object: %d, Confidence: %.2f\n",
			f.Subject, f.Predicate, f.Object, f.Confidence)
	}

	// --- QUERY WITH KQL ---
	fmt.Println("\n--- KQL Query: SELECT * FROM facts WHERE subject = 1 ---")
	qr := db.Query("SELECT * FROM facts")
	count := 0
	for {
		fact, ok := qr.Next()
		if !ok {
			break
		}
		if fact.Subject == 1 {
			fmt.Printf("  Subject: %d, Predicate: %d, Object: %d\n",
				fact.Subject, fact.Predicate, fact.Object)
			count++
		}
	}
	qr.Close()
	fmt.Printf("  Found %d facts with subject=1\n", count)

	// --- UPDATE ---
	fmt.Println("\n--- Update Fact ---")
	err = db.Update(0, kcm.Fact{
		Subject: 10, Predicate: 0, Object: 20, Confidence: 0.99,
		Evidence: 5, Context: 3, Version: 2, Priority: 2, Owner: 10,
	})
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println("  Updated row 0: subject changed to 10")

	// --- DELETE ---
	fmt.Println("\n--- Delete Fact ---")
	if err := db.Delete(3); err != nil {
		log.Fatal(err)
	}
	fmt.Printf("  Deleted row 3\n")
	fmt.Printf("  Total: %d, Active: %d\n", db.FactCount(), db.ActiveFactCount())

	// --- VERIFY COUNTS ---
	fmt.Println("\n--- Verify Counts ---")
	if db.FactCount() != 4 {
		log.Fatalf("Expected 4 total, got %d", db.FactCount())
	}
	if db.ActiveFactCount() != 3 {
		log.Fatalf("Expected 3 active, got %d", db.ActiveFactCount())
	}
	fmt.Println("  Counts verified: 4 total, 3 active")

	fmt.Println("\n=== All operations completed ===")
}
