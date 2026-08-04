use kcm_core::types::*;
use kcm_reasoning::rule::{Rule, RulePattern, RuleRegistry};
use kcm_reasoning::inference::InferenceEngine;
use kcm_storage::column::Schema;
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== KCM Reasoning Engine ===\n");

    let mut schema = Schema::new(100)?;
    schema.append_fact(&Fact::new(SubjectID(1), PredicateID(10), ObjectID(2), 1.0)?)?;
    schema.append_fact(&Fact::new(SubjectID(2), PredicateID(10), ObjectID(3), 1.0)?)?;

    println!("Base facts:");
    println!("  Subject 1 --predicate 10--> Subject 2");
    println!("  Subject 2 --predicate 10--> Subject 3");

    let rule = Rule::new(
        1,
        "transitive_rule".to_string(),
        RulePattern::subject_predicate_object(None, PredicateID(10), None),
        PredicateID(20),
        Box::new(|confidences: &[f64]| {
            if confidences.is_empty() { 0.0 } else { confidences[0] * 0.9 }
        }),
    );

    let mut engine = InferenceEngine::new();
    engine.register_rule(rule)?;

    let (derived, stats) = engine.infer_with_stats(&mut schema)?;

    println!("\nDerived facts:");
    for d in &derived {
        println!("  subject={} predicate={} confidence={:.2}",
            d.derived_fact.subject.0, d.derived_fact.predicate.0, d.derived_fact.confidence);
    }
    println!("\nStats: iterations={}, facts_derived={}", stats.iterations, stats.facts_derived);
    println!("Total facts after inference: {}", schema.len());
    Ok(())
}
