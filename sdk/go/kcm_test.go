package kcm

import (
	"math"
	"os"
	"testing"
)

func TestFactCreation(t *testing.T) {
	f := Fact{Subject: 1, Predicate: 0, Object: 2, Confidence: 0.95}
	if f.Subject != 1 {
		t.Errorf("Expected subject 1, got %d", f.Subject)
	}
	if f.Confidence != 0.95 {
		t.Errorf("Expected confidence 0.95, got %f", f.Confidence)
	}
}

func TestFactRoundtrip(t *testing.T) {
	f := Fact{
		Subject:    42,
		Predicate:  5,
		Object:     100,
		Confidence: 0.75,
		Evidence:   3,
		Timestamp:  1700000000000000000,
		Context:    2,
		Version:    7,
		Priority:   -1,
		Owner:      15,
	}
	cf := factToC(f)
	f2 := factFromC(cf)

	if f.Subject != f2.Subject {
		t.Errorf("Subject mismatch: %d != %d", f.Subject, f2.Subject)
	}
	if f.Predicate != f2.Predicate {
		t.Errorf("Predicate mismatch: %d != %d", f.Predicate, f2.Predicate)
	}
	if f.Object != f2.Object {
		t.Errorf("Object mismatch: %d != %d", f.Object, f2.Object)
	}
	if math.Abs(f.Confidence-f2.Confidence) > 1e-10 {
		t.Errorf("Confidence mismatch: %f != %f", f.Confidence, f2.Confidence)
	}
	if f.Evidence != f2.Evidence {
		t.Errorf("Evidence mismatch: %d != %d", f.Evidence, f2.Evidence)
	}
	if f.Timestamp != f2.Timestamp {
		t.Errorf("Timestamp mismatch: %d != %d", f.Timestamp, f2.Timestamp)
	}
	if f.Context != f2.Context {
		t.Errorf("Context mismatch: %d != %d", f.Context, f2.Context)
	}
	if f.Version != f2.Version {
		t.Errorf("Version mismatch: %d != %d", f.Version, f2.Version)
	}
	if f.Priority != f2.Priority {
		t.Errorf("Priority mismatch: %d != %d", f.Priority, f2.Priority)
	}
	if f.Owner != f2.Owner {
		t.Errorf("Owner mismatch: %d != %d", f.Owner, f2.Owner)
	}
}

func TestErrorCodes(t *testing.T) {
	if OK != 0 {
		t.Error("OK should be 0")
	}
	if ErrNotFound != 1 {
		t.Error("ErrNotFound should be 1")
	}
	if ErrOutOfMemory != 2 {
		t.Error("ErrOutOfMemory should be 2")
	}
	if ErrInvalidArgument != 3 {
		t.Error("ErrInvalidArgument should be 3")
	}
	if ErrIO != 4 {
		t.Error("ErrIO should be 4")
	}
	if ErrCorrupted != 5 {
		t.Error("ErrCorrupted should be 5")
	}
	if ErrConflict != 6 {
		t.Error("ErrConflict should be 6")
	}
	if ErrTransactionAborted != 7 {
		t.Error("ErrTransactionAborted should be 7")
	}
}

func TestErrorMessages(t *testing.T) {
	tests := []struct {
		err  Error
		name string
	}{
		{OK, "OK"},
		{ErrNotFound, "NotFound"},
		{ErrInvalidArgument, "InvalidArgument"},
		{ErrIO, "IO"},
		{ErrCorrupted, "Corrupted"},
		{ErrConflict, "Conflict"},
		{ErrTransactionAborted, "TransactionAborted"},
	}
	for _, tt := range tests {
		msg := tt.err.Error()
		if msg == "" {
			t.Errorf("Error(%s) message should not be empty", tt.name)
		}
	}
}

func TestNewDatabase(t *testing.T) {
	db, err := NewDatabase()
	if err != nil {
		t.Fatalf("NewDatabase failed: %v", err)
	}
	defer db.Close()

	if db.db == nil {
		t.Error("Database handle should not be nil")
	}
	if db.FactCount() != 0 {
		t.Errorf("Expected 0 facts, got %d", db.FactCount())
	}
}

func TestDatabaseClose(t *testing.T) {
	db, err := NewDatabase()
	if err != nil {
		t.Fatalf("NewDatabase failed: %v", err)
	}
	db.Close()
	if db.db != nil {
		t.Error("Database handle should be nil after Close")
	}
	// Double close should be safe
	db.Close()
}

func TestDatabaseFinalizer(t *testing.T) {
	db, err := NewDatabase()
	if err != nil {
		t.Fatalf("NewDatabase failed: %v", err)
	}
	// Clear finalizer so Close() is deterministic in test
	// db.Close() will be called by GC or by explicit call
	_ = db
}

func TestInsert(t *testing.T) {
	db, err := NewDatabase()
	if err != nil {
		t.Fatalf("NewDatabase failed: %v", err)
	}
	defer db.Close()

	f := Fact{Subject: 1, Predicate: 0, Object: 2, Confidence: 0.95}
	if err := db.Insert(f); err != nil {
		t.Fatalf("Insert failed: %v", err)
	}
	if db.FactCount() != 1 {
		t.Errorf("Expected 1 fact, got %d", db.FactCount())
	}
	if db.ActiveFactCount() != 1 {
		t.Errorf("Expected 1 active fact, got %d", db.ActiveFactCount())
	}
}

func TestInsertMultiple(t *testing.T) {
	db, err := NewDatabase()
	if err != nil {
		t.Fatalf("NewDatabase failed: %v", err)
	}
	defer db.Close()

	for i := uint32(0); i < 10; i++ {
		f := Fact{Subject: i, Predicate: 0, Object: i + 100, Confidence: 0.5 + float64(i)*0.05}
		if err := db.Insert(f); err != nil {
			t.Fatalf("Insert %d failed: %v", i, err)
		}
	}
	if db.FactCount() != 10 {
		t.Errorf("Expected 10 facts, got %d", db.FactCount())
	}
}

func TestQueryAll(t *testing.T) {
	db, err := NewDatabase()
	if err != nil {
		t.Fatalf("NewDatabase failed: %v", err)
	}
	defer db.Close()

	f1 := Fact{Subject: 1, Predicate: 0, Object: 2, Confidence: 0.95}
	f2 := Fact{Subject: 3, Predicate: 1, Object: 4, Confidence: 0.80}
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
		t.Fatalf("Expected 2 facts, got %d", len(facts))
	}

	found1, found2 := false, false
	for _, f := range facts {
		if f.Subject == 1 && f.Object == 2 {
			found1 = true
		}
		if f.Subject == 3 && f.Object == 4 {
			found2 = true
		}
	}
	if !found1 {
		t.Error("Fact 1 not found in results")
	}
	if !found2 {
		t.Error("Fact 2 not found in results")
	}
}

func TestQueryAllEmpty(t *testing.T) {
	db, err := NewDatabase()
	if err != nil {
		t.Fatalf("NewDatabase failed: %v", err)
	}
	defer db.Close()

	facts, err := db.QueryAll()
	if err != nil {
		t.Fatalf("QueryAll failed: %v", err)
	}
	if len(facts) != 0 {
		t.Errorf("Expected 0 facts, got %d", len(facts))
	}
}

func TestQueryKQL(t *testing.T) {
	db, err := NewDatabase()
	if err != nil {
		t.Fatalf("NewDatabase failed: %v", err)
	}
	defer db.Close()

	f := Fact{Subject: 1, Predicate: 0, Object: 2, Confidence: 0.95}
	if err := db.Insert(f); err != nil {
		t.Fatalf("Insert failed: %v", err)
	}

	qr := db.Query("SELECT * FROM facts")
	defer qr.Close()

	count := 0
	for {
		_, ok := qr.Next()
		if !ok {
			break
		}
		count++
	}
	if count != 1 {
		t.Errorf("Expected 1 fact, got %d", count)
	}
}

func TestQueryResultClose(t *testing.T) {
	db, err := NewDatabase()
	if err != nil {
		t.Fatalf("NewDatabase failed: %v", err)
	}
	defer db.Close()

	f := Fact{Subject: 1, Predicate: 0, Object: 2, Confidence: 0.95}
	if err := db.Insert(f); err != nil {
		t.Fatalf("Insert failed: %v", err)
	}

	qr := db.Query("*")
	qr.Close()

	// Should be safe to call Close multiple times
	qr.Close()

	// Next after close should return false
	_, ok := qr.Next()
	if ok {
		t.Error("Next() should return false after Close()")
	}
}

func TestQueryAllFindsAll(t *testing.T) {
	db, err := NewDatabase()
	if err != nil {
		t.Fatalf("NewDatabase failed: %v", err)
	}
	defer db.Close()

	for i := uint32(0); i < 5; i++ {
		f := Fact{Subject: i, Predicate: uint8(i), Object: i + 10, Confidence: float64(i) * 0.2}
		if err := db.Insert(f); err != nil {
			t.Fatalf("Insert %d failed: %v", i, err)
		}
	}

	facts, err := db.QueryAll()
	if err != nil {
		t.Fatalf("QueryAll failed: %v", err)
	}
	if len(facts) != 5 {
		t.Errorf("Expected 5 facts, got %d", len(facts))
	}
}

func TestUpdate(t *testing.T) {
	db, err := NewDatabase()
	if err != nil {
		t.Fatalf("NewDatabase failed: %v", err)
	}
	defer db.Close()

	f := Fact{Subject: 1, Predicate: 0, Object: 2, Confidence: 0.5}
	if err := db.Insert(f); err != nil {
		t.Fatalf("Insert failed: %v", err)
	}

	f2 := Fact{Subject: 1, Predicate: 0, Object: 3, Confidence: 0.99}
	if err := db.Update(0, f2); err != nil {
		t.Fatalf("Update failed: %v", err)
	}

	facts, err := db.QueryAll()
	if err != nil {
		t.Fatalf("QueryAll failed: %v", err)
	}
	if len(facts) != 1 {
		t.Fatalf("Expected 1 fact, got %d", len(facts))
	}
	if facts[0].Object != 3 {
		t.Errorf("Expected Object=3 after update, got %d", facts[0].Object)
	}
	if math.Abs(facts[0].Confidence-0.99) > 1e-10 {
		t.Errorf("Expected Confidence=0.99 after update, got %f", facts[0].Confidence)
	}
}

func TestDelete(t *testing.T) {
	db, err := NewDatabase()
	if err != nil {
		t.Fatalf("NewDatabase failed: %v", err)
	}
	defer db.Close()

	f := Fact{Subject: 1, Predicate: 0, Object: 2, Confidence: 0.95}
	if err := db.Insert(f); err != nil {
		t.Fatalf("Insert failed: %v", err)
	}

	if err := db.Delete(0); err != nil {
		t.Fatalf("Delete failed: %v", err)
	}

	if db.ActiveFactCount() != 0 {
		t.Errorf("Expected 0 active facts after delete, got %d", db.ActiveFactCount())
	}
	// Total count should still be 1
	if db.FactCount() != 1 {
		t.Errorf("Expected 1 total fact (tombstone), got %d", db.FactCount())
	}
}

func TestTransactionCommit(t *testing.T) {
	db, err := NewDatabase()
	if err != nil {
		t.Fatalf("NewDatabase failed: %v", err)
	}
	defer db.Close()

	txn := db.BeginTransaction()
	f := Fact{Subject: 1, Predicate: 0, Object: 2, Confidence: 0.95}
	if err := db.Insert(f); err != nil {
		t.Fatalf("Insert failed: %v", err)
	}

	if err := txn.Commit(); err != nil {
		t.Fatalf("Commit failed: %v", err)
	}

	facts, err := db.QueryAll()
	if err != nil {
		t.Fatalf("QueryAll failed: %v", err)
	}
	if len(facts) != 1 {
		t.Errorf("Expected 1 fact after commit, got %d", len(facts))
	}
}

func TestTransactionRollback(t *testing.T) {
	db, err := NewDatabase()
	if err != nil {
		t.Fatalf("NewDatabase failed: %v", err)
	}
	defer db.Close()

	txn := db.BeginTransaction()
	f := Fact{Subject: 1, Predicate: 0, Object: 2, Confidence: 0.95}
	if err := db.Insert(f); err != nil {
		t.Fatalf("Insert failed: %v", err)
	}

	if err := txn.Rollback(); err != nil {
		t.Fatalf("Rollback failed: %v", err)
	}

	facts, err := db.QueryAll()
	if err != nil {
		t.Fatalf("QueryAll failed: %v", err)
	}
	if len(facts) != 0 {
		t.Errorf("Expected 0 facts after rollback, got %d", len(facts))
	}
}

func TestTransactionFree(t *testing.T) {
	db, err := NewDatabase()
	if err != nil {
		t.Fatalf("NewDatabase failed: %v", err)
	}
	defer db.Close()

	txn := db.BeginTransaction()
	txn.Free()

	// Double free should be safe
	txn.Free()
}

func TestTransactionCommitNil(t *testing.T) {
	txn := &Transaction{txn: nil}
	err := txn.Commit()
	if err != ErrTransactionAborted {
		t.Errorf("Expected ErrTransactionAborted, got %v", err)
	}
}

func TestTransactionRollbackNil(t *testing.T) {
	txn := &Transaction{txn: nil}
	err := txn.Rollback()
	if err != nil {
		t.Errorf("Expected nil error, got %v", err)
	}
}

func TestSaveLoadVerify(t *testing.T) {
	db, err := NewDatabase()
	if err != nil {
		t.Fatalf("NewDatabase failed: %v", err)
	}
	defer db.Close()

	f := Fact{Subject: 1, Predicate: 0, Object: 2, Confidence: 0.95}
	if err := db.Insert(f); err != nil {
		t.Fatalf("Insert failed: %v", err)
	}

	tmpFile, err := os.CreateTemp("", "kcm_test_*.kcm")
	if err != nil {
		t.Fatalf("CreateTemp failed: %v", err)
	}
	tmpFile.Close()
	defer os.Remove(tmpFile.Name())

	if err := db.Save(tmpFile.Name()); err != nil {
		t.Fatalf("Save failed: %v", err)
	}

	if err := Verify(tmpFile.Name()); err != nil {
		t.Fatalf("Verify failed: %v", err)
	}

	db2, err := NewDatabase()
	if err != nil {
		t.Fatalf("NewDatabase failed: %v", err)
	}
	defer db2.Close()

	if err := db2.Load(tmpFile.Name()); err != nil {
		t.Fatalf("Load failed: %v", err)
	}

	if db2.FactCount() != 1 {
		t.Errorf("Expected 1 fact after load, got %d", db2.FactCount())
	}

	facts, err := db2.QueryAll()
	if err != nil {
		t.Fatalf("QueryAll failed: %v", err)
	}
	if len(facts) != 1 {
		t.Fatalf("Expected 1 fact after load, got %d", len(facts))
	}
	if facts[0].Subject != 1 || facts[0].Object != 2 {
		t.Errorf("Loaded fact mismatch: %+v", facts[0])
	}
}

func TestVerifyInvalidPath(t *testing.T) {
	err := Verify("/nonexistent/path/to/file.kcm")
	if err == nil {
		t.Error("Verify should fail for nonexistent file")
	}
}

func TestAllFactFields(t *testing.T) {
	db, err := NewDatabase()
	if err != nil {
		t.Fatalf("NewDatabase failed: %v", err)
	}
	defer db.Close()

	f := Fact{
		Subject:    42,
		Predicate:  7,
		Object:     99,
		Confidence: 0.42,
		Evidence:   3,
		Timestamp:  1700000000000000000,
		Context:    5,
		Version:    11,
		Priority:   -3,
		Owner:      255,
	}
	if err := db.Insert(f); err != nil {
		t.Fatalf("Insert failed: %v", err)
	}

	facts, err := db.QueryAll()
	if err != nil {
		t.Fatalf("QueryAll failed: %v", err)
	}
	if len(facts) != 1 {
		t.Fatalf("Expected 1 fact, got %d", len(facts))
	}

	got := facts[0]
	if got.Subject != 42 {
		t.Errorf("Subject: got %d, want 42", got.Subject)
	}
	if got.Predicate != 7 {
		t.Errorf("Predicate: got %d, want 7", got.Predicate)
	}
	if got.Object != 99 {
		t.Errorf("Object: got %d, want 99", got.Object)
	}
	if math.Abs(got.Confidence-0.42) > 1e-10 {
		t.Errorf("Confidence: got %f, want 0.42", got.Confidence)
	}
	if got.Evidence != 3 {
		t.Errorf("Evidence: got %d, want 3", got.Evidence)
	}
	if got.Timestamp != 1700000000000000000 {
		t.Errorf("Timestamp: got %d, want 1700000000000000000", got.Timestamp)
	}
	if got.Context != 5 {
		t.Errorf("Context: got %d, want 5", got.Context)
	}
	if got.Version != 11 {
		t.Errorf("Version: got %d, want 11", got.Version)
	}
	if got.Priority != -3 {
		t.Errorf("Priority: got %d, want -3", got.Priority)
	}
	if got.Owner != 255 {
		t.Errorf("Owner: got %d, want 255", got.Owner)
	}
}
