use kcm_core::types::*;
use kcm_reasoning::inference::InferenceEngine;
use kcm_reasoning::rule::{Rule, RulePattern};
use kcm_storage::column::Schema;

fn setup_schema_with_facts() -> Schema {
    let mut schema = Schema::new(10_000).unwrap();
    for i in 0..5u32 {
        schema
            .append_fact(&Fact::new(SubjectID(i), PredicateID(0), ObjectID(i + 10), 0.8).unwrap())
            .unwrap();
    }
    schema
}

#[test]
fn test_derivation_has_rule_id() {
    let mut engine = InferenceEngine::new();
    let rule = Rule::new(
        1,
        "test_rule".to_string(),
        RulePattern::subject_predicate_object(None, PredicateID(0), None),
        PredicateID(1),
        Box::new(|c| c.first().copied().unwrap_or(0.0) * 0.9),
    );
    engine.register_rule(rule).unwrap();
    let mut schema = setup_schema_with_facts();
    let (derivations, stats) = engine.infer_with_stats(&mut schema).unwrap();
    assert!(!derivations.is_empty());
    assert_eq!(derivations[0].rule_id, 1);
    assert!(derivations[0].confidence_formula_result > 0.0);
    assert!(stats.iterations >= 1);
}

#[test]
fn test_inference_stats() {
    let mut engine = InferenceEngine::new().with_max_iterations(5);
    let rule = Rule::new(
        1,
        "rule1".to_string(),
        RulePattern::subject_predicate_object(None, PredicateID(0), None),
        PredicateID(1),
        Box::new(|c| c.first().copied().unwrap_or(0.0) * 0.5),
    );
    engine.register_rule(rule).unwrap();
    let mut schema = setup_schema_with_facts();
    let (derivations, stats) = engine.infer_with_stats(&mut schema).unwrap();
    assert!(stats.iterations >= 1);
    assert_eq!(stats.facts_derived, derivations.len());
    let _ = stats.duration_ms;
}

#[test]
fn test_inference_backward_compatible() {
    let mut engine = InferenceEngine::new();
    let rule = Rule::new(
        1,
        "rule1".to_string(),
        RulePattern::subject_predicate_object(None, PredicateID(0), None),
        PredicateID(1),
        Box::new(|c| c.first().copied().unwrap_or(0.0) * 0.9),
    );
    engine.register_rule(rule).unwrap();
    let mut schema = setup_schema_with_facts();
    let derived = engine.infer_forward_chaining(&mut schema).unwrap();
    assert!(!derived.is_empty());
    assert_eq!(derived[0].1, 1);
}
