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
    assert!(executor.cardinality_correction_factor(42) > 0.0);
    assert_eq!(executor.history_size(), 2);
}

#[test]
fn test_adaptive_reoptimize() {
    let executor = AdaptiveExecutor::new().with_threshold(0.5);
    assert!(executor.should_reoptimize(0.6));
    assert!(!executor.should_reoptimize(0.4));
}

#[test]
fn test_adaptive_cost_error() {
    let executor = AdaptiveExecutor::new();
    executor.record(1, 100, 200, 1.0, 2.0);
    assert!(executor.average_cost_error() > 0.0);
}

#[test]
fn test_execution_stats() {
    let stats = ExecutionStats {
        actual_rows: 200,
        actual_time_ms: 10,
        estimated_rows: 100,
        estimated_time_ms: 5,
    };
    assert!((stats.row_error_ratio() - 1.0).abs() < 1e-10);
}

#[test]
fn test_execution_stats_zero() {
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
fn test_column_pruning() {
    let pruning = ColumnPruningOptimizer::new(vec![ColumnID::Subject, ColumnID::Object]);
    assert_eq!(pruning.required_column_ids().len(), 2);
    let plan = PlanNode::Project {
        child: Box::new(PlanNode::Scan {
            confidence_filter: None,
        }),
        columns: vec![ColumnID::Subject],
    };
    let pruned = pruning.prune(&plan);
    assert!(!matches!(pruned, PlanNode::Project { .. }));
}

#[test]
fn test_join_ordering() {
    let cost = JoinOrderingOptimizer::estimate_join_cost(100, 1000);
    assert!(cost > 0.0);
    let plan1 = PlanNode::Scan {
        confidence_filter: None,
    };
    let plan2 = PlanNode::Scan {
        confidence_filter: None,
    };
    let (a, b) = JoinOrderingOptimizer::reorder(&plan1, &plan2);
    assert!(matches!(a, PlanNode::Scan { .. }));
    assert!(matches!(b, PlanNode::Scan { .. }));
}

#[test]
fn test_index_selection() {
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
fn test_optimizer_pipeline() {
    let pipeline = OptimizerPipeline::new();
    let plan = PlanNode::Scan {
        confidence_filter: None,
    };
    let optimized = pipeline.optimize(&plan);
    assert!(matches!(optimized, PlanNode::Scan { .. }));
}
