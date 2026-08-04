# Tutorial 02: First Database

## Objective

Create a KCM database, insert facts, and query them.

## Prerequisites

- Completed Tutorial 01

## Steps

### Step 1: Start the Server

```bash
./target/release/kcm-server --db my_knowledge.db
```

Server starts on http://localhost:8080.

### Step 2: Insert a Fact

```bash
curl -X POST http://localhost:8080/api/facts \
  -H "Content-Type: application/json" \
  -d '{
    "subject": "planet",
    "predicate": "orbits",
    "object": "sun",
    "confidence": 0.99
  }'
```

### Step 3: Query Facts

```bash
curl "http://localhost:8080/api/query?kql=SELECT * FROM facts"
```

### Step 4: Verify

```bash
curl "http://localhost:8080/api/stats"
```

Returns:
```json
{
  "fact_count": 1,
  "active_count": 1
}
```

## What You Learned

- How to start KCM server
- How to insert facts via REST API
- How to query using KQL
- How to check database statistics

## Next Steps

- Tutorial 03: Learn KQL query language
