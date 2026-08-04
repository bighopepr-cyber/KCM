# Tutorial 03: Basic Queries

## Objective

Learn the KQL (KCM Query Language) syntax for querying knowledge facts.

## KQL Syntax

### SELECT

```sql
-- Select all facts
SELECT * FROM facts;

-- Select specific columns
SELECT subject, object FROM facts;

-- With WHERE clause
SELECT * FROM facts WHERE subject = 'planet';

-- With confidence threshold
SELECT * FROM facts WHERE confidence > 0.9;

-- With LIMIT
SELECT * FROM facts LIMIT 10;
```

### INSERT

```sql
INSERT INTO facts (subject, predicate, object, confidence)
VALUES ('star', 'belongs_to', 'galaxy', 0.95);
```

### DELETE

```sql
DELETE FROM facts WHERE subject = 'planet' AND confidence < 0.5;
```

### Aggregate

```sql
-- Count facts
SELECT COUNT(*) FROM facts;

-- Average confidence
SELECT AVG(confidence) FROM facts;

-- Group by subject
SELECT subject, COUNT(*) FROM facts GROUP BY subject;
```

## Examples

### Find High-Confidence Facts

```sql
SELECT * FROM facts WHERE confidence > 0.95 ORDER BY confidence DESC;
```

### Find Related Facts

```sql
SELECT * FROM facts WHERE subject = 'planet' OR object = 'planet';
```

## Next Steps

- Tutorial 04: Learn about transactions
