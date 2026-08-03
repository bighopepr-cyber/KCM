# KCM Go SDK

Go bindings for KCM via CGo.

## Status: Planned

## Architecture

- CGo FFI to `kcm-interface` C API
- Package: `github.com/kcm/go-sdk`

## API Design

```go
package main

import (
    "fmt"
    "github.com/kcm/go-sdk"
)

func main() {
    db, err := kcm.NewDatabase("my_knowledge.db")
    if err != nil {
        panic(err)
    }
    defer db.Close()

    fact := kcm.Fact{
        Subject:    "planet",
        Predicate:  "orbits",
        Object:     "sun",
        Confidence: 0.99,
    }
    db.Insert(fact)

    results, _ := db.Query("SELECT * FROM facts WHERE subject = 'planet'")
    for _, r := range results {
        fmt.Println(r.Subject, r.Object)
    }
}
```

## Installation

```bash
go get github.com/kcm/go-sdk
```
