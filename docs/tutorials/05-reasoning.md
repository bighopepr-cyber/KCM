# Tutorial 05: Reasoning with Rules

## Objective

Learn how to use KCM's inference engine to derive new knowledge.

## What is Reasoning?

KCM can automatically derive new facts from existing facts using rules. This is called inference.

## Rule Syntax

### Basic Rule

```rust
Rule::new(
    "derive_grandparent",
    RulePattern::subject_predicate_object(
        Some(Variable::Subject),   // parent
        PredicateID(2),            // has_child
        Some(Variable::Object),    // child
    ),
    PredicateID(3),  // has_grandparent
    0.9,             // confidence
)
```

### Chained Rule

```rust
Rule::new(
    "derive_ancestor",
    RulePattern::and(vec![
        RulePattern::subject_predicate_object(
            Some(Variable::X),
            PredicateID(3),  // has_grandparent
            Some(Variable::Y),
        ),
    ]),
    PredicateID(4),  // has_ancestor
    0.85,
)
```

## Running Inference

```rust
let engine = InferenceEngine::new(rules);
engine.infer(&mut schema, max_iterations)?;
```

## Example

Given facts:
- bob has_child alice
- alice has_child charlie

Rule: If X has_child Y and Y has_child Z, then X has_grandparent Z.

Derived fact:
- bob has_grandparent charlie (confidence: 0.9)

## Next Steps

- Tutorial 06: Performance optimization
