# kcm-reasoning

Forward-chaining inference engine and rule definitions for KCM.

## Purpose

Implements a rule-based reasoning engine that derives new facts from existing knowledge using forward-chaining inference. Rules are defined as condition-action pairs over the knowledge base.

## Modules

| Module | Purpose |
|--------|---------|
| `rule` | Rule definitions (conditions, actions, priorities) |
| `inference` | Forward-chaining inference engine |

## Dependencies

| Dependency | Purpose |
|------------|---------|
| `kcm-core` | Core types |
| `kcm-storage` | Knowledge base access |

## Inference Model

Forward-chaining cycle:
1. Match all rules against current facts
2. Select highest-priority rule
3. Execute rule action (derive new facts)
4. Repeat until no new facts are derived

```rust
pub trait Rule: Send + Sync {
    fn matches(&self, facts: &[Fact]) -> bool;
    fn execute(&self, facts: &[Fact]) -> Vec<Fact>;
    fn priority(&self) -> u32;
}
```

## Usage

```rust
use kcm_reasoning::rule::{Rule, RuleSet};
use kcm_reasoning::inference::InferenceEngine;

let rules = RuleSet::new();
rules.add(Box::new(MyRule::new()));

let engine = InferenceEngine::new(rules);
let new_facts = engine.run(&existing_facts)?;
```

## Termination

The inference engine terminates when:
- No rules match (fixpoint reached)
- Maximum iterations exceeded (configurable, default 1000)
- All derived facts are duplicates
