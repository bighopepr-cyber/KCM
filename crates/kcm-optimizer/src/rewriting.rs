use crate::planner::{PlanNode, PlannerFilterPredicate};
use kcm_core::types::ColumnID;
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
            PlanNode::Filter { child, predicate } => match child.as_ref() {
                PlanNode::Join {
                    left,
                    right,
                    join_column,
                } => PlanNode::Join {
                    left: Box::new(self.apply(&PlanNode::Filter {
                        child: left.clone(),
                        predicate: predicate.clone(),
                    })),
                    right: right.clone(),
                    join_column: *join_column,
                },
                other => PlanNode::Filter {
                    child: Box::new(self.apply(other)),
                    predicate: predicate.clone(),
                },
            },
            other => other.clone(),
        }
    }
    fn name(&self) -> &str {
        "FilterPushdown"
    }
}

pub struct ColumnPruningOptimizer {
    required_columns: HashSet<ColumnID>,
}

impl ColumnPruningOptimizer {
    pub fn new(required_columns: Vec<ColumnID>) -> Self {
        ColumnPruningOptimizer {
            required_columns: required_columns.into_iter().collect(),
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
            PlanNode::Project { child, .. } => self.prune(child),
            PlanNode::Join {
                left,
                right,
                join_column,
            } => PlanNode::Join {
                left: Box::new(self.prune(left)),
                right: Box::new(self.prune(right)),
                join_column: *join_column,
            },
            PlanNode::Aggregate { child, group_by } => PlanNode::Aggregate {
                child: Box::new(self.prune(child)),
                group_by: *group_by,
            },
            PlanNode::Infer { child, rule_id } => PlanNode::Infer {
                child: Box::new(self.prune(child)),
                rule_id: *rule_id,
            },
        }
    }

    pub fn required_column_ids(&self) -> &HashSet<ColumnID> {
        &self.required_columns
    }
}

pub struct JoinOrderingOptimizer;

impl RuleOptimizer for JoinOrderingOptimizer {
    fn apply(&self, node: &PlanNode) -> PlanNode {
        match node {
            PlanNode::Join {
                left,
                right,
                join_column,
            } => {
                let (new_left, new_right) = Self::reorder(left, right);
                PlanNode::Join {
                    left: Box::new(new_left),
                    right: Box::new(new_right),
                    join_column: *join_column,
                }
            }
            other => other.clone(),
        }
    }
    fn name(&self) -> &str {
        "JoinOrdering"
    }
}

impl JoinOrderingOptimizer {
    pub fn estimate_join_cost(left_rows: usize, right_rows: usize) -> f64 {
        let smaller = left_rows.min(right_rows) as f64;
        let larger = left_rows.max(right_rows) as f64;
        smaller + larger * smaller.log2()
    }

    pub fn reorder(left: &PlanNode, right: &PlanNode) -> (PlanNode, PlanNode) {
        let left_cost =
            Self::estimate_join_cost(left.estimated_cost_rows(), right.estimated_cost_rows());
        let right_cost =
            Self::estimate_join_cost(right.estimated_cost_rows(), left.estimated_cost_rows());
        if left_cost <= right_cost {
            (left.clone(), right.clone())
        } else {
            (right.clone(), left.clone())
        }
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
                | (
                    PlannerFilterPredicate::EqualObject(_),
                    IndexType::BloomFilter
                )
                | (
                    PlannerFilterPredicate::EqualSubject(_),
                    IndexType::Composite
                )
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

impl PlanNode {
    pub fn estimated_cost_rows(&self) -> usize {
        match self {
            PlanNode::Scan { .. } => 1000,
            PlanNode::Filter { child, .. } => child.estimated_cost_rows() / 2,
            PlanNode::Join { left, right, .. } => {
                left.estimated_cost_rows().max(right.estimated_cost_rows())
            }
            PlanNode::Aggregate { child, .. } => child.estimated_cost_rows(),
            PlanNode::Infer { child, .. } => child.estimated_cost_rows(),
            PlanNode::Project { child, .. } => child.estimated_cost_rows(),
        }
    }
}

impl PartialEq for PlanNode {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                PlanNode::Scan {
                    confidence_filter: a,
                },
                PlanNode::Scan {
                    confidence_filter: b,
                },
            ) => a == b,
            (
                PlanNode::Filter {
                    child: a1,
                    predicate: a2,
                },
                PlanNode::Filter {
                    child: b1,
                    predicate: b2,
                },
            ) => a1 == b1 && a2 == b2,
            (
                PlanNode::Project {
                    child: a1,
                    columns: a2,
                },
                PlanNode::Project {
                    child: b1,
                    columns: b2,
                },
            ) => a1 == b1 && a2 == b2,
            (
                PlanNode::Join {
                    left: a1,
                    right: a2,
                    join_column: a3,
                },
                PlanNode::Join {
                    left: b1,
                    right: b2,
                    join_column: b3,
                },
            ) => a1 == b1 && a2 == b2 && a3 == b3,
            (
                PlanNode::Aggregate {
                    child: a1,
                    group_by: a2,
                },
                PlanNode::Aggregate {
                    child: b1,
                    group_by: b2,
                },
            ) => a1 == b1 && a2 == b2,
            (
                PlanNode::Infer {
                    child: a1,
                    rule_id: a2,
                },
                PlanNode::Infer {
                    child: b1,
                    rule_id: b2,
                },
            ) => a1 == b1 && a2 == b2,
            _ => std::mem::discriminant(self) == std::mem::discriminant(other),
        }
    }
}
