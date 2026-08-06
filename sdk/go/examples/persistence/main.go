package main

import (
	"fmt"
	"log"
	"os"
	"path/filepath"

	kcm "github.com/kcm-project/go-sdk"
)

// Persistence: save, load, and verify database persistence.
func main() {
	db, err := kcm.NewDatabase()
	if err != nil {
		log.Fatal(err)
	}
	defer db.Close()

	// Insert facts
	if err := db.Insert(kcm.Fact{Subject: 1, Predicate: 0, Object: 2, Confidence: 0.95,
		Evidence: 1, Context: 1, Version: 1, Owner: 1}); err != nil {
		log.Fatal(err)
	}
	if err := db.Insert(kcm.Fact{Subject: 2, Predicate: 1, Object: 3, Confidence: 0.90,
		Evidence: 2, Context: 1, Version: 1, Owner: 2}); err != nil {
		log.Fatal(err)
	}
	if err := db.Insert(kcm.Fact{Subject: 3, Predicate: 2, Object: 4, Confidence: 0.85,
		Evidence: 3, Context: 2, Version: 1, Owner: 3}); err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Facts: %d total, %d active\n\n", db.FactCount(), db.ActiveFactCount())

	// --- SAVE ---
	fmt.Println("--- Save Database ---")
	path := filepath.Join(os.TempDir(), "kcm_go_example.kcm")
	if err := db.Save(path); err != nil {
		log.Fatal(err)
	}
	fmt.Printf("  Saved to %s\n", path)
	defer os.Remove(path)

	// --- VERIFY ---
	fmt.Println("\n--- Verify Database ---")
	if err := kcm.Verify(path); err != nil {
		log.Fatal(err)
	}
	fmt.Println("  Verification passed")

	// --- LOAD ---
	fmt.Println("\n--- Load Database ---")
	db2, err := kcm.NewDatabase()
	if err != nil {
		log.Fatal(err)
	}
	defer db2.Close()

	if err := db2.Load(path); err != nil {
		log.Fatal(err)
	}
	fmt.Printf("  Loaded: %d total, %d active\n", db2.FactCount(), db2.ActiveFactCount())
	if db2.FactCount() != 3 {
		log.Fatal("Expected 3 facts after load")
	}

	// --- VERIFY DATA INTEGRITY ---
	fmt.Println("\n--- Verify Data Integrity ---")
	allFacts, err := db2.QueryAll()
	if err != nil {
		log.Fatal(err)
	}
	for _, f := range allFacts {
		fmt.Printf("  Subject: %d, Predicate: %d, Object: %d, Confidence: %.2f\n",
			f.Subject, f.Predicate, f.Object, f.Confidence)
	}
	if len(allFacts) != 3 {
		log.Fatal("Expected 3 active facts")
	}

	// --- SAVE-LOAD ROUND TRIP ---
	fmt.Println("\n--- Save-Load Round Trip ---")
	if err := db2.Insert(kcm.Fact{Subject: 10, Predicate: 0, Object: 20, Confidence: 0.99}); err != nil {
		log.Fatal(err)
	}
	if err := db2.Save(path); err != nil {
		log.Fatal(err)
	}
	db3, err := kcm.NewDatabase()
	if err != nil {
		log.Fatal(err)
	}
	defer db3.Close()

	if err := db3.Load(path); err != nil {
		log.Fatal(err)
	}
	fmt.Printf("  Round-trip: %d total, %d active\n", db3.FactCount(), db3.ActiveFactCount())
	if db3.FactCount() != 4 {
		log.Fatal("Expected 4 facts after round-trip")
	}

	fmt.Println("\n=== All persistence operations completed ===")
}
