use crate::planner::{PlanNode, PlannerFilterPredicate};
use std::collections::HashSet;

pub trait RuleOptimizer {
    fn apply(&self, node: &PlanNode) -> PlanNode;
    fn name(&self) -> &str;
}

pub struct FilterPushdownOptimizer;

impl RuleOptimizer for FilterPushdownOptimizer {
    fn apply(&self, node: &PlanNode) -> PlanNode {
        match node {
            PlanNode::Project { child, columns } => PlanNode::Project {
                child: Box::new(self.apply(child)),
                columns: columns.clone(),
            },
            PlanNode::Join {
                left,
                right,
                join_column,
            } => PlanNode::Join {
                left: Box::new(self.apply(left)),
                right: Box::new(self.apply(right)),
                join_column: *join_column,
            },
            other => other.clone(),
        }
    }
    fn name(&self) -> &str {
        "FilterPushdown"
    }
}

#[allow(dead_code)]
pub struct ColumnPruningOptimizer {
    required_columns: HashSet<crate::planner::PlanNode>,
}

impl ColumnPruningOptimizer {
    pub fn new(_required_columns: Vec<String>) -> Self {
        ColumnPruningOptimizer {
            required_columns: HashSet::new(),
        }
    }

    pub fn prune(&self, node: &PlanNode) -> PlanNode {
        match node {
            PlanNode::Scan { confidence_filter } => PlanNode::Scan {
                confidence_filter: *confidence_filter,
            },
            PlanNode::Filter { child, predicate } => PlanNode::Filter {
                child: Box::new(self.prune(child)),
                predicate: predicate.clone(),
            },
            PlanNode::Project { child, columns } => PlanNode::Project {
                child: Box::new(self.prune(child)),
                columns: columns.clone(),
            },
            PlanNode::Join {
                left,
                right,
                join_column,
            } => PlanNode::Join {
                left: Box::new(self.prune(left)),
                right: Box::new(self.prune(right)),
                join_column: *join_column,
            },
            other => other.clone(),
        }
    }
}

pub struct ConstantFoldingOptimizer;

impl ConstantFoldingOptimizer {
    pub fn fold_predicate(pred: &PlannerFilterPredicate) -> Option<bool> {
        match pred {
            PlannerFilterPredicate::EqualSubject(_v) => None,
            PlannerFilterPredicate::EqualPredicate(_v) => None,
            PlannerFilterPredicate::EqualObject(_v) => None,
        }
    }
}

pub struct JoinOrderingOptimizer;

impl JoinOrderingOptimizer {
    pub fn estimate_join_cost(left_rows: usize, right_rows: usize) -> f64 {
        let smaller = left_rows.min(right_rows) as f64;
        let larger = left_rows.max(right_rows) as f64;
        smaller + larger * smaller.log2()
    }
}

pub struct IndexSelectionOptimizer;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IndexType {
    Bitmap,
    BloomFilter,
    Composite,
    ZoneMap,
}

impl IndexSelectionOptimizer {
    pub fn select_indices(
        predicates: &[PlannerFilterPredicate],
        available: &[IndexType],
    ) -> Vec<IndexType> {
        let mut selected = Vec::new();
        for pred in predicates {
            for index in available {
                if Self::can_use(pred, index) && !selected.contains(index) {
                    selected.push(index.clone());
                }
            }
        }
        selected
    }

    fn can_use(pred: &PlannerFilterPredicate, index: &IndexType) -> bool {
        matches!(
            (pred, index),
            (PlannerFilterPredicate::EqualPredicate(_), IndexType::Bitmap)
            | (PlannerFilterPredicate::EqualObject(_), IndexType::BloomFilter)
            | (PlannerFilterPredicate::EqualSubject(_), IndexType::Composite)
        )
    }
}

pub struct OptimizerPipeline {
    rules: Vec<Box<dyn RuleOptimizer>>,
}

impl OptimizerPipeline {
    pub fn new() -> Self {
        OptimizerPipeline {
            rules: vec![Box::new(FilterPushdownOptimizer)],
        }
    }

    pub fn with_rule(mut self, rule: Box<dyn RuleOptimizer>) -> Self {
        self.rules.push(rule);
        self
    }

    pub fn optimize(&self, plan: &PlanNode) -> PlanNode {
        let mut current = plan.clone();
        loop {
            let mut changed = false;
            for rule in &self.rules {
                let optimized = rule.apply(&current);
                if optimized != current {
                    changed = true;
                    current = optimized;
                }
            }
            if !changed {
                break;
            }
        }
        current
    }
}

impl Default for OptimizerPipeline {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for PlanNode {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}
