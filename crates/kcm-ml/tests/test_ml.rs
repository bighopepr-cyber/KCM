#![allow(clippy::unwrap_used, clippy::panic)]

use kcm_core::types::*;
use kcm_ml::confidence_learner::*;
use kcm_ml::learned_index::*;
use kcm_ml::rule_discovery::*;

#[test]
fn test_regression_model_train_predict() {
    let mut model = RegressionModel::new();
    let x: Vec<u32> = (0..100).collect();
    let y: Vec<usize> = (0..100).collect();
    model.train(&x, &y);
    let predicted = model.predict(50);
    assert!((predicted as i64 - 50).abs() < 5);
}

#[test]
fn test_regression_model_linear() {
    let mut model = RegressionModel::new();
    let x = vec![1, 2, 3, 4, 5];
    let y = vec![2, 4, 6, 8, 10];
    model.train(&x, &y);
    assert_eq!(model.predict(6), 12);
    assert_eq!(model.predict(10), 20);
}

#[test]
fn test_regression_model_empty() {
    let mut model = RegressionModel::new();
    model.train(&[], &[]);
    assert_eq!(model.predict(42), 0);
}

#[test]
fn test_regression_model_negative_slope() {
    let mut model = RegressionModel::new();
    let x = vec![1, 2, 3, 4, 5];
    let y = vec![50, 40, 30, 20, 10];
    model.train(&x, &y);
    assert_eq!(model.predict(6), 0);
}

#[test]
fn test_learned_index_search() {
    let mut index = LearnedIndex::new(4);
    let values: Vec<u32> = (0..1000).collect();
    let positions: Vec<usize> = (0..1000).collect();
    index.train(&values, &positions);
    let (lower, upper) = index.search(500);
    assert!(lower <= 500);
    assert!(upper >= 500);
}

#[test]
fn test_learned_index_search_bounds() {
    let mut index = LearnedIndex::new(2);
    let values = vec![0u32, 100, 200, 300, 400, 500];
    let positions = vec![0usize, 1, 2, 3, 4, 5];
    index.train(&values, &positions);
    let (lower, upper) = index.search(250);
    assert!(
        lower <= upper,
        "lower {} should be <= upper {}",
        lower,
        upper
    );
    assert!(upper <= positions.len() + 100, "upper {} too large", upper);
}

#[test]
fn test_confidence_learner() {
    let mut learner = ConfidenceLearner::new();
    learner.observe_fact("f1".to_string(), 0.9, true);
    learner.observe_fact("f1".to_string(), 0.8, true);
    learner.observe_fact("f1".to_string(), 0.2, false);
    let predicted = learner.predict_confidence("f1").unwrap();
    assert!(predicted > 0.0 && predicted <= 1.0);
    assert!(learner.predict_confidence("nonexistent").is_none());
}

#[test]
fn test_confidence_learner_rule_accuracy() {
    let mut learner = ConfidenceLearner::new();
    learner.observe_rule_inference(1, 0.8, 0.9);
    learner.observe_rule_inference(1, 0.7, 0.8);
    let accuracy = learner.get_rule_accuracy(1);
    assert!(accuracy > 0.0 && accuracy <= 1.0);
    assert_eq!(learner.get_rule_accuracy(999), 0.5);
}

#[test]
fn test_confidence_learner_adjust() {
    let mut learner = ConfidenceLearner::new();
    learner.observe_rule_inference(1, 0.9, 0.9);
    let adjusted = learner.adjust_confidence(1, 0.8);
    assert!(adjusted > 0.0 && adjusted <= 1.0);
}

#[test]
fn test_confidence_learner_tracked_rules() {
    let mut learner = ConfidenceLearner::new();
    learner.observe_rule_inference(1, 0.8, 0.9);
    learner.observe_rule_inference(2, 0.7, 0.8);
    assert_eq!(learner.rules_tracked(), 2);
}

#[test]
fn test_confidence_learner_ema_convergence() {
    let mut learner = ConfidenceLearner::new();
    for _ in 0..100 {
        learner.observe_rule_inference(1, 0.8, 0.8);
    }
    let accuracy = learner.get_rule_accuracy(1);
    assert!(accuracy > 0.5, "Expected accuracy > 0.5, got {}", accuracy);
}

#[test]
fn test_rule_discovery() {
    let engine = RuleDiscoveryEngine::new(0.01, 0.1);
    let facts: Vec<Fact> = (0..50u32)
        .map(|i| Fact::new(SubjectID(i), PredicateID(0), ObjectID(i + 100), 0.9).unwrap())
        .collect();
    let patterns = engine.discover_patterns(&facts);
    let rules = engine.patterns_to_rules(&patterns);
    assert!(rules.len() <= patterns.len());
}

#[test]
fn test_rule_discovery_empty() {
    let engine = RuleDiscoveryEngine::new(0.5, 0.5);
    let patterns = engine.discover_patterns(&[]);
    assert!(patterns.is_empty());
}

#[test]
fn test_regression_model_constant() {
    let mut model = RegressionModel::new();
    let x = vec![1, 2, 3, 4, 5];
    let y = vec![10, 10, 10, 10, 10];
    model.train(&x, &y);
    assert_eq!(model.predict(100), 10);
}
