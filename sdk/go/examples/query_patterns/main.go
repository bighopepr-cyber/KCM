package main

import (
	"fmt"
	"log"

	kcm "github.com/kcm-project/go-sdk"
)

// Query Patterns: different KQL query patterns and filtering options.
func main() {
	db, err := kcm.NewDatabase()
	if err != nil {
		log.Fatal(err)
	}
	defer db.Close()

	// Insert test data
	facts := []kcm.Fact{
		{Subject: 1, Predicate: 0, Object: 2, Confidence: 0.95, Evidence: 1, Context: 1, Version: 1, Owner: 1},
		{Subject: 2, Predicate: 1, Object: 3, Confidence: 0.90, Evidence: 2, Context: 1, Version: 1, Owner: 2},
		{Subject: 3, Predicate: 2, Object: 4, Confidence: 0.85, Evidence: 3, Context: 2, Version: 1, Owner: 3},
		{Subject: 1, Predicate: 3, Object: 5, Confidence: 0.80, Evidence: 1, Context: 2, Version: 1, Owner: 1},
		{Subject: 4, Predicate: 0, Object: 6, Confidence: 0.75, Evidence: 2, Context: 1, Version: 1, Owner: 2},
	}
	for _, f := range facts {
		if err := db.Insert(f); err != nil {
			log.Fatal(err)
		}
	}
	fmt.Println("Inserted 5 facts\n")

	// --- SELECT ALL ---
	fmt.Println("--- SELECT * FROM facts ---")
	qr := db.Query("SELECT * FROM facts")
	count := 0
	for {
		if _, ok := qr.Next(); !ok {
			break
		}
		count++
	}
	qr.Close()
	fmt.Printf("  Returned %d facts\n", count)

	// --- QUERY ALL CONVENIENCE ---
	fmt.Println("\n--- QueryAll() convenience method ---")
	allFacts, err := db.QueryAll()
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("  Returned %d facts\n", len(allFacts))
	for _, f := range allFacts {
		fmt.Printf("  Subject: %d, Predicate: %d, Object: %d, Confidence: %.2f\n",
			f.Subject, f.Predicate, f.Object, f.Confidence)
	}

	// --- FILTER BY SUBJECT ---
	fmt.Println("\n--- Filter by Subject = 1 ---")
	qr2 := db.Query("SELECT * FROM facts")
	subjectCount := 0
	for {
		fact, ok := qr2.Next()
		if !ok {
			break
		}
		if fact.Subject == 1 {
			fmt.Printf("  Subject: %d, Predicate: %d, Object: %d\n",
				fact.Subject, fact.Predicate, fact.Object)
			subjectCount++
		}
	}
	qr2.Close()
	fmt.Printf("  Found %d facts with subject=1\n", subjectCount)

	// --- FILTER BY PREDICATE ---
	fmt.Println("\n--- Filter by Predicate = 0 ---")
	qr3 := db.Query("SELECT * FROM facts")
	predCount := 0
	for {
		fact, ok := qr3.Next()
		if !ok {
			break
		}
		if fact.Predicate == 0 {
			fmt.Printf("  Subject: %d, Predicate: %d, Object: %d\n",
				fact.Subject, fact.Predicate, fact.Object)
			predCount++
		}
	}
	qr3.Close()
	fmt.Printf("  Found %d facts with predicate=0\n", predCount)

	// --- MULTI-CONDITION FILTER ---
	fmt.Println("\n--- Filter: subject=1 AND predicate=3 ---")
	qr4 := db.Query("SELECT * FROM facts")
	multiCount := 0
	for {
		fact, ok := qr4.Next()
		if !ok {
			break
		}
		if fact.Subject == 1 && fact.Predicate == 3 {
			fmt.Printf("  Subject: %d, Predicate: %d, Object: %d\n",
				fact.Subject, fact.Predicate, fact.Object)
			multiCount++
		}
	}
	qr4.Close()
	fmt.Printf("  Found %d facts matching multi-condition\n", multiCount)

	fmt.Println("\n=== All query patterns completed ===")
}
