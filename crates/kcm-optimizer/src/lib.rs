pub mod adaptive;
pub mod cost_model;
pub mod planner;
pub mod rewriting;
pub mod statistics;

pub use cost_model::{CostModel, OperatorCost};
pub use planner::{PlanNode, Planner, PlannerAggregateFunc, PlannerFilterPredicate, QueryPlan};
pub use rewriting::{
    ColumnPruningOptimizer, FilterPushdownOptimizer, IndexSelectionOptimizer, IndexType,
    JoinOrderingOptimizer, OptimizerPipeline, RuleOptimizer,
};
pub use statistics::{ColumnStatistics, Statistics};

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;
    use kcm_core::types::*;

    #[test]
    fn test_plan_node_canonical() {
        let plan = PlanNode::Scan {
            context_filter: None,
            confidence_filter: None,
        };
        let explain = QueryPlan {
            root: plan,
            total_cost: OperatorCost {
                cpu_cost: 0.0,
                io_cost: 0.0,
                memory_cost: 0.0,
                estimated_rows: 1000,
            },
        };
        let output = explain.explain();
        assert!(output.contains("Scan"));
    }

    #[test]
    fn test_optimizer_pipeline_pushdown() {
        let pipeline = OptimizerPipeline::new();
        let plan = PlanNode::Filter {
            child: Box::new(PlanNode::Join {
                left: Box::new(PlanNode::Scan {
                    context_filter: None,
                    confidence_filter: None,
                }),
                right: Box::new(PlanNode::Scan {
                    context_filter: None,
                    confidence_filter: None,
                }),
                join_column: ColumnID::Object,
            }),
            predicate: PlannerFilterPredicate::EqualSubject(1),
        };

        let optimized = pipeline.optimize(&plan);
        match optimized {
            PlanNode::Join { left, .. } => {
                assert!(matches!(*left, PlanNode::Filter { .. }));
            }
            _ => panic!("Expected Join after filter pushdown"),
        }
    }

    #[test]
    fn test_optimizer_pipeline_reorder() {
        let pipeline =
            OptimizerPipeline::new().with_rule(Box::new(rewriting::JoinOrderingOptimizer));
        let plan = PlanNode::Join {
            left: Box::new(PlanNode::Scan {
                context_filter: None,
                confidence_filter: None,
            }),
            right: Box::new(PlanNode::Scan {
                context_filter: None,
                confidence_filter: None,
            }),
            join_column: ColumnID::Object,
        };

        let _optimized = pipeline.optimize(&plan);
    }

    #[test]
    fn test_planner_simple_query() {
        let planner = Planner::new(1000);
        let plan =
            planner.plan_simple_query(Some(SubjectID(1)), Some(PredicateID(5)), None, Some(0.8));
        let output = plan.explain();
        assert!(output.contains("Filter"));
        assert!(output.contains("Scan"));
    }

    #[test]
    fn test_planner_join() {
        let planner = Planner::new(1000);
        let plan = planner.plan_join(
            vec![PlannerFilterPredicate::EqualSubject(1)],
            vec![PlannerFilterPredicate::EqualObject(2)],
            ColumnID::Object,
        );
        let output = plan.explain();
        assert!(output.contains("Join"));
    }

    #[test]
    fn test_cost_model() {
        let model = CostModel::new(1000);
        let scan_cost = model.estimate_scan(1.0);
        assert_eq!(scan_cost.estimated_rows, 1000);

        let filter_cost = model.estimate_filter(1000, 0.1);
        assert_eq!(filter_cost.estimated_rows, 100);

        let join_cost = model.estimate_join(100, 200, 0.1);
        assert!(join_cost.estimated_rows > 0);
    }

    #[test]
    fn test_statistics_selectivity() {
        let stats = Statistics::new();
        let sel = stats.estimate_selectivity(ColumnID::Subject, 0, 100);
        assert!((0.0..=1.0).contains(&sel));
    }

    #[test]
    fn test_index_selection() {
        let predicates = vec![PlannerFilterPredicate::EqualPredicate(5)];
        let available = vec![IndexType::Bitmap, IndexType::BloomFilter];
        let selected = IndexSelectionOptimizer::select_indices(&predicates, &available);
        assert!(selected.contains(&IndexType::Bitmap));
    }
}
