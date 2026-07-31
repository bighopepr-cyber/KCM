use kcm_core::types::*;
use kcm_optimizer::adaptive::*;
use kcm_optimizer::cost_model::*;
use kcm_optimizer::planner::*;
use kcm_optimizer::rewriting::*;

#[test]
fn test_cost_model_scan() {
    let cm = CostModel::new(1_000_000);
    let cost = cm.estimate_scan(0.1);
    assert_eq!(cost.estimated_rows, 100_000);
    assert!(cost.cpu_cost > 0.0);
}

#[test]
fn test_cost_model_filter() {
    let cm = CostModel::new(1_000_000);
    let cost = cm.estimate_filter(100_000, 0.5);
    assert_eq!(cost.estimated_rows, 50_000);
}

#[test]
fn test_cost_model_join() {
    let cm = CostModel::new(1_000_000);
    let cost = cm.estimate_join(100_000, 50_000, 0.01);
    assert!(cost.estimated_rows > 0);
    assert!(cost.cpu_cost > 0.0);
}

#[test]
fn test_cost_model_aggregate() {
    let cm = CostModel::new(1_000_000);
    let cost = cm.estimate_aggregate(100_000, 100);
    assert_eq!(cost.estimated_rows, 100);
}

#[test]
fn test_planner_simple_query() {
    let planner = Planner::new(1_000_000);
    let plan = planner.plan_simple_query(Some(SubjectID(1)), Some(PredicateID(0)), None, Some(0.5));
    let explain = plan.explain();
    assert!(explain.contains("Filter"));
}

#[test]
fn test_planner_join() {
    let planner = Planner::new(1_000_000);
    let plan = planner.plan_join(
        vec![PlannerFilterPredicate::EqualSubject(1)],
        vec![PlannerFilterPredicate::EqualObject(2)],
        ColumnID::Object,
    );
    let explain = plan.explain();
    assert!(explain.contains("Join"));
}

#[test]
fn test_adaptive_executor() {
    let executor = AdaptiveExecutor::new();
    executor.record(42, 100, 120, 1.0, 1.2);
    executor.record(42, 100, 80, 1.0, 0.8);
    let factor = executor.cardinality_correction_factor(42);
    assert!(factor > 0.0);
    assert_eq!(executor.history_size(), 2);
}

#[test]
fn test_adaptive_executor_reoptimize() {
    let executor = AdaptiveExecutor::new().with_threshold(0.5);
    assert!(executor.should_reoptimize(0.6));
    assert!(!executor.should_reoptimize(0.4));
}

#[test]
fn test_adaptive_cost_error() {
    let executor = AdaptiveExecutor::new();
    executor.record(1, 100, 200, 1.0, 2.0);
    let error = executor.average_cost_error();
    assert!(error > 0.0);
}

#[test]
fn test_execution_stats_row_error() {
    let stats = ExecutionStats {
        actual_rows: 200,
        actual_time_ms: 10,
        estimated_rows: 100,
        estimated_time_ms: 5,
    };
    assert!((stats.row_error_ratio() - 1.0).abs() < 1e-10);
}

#[test]
fn test_execution_stats_zero_estimated() {
    let stats = ExecutionStats {
        actual_rows: 0,
        actual_time_ms: 0,
        estimated_rows: 0,
        estimated_time_ms: 0,
    };
    assert_eq!(stats.row_error_ratio(), 0.0);
}

#[test]
fn test_filter_pushdown() {
    let optimizer = FilterPushdownOptimizer;
    let plan = PlanNode::Scan {
        confidence_filter: None,
    };
    let optimized = optimizer.apply(&plan);
    assert!(matches!(optimized, PlanNode::Scan { .. }));
}

#[test]
fn test_index_selection() {
    use kcm_optimizer::rewriting::IndexType;
    let predicates = vec![
        PlannerFilterPredicate::EqualPredicate(0),
        PlannerFilterPredicate::EqualObject(1),
        PlannerFilterPredicate::EqualSubject(2),
    ];
    let available = vec![
        IndexType::Bitmap,
        IndexType::BloomFilter,
        IndexType::Composite,
    ];
    let selected = IndexSelectionOptimizer::select_indices(&predicates, &available);
    assert_eq!(selected.len(), 3);
}

#[test]
fn test_timer_guard() {
    let timer = TimerGuard::new("test");
    std::thread::sleep(std::time::Duration::from_millis(10));
    assert!(timer.elapsed_ms() >= 5);
    assert_eq!(timer.label(), "test");
}
