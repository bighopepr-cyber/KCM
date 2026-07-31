use kcm_core::types::*;
use kcm_reasoning::confidence::ConfidenceCalculator;
use kcm_reasoning::inference::InferenceEngine;
use kcm_reasoning::rule::{Rule, RulePattern, RuleRegistry};
use kcm_storage::column::Schema;

#[test]
fn test_rule_registry_register() {
    let mut registry = RuleRegistry::new();

    let rule = Rule::new(
        1,
        "rule1".to_string(),
        RulePattern::subject_predicate_object(None, PredicateID(0), None),
        PredicateID(1),
        Box::new(|confs| confs[0] * 0.9),
    );

    registry.register(rule).unwrap();
    assert!(registry.get(1).is_some());
    assert_eq!(registry.get(1).unwrap().name, "rule1");
}

#[test]
fn test_rule_registry_duplicate() {
    let mut registry = RuleRegistry::new();

    let rule1 = Rule::new(
        1,
        "rule1".to_string(),
        RulePattern::subject_predicate_object(None, PredicateID(0), None),
        PredicateID(1),
        Box::new(|confs| confs[0]),
    );

    let rule2 = Rule::new(
        1,
        "rule2".to_string(),
        RulePattern::subject_predicate_object(None, PredicateID(0), None),
        PredicateID(2),
        Box::new(|confs| confs[0]),
    );

    registry.register(rule1).unwrap();
    assert!(registry.register(rule2).is_err());
}

#[test]
fn test_rule_registry_all_enabled() {
    let mut registry = RuleRegistry::new();

    for i in 0..5 {
        let rule = Rule::new(
            i,
            format!("rule_{}", i),
            RulePattern::subject_predicate_object(None, PredicateID(0), None),
            PredicateID(1),
            Box::new(|confs| confs[0]),
        );
        registry.register(rule).unwrap();
    }

    assert_eq!(registry.all_enabled().len(), 5);
}

#[test]
fn test_rule_with_description() {
    let rule = Rule::new(
        1,
        "test_rule".to_string(),
        RulePattern::subject_predicate_object(None, PredicateID(0), None),
        PredicateID(1),
        Box::new(|confs| confs[0]),
    )
    .with_description("A test rule".to_string());

    assert_eq!(rule.description, "A test rule");
}

#[test]
fn test_rule_pattern_and() {
    let left = RulePattern::subject_predicate_object(None, PredicateID(0), None);
    let right = RulePattern::subject_predicate_object(None, PredicateID(1), None);
    let combined = RulePattern::and(left, right);

    match combined {
        RulePattern::And(l, r) => {
            match *l {
                RulePattern::Triple(_, p, _) => assert_eq!(p, PredicateID(0)),
                _ => panic!("Expected Triple"),
            }
            match *r {
                RulePattern::Triple(_, p, _) => assert_eq!(p, PredicateID(1)),
                _ => panic!("Expected Triple"),
            }
        }
        _ => panic!("Expected And"),
    }
}

#[test]
fn test_rule_pattern_or() {
    let left = RulePattern::subject_predicate_object(None, PredicateID(0), None);
    let right = RulePattern::subject_predicate_object(None, PredicateID(1), None);
    let combined = RulePattern::or(left, right);

    match combined {
        RulePattern::Or(l, r) => {
            match *l {
                RulePattern::Triple(_, p, _) => assert_eq!(p, PredicateID(0)),
                _ => panic!("Expected Triple"),
            }
            match *r {
                RulePattern::Triple(_, p, _) => assert_eq!(p, PredicateID(1)),
                _ => panic!("Expected Triple"),
            }
        }
        _ => panic!("Expected Or"),
    }
}

#[test]
fn test_rule_pattern_not() {
    let inner = RulePattern::subject_predicate_object(None, PredicateID(0), None);
    let negated = RulePattern::not(inner);

    match negated {
        RulePattern::Not(inner) => match *inner {
            RulePattern::Triple(_, p, _) => assert_eq!(p, PredicateID(0)),
            _ => panic!("Expected Triple"),
        },
        _ => panic!("Expected Not"),
    }
}

#[test]
fn test_inference_engine_simple() {
    let mut engine = InferenceEngine::new();

    let rule = Rule::new(
        1,
        "transitive_rule".to_string(),
        RulePattern::subject_predicate_object(None, PredicateID(0), None),
        PredicateID(1),
        Box::new(|confs| confs[0] * 0.9),
    );

    engine.register_rule(rule).unwrap();

    let mut schema = Schema::new(10_000).unwrap();

    for i in 0..5u32 {
        let fact = Fact::new(SubjectID(i), PredicateID(0), ObjectID(i + 10), 0.8).unwrap();
        schema.append_fact(&fact).unwrap();
    }

    let derived = engine.infer_forward_chaining(&mut schema).unwrap();
    assert!(!derived.is_empty());

    for (fact, rule_id) in &derived {
        assert_eq!(*rule_id, 1);
        assert_eq!(fact.predicate, PredicateID(1));
        assert!(fact.confidence >= 0.3);
        assert!(fact.confidence <= 1.0);
    }
}

#[test]
fn test_inference_engine_no_rules() {
    let engine = InferenceEngine::new();
    let mut schema = Schema::new(10_000).unwrap();

    let derived = engine.infer_forward_chaining(&mut schema).unwrap();
    assert!(derived.is_empty());
}

#[test]
fn test_inference_engine_disabled_rule() {
    let mut engine = InferenceEngine::new();

    let mut rule = Rule::new(
        1,
        "disabled_rule".to_string(),
        RulePattern::subject_predicate_object(None, PredicateID(0), None),
        PredicateID(1),
        Box::new(|confs| confs[0]),
    );
    rule.enabled = false;

    engine.register_rule(rule).unwrap();

    let mut schema = Schema::new(10_000).unwrap();
    let fact = Fact::new(SubjectID(1), PredicateID(0), ObjectID(2), 0.9).unwrap();
    schema.append_fact(&fact).unwrap();

    let derived = engine.infer_forward_chaining(&mut schema).unwrap();
    assert!(derived.is_empty());
}

#[test]
fn test_inference_engine_and_pattern() {
    let mut engine = InferenceEngine::new();

    let pattern = RulePattern::and(
        RulePattern::subject_predicate_object(None, PredicateID(0), None),
        RulePattern::subject_predicate_object(None, PredicateID(1), None),
    );

    let rule = Rule::new(
        1,
        "and_rule".to_string(),
        pattern,
        PredicateID(2),
        Box::new(|confs| {
            if confs.len() >= 2 {
                confs[0] * confs[1]
            } else {
                0.0
            }
        }),
    );

    engine.register_rule(rule).unwrap();

    let mut schema = Schema::new(10_000).unwrap();

    let fact1 = Fact::new(SubjectID(1), PredicateID(0), ObjectID(2), 0.9).unwrap();
    let fact2 = Fact::new(SubjectID(2), PredicateID(1), ObjectID(3), 0.8).unwrap();

    schema.append_fact(&fact1).unwrap();
    schema.append_fact(&fact2).unwrap();

    let derived = engine.infer_forward_chaining(&mut schema).unwrap();
    assert!(!derived.is_empty());
}

#[test]
fn test_confidence_conjunction() {
    assert!((ConfidenceCalculator::conjunction(0.5, 0.6) - 0.3).abs() < 0.0001);
    assert!((ConfidenceCalculator::conjunction(1.0, 1.0) - 1.0).abs() < 0.0001);
    assert!((ConfidenceCalculator::conjunction(0.0, 1.0) - 0.0).abs() < 0.0001);
}

#[test]
fn test_confidence_disjunction() {
    assert!((ConfidenceCalculator::disjunction(0.5, 0.6) - 0.8).abs() < 0.0001);
    assert!((ConfidenceCalculator::disjunction(0.0, 0.0) - 0.0).abs() < 0.0001);
    assert!((ConfidenceCalculator::disjunction(1.0, 0.5) - 1.0).abs() < 0.0001);
}

#[test]
fn test_confidence_negation() {
    assert!((ConfidenceCalculator::negation(0.7) - 0.3).abs() < 0.0001);
    assert!((ConfidenceCalculator::negation(0.0) - 1.0).abs() < 0.0001);
    assert!((ConfidenceCalculator::negation(1.0) - 0.0).abs() < 0.0001);
}

#[test]
fn test_confidence_chain() {
    let values = vec![0.9, 0.8, 0.7];
    let result = ConfidenceCalculator::chain(&values);
    let expected = 0.9 * 0.8 * 0.7;
    assert!((result - expected).abs() < 0.0001);
}

#[test]
fn test_confidence_weighted() {
    let values = vec![0.9, 0.5];
    let weights = vec![2.0, 1.0];
    let result = ConfidenceCalculator::weighted(&values, &weights);
    let expected = (0.9 * 2.0 + 0.5 * 1.0) / 3.0;
    assert!((result - expected).abs() < 0.0001);
}

#[test]
fn test_inference_engine_multiple_iterations() {
    let mut engine = InferenceEngine::new().with_max_iterations(2);

    let rule1 = Rule::new(
        1,
        "rule_a".to_string(),
        RulePattern::subject_predicate_object(None, PredicateID(0), None),
        PredicateID(1),
        Box::new(|confs| confs[0] * 0.9),
    );

    let rule2 = Rule::new(
        2,
        "rule_b".to_string(),
        RulePattern::subject_predicate_object(None, PredicateID(1), None),
        PredicateID(2),
        Box::new(|confs| confs[0] * 0.8),
    );

    engine.register_rule(rule1).unwrap();
    engine.register_rule(rule2).unwrap();

    let mut schema = Schema::new(10_000).unwrap();
    let fact = Fact::new(SubjectID(1), PredicateID(0), ObjectID(2), 0.9).unwrap();
    schema.append_fact(&fact).unwrap();

    let derived = engine.infer_forward_chaining(&mut schema).unwrap();
    assert!(!derived.is_empty());

    let has_pred1 = derived.iter().any(|(f, _)| f.predicate == PredicateID(1));
    assert!(has_pred1);
}
