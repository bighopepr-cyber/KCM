package main

import (
	"fmt"
	"log"

	kcm "github.com/kcm-project/go-sdk"
)

// Error Handling: proper error handling patterns.
func main() {
	db, err := kcm.NewDatabase()
	if err != nil {
		log.Fatal(err)
	}
	defer db.Close()

	// --- NOT FOUND (update non-existent row) ---
	fmt.Println("--- Not Found (update non-existent row) ---")
	err = db.Update(99999, kcm.Fact{
		Subject: 1, Predicate: 0, Object: 2, Confidence: 0.5,
	})
	if err != nil {
		fmt.Printf("  Caught error: %v\n", err)
		fmt.Printf("  Error type: %T\n", err)
	}

	// --- NOT FOUND (delete non-existent row) ---
	fmt.Println("\n--- Not Found (delete non-existent row) ---")
	err = db.Delete(99999)
	if err != nil {
		fmt.Printf("  Caught error: %v\n", err)
	} else {
		fmt.Println("  Delete returned nil (success)")
	}

	// --- ERROR CODES ---
	fmt.Println("\n--- Error Codes ---")
	errorCodes := []struct {
		code kcm.Error
		name string
	}{
		{kcm.OK, "OK"},
		{kcm.ErrNotFound, "NOT_FOUND"},
		{kcm.ErrOutOfMemory, "OUT_OF_MEMORY"},
		{kcm.ErrInvalidArgument, "INVALID_ARGUMENT"},
		{kcm.ErrIO, "IO"},
		{kcm.ErrCorrupted, "CORRUPTED"},
		{kcm.ErrConflict, "CONFLICT"},
		{kcm.ErrTransactionAborted, "TRANSACTION_ABORTED"},
	}
	for _, ec := range errorCodes {
		fmt.Printf("  %s: %s\n", ec.name, ec.code.Error())
	}

	// --- INVALID FACT ---
	fmt.Println("\n--- Invalid Confidence (out of range) ---")
	err = db.Insert(kcm.Fact{
		Subject: 1, Predicate: 0, Object: 2, Confidence: 1.5,
	})
	if err != nil {
		fmt.Printf("  Caught error: %v\n", err)
	}

	// --- TRY-CATCH PATTERN ---
	fmt.Println("\n--- Try-Catch Pattern ---")
	err = db.Insert(kcm.Fact{Subject: 1, Predicate: 0, Object: 2, Confidence: 0.95})
	if err != nil {
		fmt.Printf("  Database error: %v\n", err)
		return
	}
	err = db.Insert(kcm.Fact{Subject: 2, Predicate: 1, Object: 3, Confidence: 0.90})
	if err != nil {
		fmt.Printf("  Database error: %v\n", err)
		return
	}
	qr := db.Query("SELECT * FROM facts")
	count := 0
	for {
		if _, ok := qr.Next(); !ok {
			break
		}
		count++
	}
	qr.Close()
	fmt.Printf("  Query returned %d results\n", count)

	// --- SAVE ERROR (file path) ---
	fmt.Println("\n--- Save to Invalid Path ---")
	err = db.Save("/nonexistent/dir/db.kcm")
	if err != nil {
		fmt.Printf("  Caught error: %v\n", err)
	}

	// --- VERIFY ERROR (non-existent file) ---
	fmt.Println("\n--- Verify Non-Existent File ---")
	err = kcm.Verify("/nonexistent/path/db.kcm")
	if err != nil {
		fmt.Printf("  Caught error: %v\n", err)
	}

	fmt.Println("\n=== All error handling patterns completed ===")
}
