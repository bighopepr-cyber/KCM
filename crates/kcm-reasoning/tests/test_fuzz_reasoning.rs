use kcm_core::types::*;
use kcm_reasoning::confidence::ConfidenceCalculator;
use kcm_reasoning::inference::InferenceEngine;
use kcm_reasoning::rule::{Rule, RulePattern};
use kcm_storage::column::Schema;

#[test]
fn test_confidence_multiply_bounds() {
    let pairs = vec![
        (0.0, 0.0),
        (0.0, 1.0),
        (1.0, 0.0),
        (1.0, 1.0),
        (0.5, 0.5),
        (0.001, 0.001),
        (0.999, 0.999),
    ];
    for (a, b) in pairs {
        let c1 = Confidence::new(a).unwrap();
        let c2 = Confidence::new(b).unwrap();
        let result = c1.multiply(c2);
        assert!(result.0 >= 0.0, "multiply({},{}) < 0", a, b);
        assert!(result.0 <= 1.0, "multiply({},{}) > 1", a, b);
        assert!(result.0.is_finite(), "multiply({},{}) not finite", a, b);
    }
}

#[test]
fn test_confidence_combine_or_bounds() {
    let pairs = vec![(0.0, 0.0), (0.0, 1.0), (1.0, 1.0), (0.5, 0.5)];
    for (a, b) in pairs {
        let c1 = Confidence::new(a).unwrap();
        let c2 = Confidence::new(b).unwrap();
        let result = c1.combine_or(c2);
        assert!(result.0 >= 0.0);
        assert!(result.0 <= 1.0);
        assert!(result.0.is_finite());
    }
}

#[test]
fn test_confidence_chained_operations() {
    let mut chain = Confidence::new(1.0).unwrap();
    for i in 1..=100 {
        let c = Confidence::new(0.99).unwrap();
        chain = chain.multiply(c);
        assert!(chain.0 >= 0.0, "Chain {} failed: {}", i, chain.0);
        assert!(chain.0 <= 1.0);
        assert!(chain.0.is_finite());
    }
}

#[test]
fn test_confidence_conjunction_commutative() {
    let c1 = Confidence::new(0.3).unwrap();
    let c2 = Confidence::new(0.7).unwrap();
    let r1 = c1.multiply(c2);
    let r2 = c2.multiply(c1);
    assert!((r1.0 - r2.0).abs() < 1e-10);
}

#[test]
fn test_confidence_calculator_chain() {
    let values = vec![1.0; 100];
    let result = ConfidenceCalculator::chain(&values);
    assert!((result - 1.0).abs() < 1e-10);
}

#[test]
fn test_confidence_calculator_weighted() {
    let values = vec![1.0; 100];
    let weights = vec![1.0; 100];
    let result = ConfidenceCalculator::weighted(&values, &weights);
    assert!((result - 1.0).abs() < 1e-10);
}

#[test]
fn test_inference_large_scale() {
    let mut engine = InferenceEngine::new().with_max_iterations(3);
    for i in 0..50 {
        let rule = Rule::new(
            i,
            format!("rule_{}", i),
            RulePattern::subject_predicate_object(None, PredicateID((i % 10) as u8), None),
            PredicateID(((i + 10) % 20) as u8),
            Box::new(|c| c.first().copied().unwrap_or(0.0) * 0.9),
        );
        engine.register_rule(rule).unwrap();
    }

    let mut schema = Schema::new(100_000).unwrap();
    for i in 0..1000u32 {
        schema
            .append_fact(
                &Fact::new(
                    SubjectID(i),
                    PredicateID((i % 10) as u8),
                    ObjectID(i + 100),
                    0.8,
                )
                .unwrap(),
            )
            .unwrap();
    }

    let (derivations, stats) = engine.infer_with_stats(&mut schema).unwrap();
    assert!(stats.iterations >= 1);
    assert!(stats.facts_derived > 0);
    assert_eq!(stats.facts_derived, derivations.len());
}

#[test]
fn test_inference_deterministic() {
    let mut engine1 = InferenceEngine::new().with_max_iterations(3);
    let mut engine2 = InferenceEngine::new().with_max_iterations(3);

    let rule1 = Rule::new(
        1,
        "det_rule".to_string(),
        RulePattern::subject_predicate_object(None, PredicateID(0), None),
        PredicateID(1),
        Box::new(|c| c.first().copied().unwrap_or(0.0) * 0.5),
    );
    let rule2 = Rule::new(
        1,
        "det_rule".to_string(),
        RulePattern::subject_predicate_object(None, PredicateID(0), None),
        PredicateID(1),
        Box::new(|c| c.first().copied().unwrap_or(0.0) * 0.5),
    );
    engine1.register_rule(rule1).unwrap();
    engine2.register_rule(rule2).unwrap();

    let mut schema1 = Schema::new(1000).unwrap();
    let mut schema2 = Schema::new(1000).unwrap();
    for i in 0..100u32 {
        let f = Fact::new(SubjectID(i), PredicateID(0), ObjectID(i), 0.8).unwrap();
        schema1.append_fact(&f).unwrap();
        schema2.append_fact(&f).unwrap();
    }

    let (d1, _s1) = engine1.infer_with_stats(&mut schema1).unwrap();
    let (d2, _s2) = engine2.infer_with_stats(&mut schema2).unwrap();

    assert_eq!(d1.len(), d2.len());
    for (a, b) in d1.iter().zip(d2.iter()) {
        assert_eq!(a.rule_id, b.rule_id);
        assert!((a.confidence_formula_result - b.confidence_formula_result).abs() < 1e-10);
    }
}
