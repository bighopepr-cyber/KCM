package main

import (
	"fmt"
	"log"

	kcm "github.com/kcm-project/go-sdk"
)

func main() {
	db, err := kcm.NewDatabase()
	if err != nil {
		log.Fatal(err)
	}
	defer db.Close()

	facts := []kcm.Fact{
		{Subject: 1, Predicate: 0, Object: 2, Confidence: 0.95, Evidence: 1, Context: 1, Version: 1, Owner: 1},
		{Subject: 2, Predicate: 1, Object: 3, Confidence: 0.90, Evidence: 1, Context: 1, Version: 1, Owner: 1},
		{Subject: 3, Predicate: 0, Object: 4, Confidence: 0.85, Evidence: 2, Context: 2, Version: 1, Owner: 2},
	}

	for _, f := range facts {
		if err := db.Insert(f); err != nil {
			log.Fatal(err)
		}
	}
	fmt.Printf("Inserted %d facts\n", db.FactCount())
	fmt.Printf("Active facts: %d\n", db.ActiveFactCount())

	fmt.Println("\n--- Query with QueryAll ---")
	allFacts, err := db.QueryAll()
	if err != nil {
		log.Fatal(err)
	}
	for _, f := range allFacts {
		fmt.Printf("  Subject: %d, Predicate: %d, Object: %d, Confidence: %.2f\n",
			f.Subject, f.Predicate, f.Object, f.Confidence)
	}

	fmt.Println("\n--- Query with KQL ---")
	qr := db.Query("SELECT * FROM facts")
	for {
		fact, ok := qr.Next()
		if !ok {
			break
		}
		fmt.Printf("  Subject: %d, Predicate: %d, Object: %d, Confidence: %.2f\n",
			fact.Subject, fact.Predicate, fact.Object, fact.Confidence)
	}
	qr.Close()

	fmt.Println("\n--- Transaction (commit) ---")
	txn := db.BeginTransaction()
	if err := db.Insert(kcm.Fact{
		Subject: 4, Predicate: 2, Object: 5, Confidence: 0.70, Evidence: 1, Context: 1, Version: 1, Owner: 3,
	}); err != nil {
		log.Fatal(err)
	}
	if err := txn.Commit(); err != nil {
		log.Fatal(err)
	}
	fmt.Printf("After commit: %d facts\n", db.ActiveFactCount())

	fmt.Println("\n--- Transaction (rollback) ---")
	txn2 := db.BeginTransaction()
	if err := db.Insert(kcm.Fact{
		Subject: 5, Predicate: 3, Object: 6, Confidence: 0.60, Evidence: 1, Context: 1, Version: 1, Owner: 4,
	}); err != nil {
		log.Fatal(err)
	}
	if err := txn2.Rollback(); err != nil {
		log.Fatal(err)
	}
	fmt.Printf("After rollback: %d facts\n", db.ActiveFactCount())

	fmt.Println("\n--- Save/Load/Verify ---")
	if err := db.Save("example.kcm"); err != nil {
		log.Fatal(err)
	}
	fmt.Println("Database saved")

	if err := kcm.Verify("example.kcm"); err != nil {
		log.Fatal(err)
	}
	fmt.Println("Database verified")

	db2, err := kcm.NewDatabase()
	if err != nil {
		log.Fatal(err)
	}
	defer db2.Close()

	if err := db2.Load("example.kcm"); err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Database loaded: %d facts\n", db2.FactCount())

	fmt.Println("\nAll operations completed successfully")
}
