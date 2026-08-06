// Package kcm provides Go bindings for the KCM Knowledge Columnar Model.
//
// This SDK wraps the C FFI interface (18 functions) using CGo.
//
// Usage:
//
//	db, err := kcm.NewDatabase()
//	if err != nil {
//	    log.Fatal(err)
//	}
//	defer db.Close()
//
//	err = db.Insert(kcm.Fact{
//	    Subject:    1,
//	    Predicate:  0,
//	    Object:     2,
//	    Confidence: 0.95,
//	})
//	if err != nil {
//	    log.Fatal(err)
//	}
//
//	qr := db.Query("*")
//	defer qr.Close()
//	for {
//	    fact, ok := qr.Next()
//	    if !ok {
//	        break
//	    }
//	    fmt.Printf("Subject: %d, Object: %d, Confidence: %.2f\n", fact.Subject, fact.Object, fact.Confidence)
//	}
package kcm

/*
#cgo LDFLAGS: -lkcm
#include "kcm.h"
#include <stdlib.h>
*/
import "C"
import (
	"runtime"
	"unsafe"
)

// Error represents a KCM error code.
type Error int

const (
	OK                    Error = Error(C.KCM_OK)
	ErrNotFound           Error = Error(C.KCM_ERR_NOT_FOUND)
	ErrOutOfMemory        Error = Error(C.KCM_ERR_OUT_OF_MEMORY)
	ErrInvalidArgument    Error = Error(C.KCM_ERR_INVALID_ARGUMENT)
	ErrIO                 Error = Error(C.KCM_ERR_IO)
	ErrCorrupted          Error = Error(C.KCM_ERR_CORRUPTED)
	ErrConflict           Error = Error(C.KCM_ERR_CONFLICT)
	ErrTransactionAborted Error = Error(C.KCM_ERR_TRANSACTION_ABORTED)
)

func (e Error) Error() string {
	return C.GoString(C.KCM_ErrorMessage(C.KCM_Error(e)))
}

// Fact represents a knowledge fact with 10 attributes.
type Fact struct {
	Subject    uint32
	Predicate  uint8
	Object     uint32
	Confidence float64
	Evidence   uint8
	Timestamp  int64
	Context    uint8
	Version    int32
	Priority   int8
	Owner      uint16
}

func factToC(f Fact) C.KCM_Fact {
	return C.KCM_Fact{
		subject:    C.uint32_t(f.Subject),
		predicate:  C.uint8_t(f.Predicate),
		object:     C.uint32_t(f.Object),
		confidence: C.double(f.Confidence),
		evidence:   C.uint8_t(f.Evidence),
		timestamp:  C.int64_t(f.Timestamp),
		context:    C.uint8_t(f.Context),
		version:    C.int32_t(f.Version),
		priority:   C.int8_t(f.Priority),
		owner:      C.uint16_t(f.Owner),
	}
}

func factFromC(cf C.KCM_Fact) Fact {
	return Fact{
		Subject:    uint32(cf.subject),
		Predicate:  uint8(cf.predicate),
		Object:     uint32(cf.object),
		Confidence: float64(cf.confidence),
		Evidence:   uint8(cf.evidence),
		Timestamp:  int64(cf.timestamp),
		Context:    uint8(cf.context),
		Version:    int32(cf.version),
		Priority:   int8(cf.priority),
		Owner:      uint16(cf.owner),
	}
}

func checkErr(rc C.KCM_Error) error {
	if rc == C.KCM_OK {
		return nil
	}
	return Error(rc)
}

// Database wraps a KCM database handle.
type Database struct {
	db           *C.KCM_Database
	transactions []*Transaction
}

// NewDatabase creates a new in-memory database.
func NewDatabase() (*Database, error) {
	var db *C.KCM_Database
	rc := C.KCM_DatabaseNew(&db)
	if err := checkErr(rc); err != nil {
		return nil, err
	}
	d := &Database{db: db}
	runtime.SetFinalizer(d, (*Database).finalize)
	return d, nil
}

func (d *Database) finalize() {
	d.Close()
}

// Close frees the database resources and all owned transactions.
func (d *Database) Close() {
	if d.db != nil {
		for _, txn := range d.transactions {
			txn.Free()
		}
		d.transactions = nil
		C.KCM_DatabaseFree(d.db)
		d.db = nil
	}
}

// Insert adds a fact to the database.
func (d *Database) Insert(f Fact) error {
	cf := factToC(f)
	return checkErr(C.KCM_DatabaseInsert(d.db, &cf))
}

// Update modifies an existing fact by row ID.
func (d *Database) Update(rowID uint64, f Fact) error {
	cf := factToC(f)
	return checkErr(C.KCM_DatabaseUpdate(d.db, C.uint64_t(rowID), &cf))
}

// Delete removes a fact by row ID.
func (d *Database) Delete(rowID uint64) error {
	return checkErr(C.KCM_DatabaseDelete(d.db, C.uint64_t(rowID)))
}

// FactCount returns the total number of facts (including deleted).
func (d *Database) FactCount() uint64 {
	return uint64(C.KCM_DatabaseFactCount(d.db))
}

// ActiveFactCount returns the number of non-deleted facts.
func (d *Database) ActiveFactCount() uint64 {
	return uint64(C.KCM_DatabaseActiveCount(d.db))
}

// Query executes a KQL query string and returns a QueryResult iterator.
// The caller must call Close() on the returned QueryResult when done.
func (d *Database) Query(kql string) *QueryResult {
	ckql := C.CString(kql)
	defer C.free(unsafe.Pointer(ckql))
	q := C.KCM_DatabaseQuery(d.db, ckql)
	return &QueryResult{query: q}
}

// QueryAll is a convenience method that queries for all facts using "*".
// Returns all active facts as a slice.
func (d *Database) QueryAll() ([]Fact, error) {
	qr := d.Query("*")
	defer qr.Close()

	var facts []Fact
	for {
		fact, ok := qr.Next()
		if !ok {
			break
		}
		facts = append(facts, *fact)
	}
	return facts, nil
}

// BeginTransaction starts a new transaction.
func (d *Database) BeginTransaction() *Transaction {
	txn := C.KCM_DatabaseBeginTransaction(d.db)
	t := &Transaction{txn: txn, db: d}
	runtime.SetFinalizer(t, (*Transaction).finalize)
	d.transactions = append(d.transactions, t)
	return t
}

// Save writes the database to a file.
func (d *Database) Save(path string) error {
	cpath := C.CString(path)
	defer C.free(unsafe.Pointer(cpath))
	return checkErr(C.KCM_DatabaseSave(d.db, cpath))
}

// Load reads the database from a file into an existing database handle.
func (d *Database) Load(path string) error {
	cpath := C.CString(path)
	defer C.free(unsafe.Pointer(cpath))
	return checkErr(C.KCM_DatabaseLoad(d.db, cpath))
}

// Verify checks database file integrity.
func Verify(path string) error {
	cpath := C.CString(path)
	defer C.free(unsafe.Pointer(cpath))
	return checkErr(C.KCM_DatabaseVerify(cpath))
}

// QueryResult holds an iterator over query results.
type QueryResult struct {
	query *C.KCM_Query
}

// Next returns the next fact from the query result.
// Returns (nil, false) when no more results are available.
func (qr *QueryResult) Next() (*Fact, bool) {
	if qr.query == nil {
		return nil, false
	}
	cf := C.KCM_QueryNext(qr.query)
	if cf == nil {
		return nil, false
	}
	f := factFromC(*cf)
	return &f, true
}

// Close frees the query result handle.
func (qr *QueryResult) Close() {
	if qr.query != nil {
		C.KCM_QueryFree(qr.query)
		qr.query = nil
	}
}

// Transaction wraps a KCM transaction handle.
type Transaction struct {
	txn *C.KCM_Transaction
	db  *Database
}

func (t *Transaction) finalize() {
	t.Free()
}

// Commit finalizes the transaction, applying all changes.
func (t *Transaction) Commit() error {
	if t.txn == nil {
		return ErrTransactionAborted
	}
	err := checkErr(C.KCM_TransactionCommit(t.txn))
	t.txn = nil
	return err
}

// Rollback undoes the transaction.
func (t *Transaction) Rollback() error {
	if t.txn == nil {
		return nil
	}
	err := checkErr(C.KCM_TransactionRollback(t.txn))
	t.txn = nil
	return err
}

// Free releases the transaction resources.
// Safe to call multiple times. Also called automatically by the garbage collector.
func (t *Transaction) Free() {
	if t.txn != nil {
		C.KCM_TransactionFree(t.txn)
		t.txn = nil
	}
}
