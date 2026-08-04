//! Reasoning example
//! 
//! This example demonstrates:
//! - Defining rules
//! - Running inference
//! - Deriving new facts

use kcm_core::types::*;
use kcm_reasoning::rule::{Rule, RulePattern};
use kcm_reasoning::inference::InferenceEngine;
use kcm_storage::column::Schema;
use anyhow::Result;

fn main() -> Result<()> {
    // Create a schema with some facts
    let mut schema = Schema::new(100)?;
    
    // Add base facts: parent relationships
    schema.append_fact(&Fact::new(SubjectID(1), PredicateID(10), ObjectID(2), 1.0)?)?; // bob is parent of alice
    schema.append_fact(&Fact::new(SubjectID(2), PredicateID(10), ObjectID(3), 1.0)?)?; // alice is parent of charlie
    
    println!("Base facts:");
    println!("  bob (1) is parent (10) of alice (2)");
    println!("  alice (2) is parent (10) of charlie (3)");
    
    // Define a rule: if X is parent of Y and Y is parent of Z, then X is grandparent of Z
    let rule = Rule::new(
        1, // rule id
        "grandparent_rule".to_string(),
        RulePattern::subject_predicate_object(
            None,
            PredicateID(10),
            None,
        ),
        PredicateID(20), // grandparent
        Box::new(|confidences: &[f64]| {
            if confidences.is_empty() {
                0.0
            } else {
                confidences[0] * 0.9
            }
        }),
    );
    
    // Create inference engine and register the rule
    let mut engine = InferenceEngine::new();
    engine.register_rule(rule)?;
    
    // Run inference
    let (derived, _stats) = engine.infer_with_stats(&mut schema)?;
    
    println!("\nDerived facts:");
    for derivation in &derived {
        let fact = &derivation.derived_fact;
        println!("  subject={}, predicate={}, object={}, confidence={}",
            fact.subject.0, fact.predicate.0, fact.object.0, fact.confidence);
    }
    
    println!("\nTotal facts after inference: {}", schema.len());
    
    Ok(())
}
