#![allow(clippy::unwrap_used, clippy::panic)]

use kcm_core::types::*;
use kcm_ml::confidence_learner::*;
use kcm_ml::learned_index::*;
use kcm_ml::rule_discovery::*;

// ============================================================
// GROUND TRUTH VALIDATION
// ============================================================

#[test]
fn test_ground_truth_learned_index_monotonic_data() {
    let mut index = LearnedIndex::new(4);
    let values: Vec<u32> = (0..1000).collect();
    let positions: Vec<usize> = (0..1000).collect();
    index.train(&values, &positions);

    for target in (0..1000u32).step_by(10) {
        let (lower, upper) = index.search(target);
        assert!(
            lower <= upper,
            "lower {} > upper {} for target {}",
            lower,
            upper,
            target
        );
        let true_pos = target as usize;
        assert!(
            lower <= true_pos + 100,
            "lower {} too far from true position {} for target {}",
            lower,
            true_pos,
            target
        );
        assert!(
            upper + 100 >= true_pos,
            "upper {} too far from true position {} for target {}",
            upper,
            true_pos,
            target
        );
    }
}

#[test]
fn test_ground_truth_regression_exact_fit() {
    let mut model = RegressionModel::new();
    let x: Vec<u32> = (0..100).collect();
    let y: Vec<usize> = (0..100).collect();
    model.train(&x, &y);

    for i in 0..100u32 {
        let predicted = model.predict(i);
        assert!(
            (predicted as i64 - i as i64).abs() <= 1,
            "Prediction {} differs from expected {} by more than 1",
            predicted,
            i
        );
    }
}

#[test]
fn test_ground_truth_regression_scaled() {
    let mut model = RegressionModel::new();
    let x: Vec<u32> = (0..100).collect();
    let y: Vec<usize> = (0..100).map(|i| i * 3).collect();
    model.train(&x, &y);

    let predicted = model.predict(50);
    assert!(
        (predicted as i64 - 150).abs() <= 2,
        "Expected ~150, got {}",
        predicted
    );
}

// ============================================================
// CONFIDENCE CALIBRATION
// ============================================================

#[test]
fn test_confidence_calibration_always_correct() {
    let mut learner = ConfidenceLearner::new();
    for _ in 0..100 {
        learner.observe_fact("reliable".to_string(), 0.9, true);
    }
    let predicted = learner.predict_confidence("reliable").unwrap();
    assert!(
        predicted > 0.8,
        "After 100 correct observations, confidence {} should be > 0.8",
        predicted
    );
}

#[test]
fn test_confidence_calibration_always_wrong() {
    let mut learner = ConfidenceLearner::new();
    for _ in 0..100 {
        learner.observe_fact("unreliable".to_string(), 0.9, false);
    }
    let predicted = learner.predict_confidence("unreliable").unwrap();
    assert!(
        predicted < 0.5,
        "After 100 incorrect observations, confidence {} should be < 0.5",
        predicted
    );
}

#[test]
fn test_confidence_calibration_mixed_50_50() {
    let mut learner = ConfidenceLearner::new();
    for i in 0..100 {
        let correct = i % 2 == 0;
        learner.observe_fact("mixed".to_string(), 0.9, correct);
    }
    let predicted = learner.predict_confidence("mixed").unwrap();
    assert!(
        (0.0..=1.0).contains(&predicted),
        "After 50/50 observations, confidence {} should be in [0, 1]",
        predicted
    );
    assert!(
        predicted <= 0.5,
        "After 50/50 observations, confidence {} should not exceed 0.5",
        predicted
    );
}

#[test]
fn test_confidence_ema_convergence_rate() {
    let mut learner = ConfidenceLearner::new();
    for _ in 0..10 {
        learner.observe_rule_inference(1, 0.9, 0.9);
    }
    let accuracy_early = learner.get_rule_accuracy(1);
    for _ in 0..90 {
        learner.observe_rule_inference(1, 0.9, 0.9);
    }
    let accuracy_late = learner.get_rule_accuracy(1);
    assert!(
        accuracy_late >= accuracy_early,
        "EMA should not decrease with more perfect observations: {} < {}",
        accuracy_late,
        accuracy_early
    );
    assert!(
        accuracy_late > 0.9,
        "EMA should converge above 0.9 after 100 perfect observations, got {}",
        accuracy_late
    );
}

// ============================================================
// RULE DISCOVERY VALIDATION
// ============================================================

#[test]
fn test_rule_discovery_support_threshold_enforced() {
    let engine_high_support = RuleDiscoveryEngine::new(0.5, 0.1);
    let facts: Vec<Fact> = (0..20u32)
        .map(|i| Fact::new(SubjectID(i), PredicateID(0), ObjectID(100), 0.9).unwrap())
        .collect();
    let patterns_high = engine_high_support.discover_patterns(&facts);

    let engine_low_support = RuleDiscoveryEngine::new(0.01, 0.1);
    let patterns_low = engine_low_support.discover_patterns(&facts);
    assert!(
        patterns_low.len() >= patterns_high.len(),
        "Lower support threshold should find at least as many patterns"
    );
}

#[test]
fn test_rule_discovery_confidence_threshold_enforced() {
    let engine_strict = RuleDiscoveryEngine::new(0.01, 0.9);
    let facts: Vec<Fact> = (0..50u32)
        .map(|i| Fact::new(SubjectID(i), PredicateID(0), ObjectID(i + 100), 0.9).unwrap())
        .collect();
    let patterns_strict = engine_strict.discover_patterns(&facts);
    let rules_strict = engine_strict.patterns_to_rules(&patterns_strict);

    let engine_loose = RuleDiscoveryEngine::new(0.01, 0.1);
    let patterns_loose = engine_loose.discover_patterns(&facts);
    let rules_loose = engine_loose.patterns_to_rules(&patterns_loose);

    assert!(
        rules_strict.len() <= rules_loose.len(),
        "Strict confidence should produce fewer or equal rules"
    );
}

#[test]
fn test_rule_discovery_empty_facts_empty_rules() {
    let engine = RuleDiscoveryEngine::new(0.01, 0.1);
    let patterns = engine.discover_patterns(&[]);
    assert!(patterns.is_empty());
    let rules = engine.patterns_to_rules(&patterns);
    assert!(rules.is_empty());
}

// ============================================================
// REPEATABILITY TESTS
// ============================================================

#[test]
fn test_regression_deterministic_training() {
    let x: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let y: Vec<usize> = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];

    let mut model1 = RegressionModel::new();
    model1.train(&x, &y);
    let pred1_a = model1.predict(5);
    let pred1_b = model1.predict(5);

    let mut model2 = RegressionModel::new();
    model2.train(&x, &y);
    let pred2 = model2.predict(5);

    assert_eq!(
        pred1_a, pred1_b,
        "Same model should produce same prediction"
    );
    assert_eq!(
        pred1_a, pred2,
        "Identical training should produce identical predictions"
    );
}

#[test]
fn test_learned_index_search_repeatability() {
    let mut index = LearnedIndex::new(4);
    let values: Vec<u32> = (0..1000).collect();
    let positions: Vec<usize> = (0..1000).collect();
    index.train(&values, &positions);

    let (l1, u1) = index.search(500);
    let (l2, u2) = index.search(500);
    let (l3, u3) = index.search(500);

    assert_eq!(
        (l1, u1),
        (l2, u2),
        "Search should be deterministic across calls"
    );
    assert_eq!(
        (l2, u2),
        (l3, u3),
        "Search should be deterministic across calls"
    );
}

#[test]
fn test_confidence_learner_repeatability() {
    let mut learner1 = ConfidenceLearner::new();
    let mut learner2 = ConfidenceLearner::new();

    let observations = vec![
        ("f1".to_string(), 0.9, true),
        ("f1".to_string(), 0.8, false),
        ("f1".to_string(), 0.7, true),
        ("f2".to_string(), 0.6, true),
        ("f2".to_string(), 0.5, false),
    ];

    for (key, conf, correct) in &observations {
        learner1.observe_fact(key.clone(), *conf, *correct);
        learner2.observe_fact(key.clone(), *conf, *correct);
    }

    let pred1 = learner1.predict_confidence("f1").unwrap();
    let pred2 = learner2.predict_confidence("f1").unwrap();
    assert_eq!(
        pred1, pred2,
        "Identical observations should produce identical predictions"
    );
}

#[test]
fn test_rule_discovery_repeatability() {
    let facts: Vec<Fact> = (0..50u32)
        .map(|i| Fact::new(SubjectID(i), PredicateID(0), ObjectID(i + 100), 0.9).unwrap())
        .collect();

    let engine1 = RuleDiscoveryEngine::new(0.01, 0.1);
    let patterns1 = engine1.discover_patterns(&facts);
    let rules1 = engine1.patterns_to_rules(&patterns1);

    let engine2 = RuleDiscoveryEngine::new(0.01, 0.1);
    let patterns2 = engine2.discover_patterns(&facts);
    let rules2 = engine2.patterns_to_rules(&patterns2);

    assert_eq!(
        patterns1.len(),
        patterns2.len(),
        "Pattern discovery should be deterministic"
    );
    assert_eq!(
        rules1.len(),
        rules2.len(),
        "Rule generation should be deterministic"
    );
}

// ============================================================
// BIAS DETECTION
// ============================================================

#[test]
fn test_regression_no_systematic_bias() {
    let mut model = RegressionModel::new();
    let x: Vec<u32> = (0..200).collect();
    let y: Vec<usize> = (0..200).collect();
    model.train(&x, &y);

    let mut over_count = 0i64;
    let mut under_count = 0i64;

    for i in 0..200u32 {
        let predicted = model.predict(i);
        if predicted > i as usize {
            over_count += 1;
        } else if predicted < i as usize {
            under_count += 1;
        }
    }
    let diff = (over_count - under_count).abs();
    assert!(
        diff < 50,
        "Systematic bias detected: over={}, under={}, diff={}",
        over_count,
        under_count,
        diff
    );
}

#[test]
fn test_confidence_no_systematic_bias() {
    let mut learner = ConfidenceLearner::new();
    for i in 0..200u32 {
        let correct = i % 2 == 0;
        learner.observe_fact("test".to_string(), 0.8, correct);
    }
    let predicted = learner.predict_confidence("test").unwrap();
    assert!(
        (0.0..=0.5).contains(&predicted),
        "Confidence for 50/50 data should be <= 0.5, got {}",
        predicted
    );
}

// ============================================================
// EDGE CASES
// ============================================================

#[test]
fn test_learned_index_single_element() {
    let mut index = LearnedIndex::new(1);
    index.train(&[42], &[0]);
    let (lower, upper) = index.search(42);
    assert!(lower <= upper);
}

#[test]
fn test_learned_index_all_identical_values() {
    let mut index = LearnedIndex::new(2);
    let values = vec![5u32; 100];
    let positions: Vec<usize> = (0..100).collect();
    index.train(&values, &positions);
    let (lower, upper) = index.search(5);
    assert!(lower <= upper);
}

#[test]
fn test_confidence_empty_observations() {
    let learner = ConfidenceLearner::new();
    assert!(learner.predict_confidence("anything").is_none());
    assert_eq!(learner.get_rule_accuracy(999), 0.5);
}

#[test]
fn test_rule_discovery_high_support_no_patterns() {
    let engine = RuleDiscoveryEngine::new(0.99, 0.1);
    let facts: Vec<Fact> = (0..10u32)
        .map(|i| Fact::new(SubjectID(i), PredicateID(0), ObjectID(100), 0.9).unwrap())
        .collect();
    let patterns = engine.discover_patterns(&facts);
    assert!(patterns.is_empty());
}

#[test]
fn test_regression_boundary_values() {
    let mut model = RegressionModel::new();
    let x = vec![0u32, u32::MAX / 2];
    let y = vec![0usize, usize::MAX / 2];
    model.train(&x, &y);
    let pred = model.predict(u32::MAX / 4);
    assert!(pred <= usize::MAX / 2 + 1);
}
