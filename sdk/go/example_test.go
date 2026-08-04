package kcm

import (
	"fmt"
	"testing"
)

func TestExampleFullWorkflow(t *testing.T) {
	fmt.Println("=== Go SDK Example ===")

	// Create fact
	f1 := Fact{Subject: 1, Predicate: 0, Object: 2, Confidence: 0.95}
	f2 := Fact{Subject: 2, Predicate: 1, Object: 3, Confidence: 0.90}

	fmt.Printf("Fact 1: %+v\n", f1)
	fmt.Printf("Fact 2: %+v\n", f2)

	// Test roundtrip
	cf := f1.toC()
	f1back := factFromC(cf)
	if f1.Subject != f1back.Subject || f1.Confidence != f1back.Confidence {
		t.Error("Roundtrip failed")
	}
	fmt.Println("Fact roundtrip: OK")

	// Test error codes
	if OK != 0 {
		t.Error("OK should be 0")
	}
	if ErrNotFound != 1 {
		t.Error("ErrNotFound should be 1")
	}
	fmt.Println("Error codes: OK")

	// Test error messages
	msg := ErrNotFound.Error()
	if msg == "" {
		t.Error("Error message should not be empty")
	}
	fmt.Printf("Error message: %s\n", msg)

	fmt.Println("All Go SDK examples completed!")
}
