package kcm

import (
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
	f := Fact{Subject: 42, Predicate: 5, Object: 100, Confidence: 0.75}
	cf := f.toC()
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
	if f.Confidence != f2.Confidence {
		t.Errorf("Confidence mismatch: %f != %f", f.Confidence, f2.Confidence)
	}
}

func TestErrorCodes(t *testing.T) {
	if OK != 0 {
		t.Error("OK should be 0")
	}
	if ErrNotFound != 1 {
		t.Error("ErrNotFound should be 1")
	}
	if ErrInvalidArgument != 3 {
		t.Error("ErrInvalidArgument should be 3")
	}
}

func TestErrorMessages(t *testing.T) {
	msg := ErrNotFound.Error()
	if msg == "" {
		t.Error("Error message should not be empty")
	}
}
