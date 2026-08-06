package kcm

import (
	"testing"
)

func TestExampleFullWorkflow(t *testing.T) {
	db, err := NewDatabase()
	if err != nil {
		t.Fatalf("NewDatabase failed: %v", err)
	}
	defer db.Close()

	f1 := Fact{Subject: 1, Predicate: 0, Object: 2, Confidence: 0.95}
	f2 := Fact{Subject: 2, Predicate: 1, Object: 3, Confidence: 0.90}

	if err := db.Insert(f1); err != nil {
		t.Fatalf("Insert f1 failed: %v", err)
	}
	if err := db.Insert(f2); err != nil {
		t.Fatalf("Insert f2 failed: %v", err)
	}

	facts, err := db.QueryAll()
	if err != nil {
		t.Fatalf("QueryAll failed: %v", err)
	}
	if len(facts) != 2 {
		t.Errorf("Expected 2 facts, got %d", len(facts))
	}

	qr := db.Query("SELECT * FROM facts")
	count := 0
	for {
		_, ok := qr.Next()
		if !ok {
			break
		}
		count++
	}
	qr.Close()
	if count != 2 {
		t.Errorf("Query iterator expected 2, got %d", count)
	}
}
