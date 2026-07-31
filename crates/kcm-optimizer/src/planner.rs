use crate::cost_model::{CostModel, OperatorCost};
use crate::statistics::Statistics;
use kcm_core::types::*;

#[derive(Debug, Clone)]
pub enum PlanNode {
    Scan {
        confidence_filter: Option<f64>,
    },
    Filter {
        child: Box<PlanNode>,
        predicate: PlannerFilterPredicate,
    },
    Join {
        left: Box<PlanNode>,
        right: Box<PlanNode>,
        join_column: ColumnID,
    },
    Aggregate {
        child: Box<PlanNode>,
        group_by: Option<ColumnID>,
    },
    Infer {
        child: Box<PlanNode>,
        rule_id: u32,
    },
    Project {
        child: Box<PlanNode>,
        columns: Vec<ColumnID>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum PlannerFilterPredicate {
    EqualSubject(u32),
    EqualPredicate(u8),
    EqualObject(u32),
}

pub struct QueryPlan {
    pub root: PlanNode,
    pub total_cost: OperatorCost,
}

impl QueryPlan {
    pub fn explain(&self) -> String {
        self.explain_node(&self.root, 0)
    }

    fn explain_node(&self, node: &PlanNode, indent: usize) -> String {
        let prefix = "  ".repeat(indent);
        match node {
            PlanNode::Scan { confidence_filter } => {
                format!(
                    "{}Scan (est. {} rows, conf={:?})",
                    prefix, self.total_cost.estimated_rows, confidence_filter
                )
            }
            PlanNode::Filter { child, predicate } => {
                let pred_str = match predicate {
                    PlannerFilterPredicate::EqualSubject(v) => format!("subject={}", v),
                    PlannerFilterPredicate::EqualPredicate(v) => format!("predicate={}", v),
                    PlannerFilterPredicate::EqualObject(v) => format!("object={}", v),
                };
                format!(
                    "{}Filter ({})\n{}",
                    prefix,
                    pred_str,
                    self.explain_node(child, indent + 1)
                )
            }
            PlanNode::Join {
                left,
                right,
                join_column,
            } => {
                format!(
                    "{}Join on {:?}\n{}\n{}",
                    prefix,
                    join_column,
                    self.explain_node(left, indent + 1),
                    self.explain_node(right, indent + 1)
                )
            }
            PlanNode::Aggregate { child, group_by } => {
                format!(
                    "{}Aggregate (group_by={:?})\n{}",
                    prefix,
                    group_by,
                    self.explain_node(child, indent + 1)
                )
            }
            PlanNode::Infer { child, rule_id } => {
                format!(
                    "{}Infer (rule={})\n{}",
                    prefix,
                    rule_id,
                    self.explain_node(child, indent + 1)
                )
            }
            PlanNode::Project { child, columns } => {
                format!(
                    "{}Project ({} cols)\n{}",
                    prefix,
                    columns.len(),
                    self.explain_node(child, indent + 1)
                )
            }
        }
    }
}

pub struct Planner {
    cost_model: CostModel,
    statistics: Option<Statistics>,
}

impl Planner {
    pub fn new(row_count: usize) -> Self {
        Planner {
            cost_model: CostModel::new(row_count),
            statistics: None,
        }
    }

    pub fn with_statistics(row_count: usize, statistics: Statistics) -> Self {
        Planner {
            cost_model: CostModel::new(row_count),
            statistics: Some(statistics),
        }
    }

    /// Estimate selectivity for a range predicate on a column.
    /// Uses real statistics when available, falls back to heuristic defaults.
    pub fn estimate_selectivity(&self, column: ColumnID, low: i64, high: i64) -> f64 {
        if let Some(ref stats) = self.statistics {
            stats.estimate_selectivity(column, low, high)
        } else {
            // Default heuristic: equality on a column with 1000 unique values
            0.001
        }
    }

    /// Estimate selectivity for an equality predicate.
    fn estimate_equality_selectivity(&self, column: ColumnID) -> f64 {
        if let Some(ref stats) = self.statistics {
            if let Some(col_stats) = stats.column_stats.get(&column) {
                if col_stats.row_count == 0 {
                    return 0.0;
                }
                let cardinality = col_stats.cardinality.max(1) as f64;
                return (1.0 / cardinality).clamp(0.0001, 1.0);
            }
        }
        0.01
    }

    /// Estimate join selectivity.
    fn estimate_join_selectivity(&self, left_cardinality: u64, right_cardinality: u64) -> f64 {
        if let Some(ref stats) = self.statistics {
            stats.estimate_join_selectivity(left_cardinality, right_cardinality)
        } else {
            0.1
        }
    }

    pub fn plan_simple_query(
        &self,
        subject_filter: Option<SubjectID>,
        predicate_filter: Option<PredicateID>,
        object_filter: Option<ObjectID>,
        confidence_filter: Option<f64>,
    ) -> QueryPlan {
        let mut node = PlanNode::Scan { confidence_filter };
        let mut cost = self.cost_model.estimate_scan(1.0);

        if let Some(subject) = subject_filter {
            let selectivity = self.estimate_equality_selectivity(ColumnID::Subject);
            cost = self
                .cost_model
                .estimate_filter(cost.estimated_rows, selectivity);
            node = PlanNode::Filter {
                child: Box::new(node),
                predicate: PlannerFilterPredicate::EqualSubject(subject.0),
            };
        }

        if let Some(pred) = predicate_filter {
            let selectivity = self.estimate_equality_selectivity(ColumnID::Predicate);
            cost = self
                .cost_model
                .estimate_filter(cost.estimated_rows, selectivity);
            node = PlanNode::Filter {
                child: Box::new(node),
                predicate: PlannerFilterPredicate::EqualPredicate(pred.0),
            };
        }

        if let Some(obj) = object_filter {
            let selectivity = self.estimate_equality_selectivity(ColumnID::Object);
            cost = self
                .cost_model
                .estimate_filter(cost.estimated_rows, selectivity);
            node = PlanNode::Filter {
                child: Box::new(node),
                predicate: PlannerFilterPredicate::EqualObject(obj.0),
            };
        }

        QueryPlan {
            root: node,
            total_cost: cost,
        }
    }

    pub fn plan_join(
        &self,
        left_filters: Vec<PlannerFilterPredicate>,
        right_filters: Vec<PlannerFilterPredicate>,
        join_column: ColumnID,
    ) -> QueryPlan {
        let mut left_node = PlanNode::Scan {
            confidence_filter: None,
        };
        let mut left_cost = self.cost_model.estimate_scan(1.0);

        for pred in left_filters {
            let selectivity = self.estimate_equality_selectivity(ColumnID::Subject);
            left_cost = self
                .cost_model
                .estimate_filter(left_cost.estimated_rows, selectivity);
            left_node = PlanNode::Filter {
                child: Box::new(left_node),
                predicate: pred,
            };
        }

        let mut right_node = PlanNode::Scan {
            confidence_filter: None,
        };
        let mut right_cost = self.cost_model.estimate_scan(1.0);

        for pred in right_filters {
            let selectivity = self.estimate_equality_selectivity(ColumnID::Object);
            right_cost = self
                .cost_model
                .estimate_filter(right_cost.estimated_rows, selectivity);
            right_node = PlanNode::Filter {
                child: Box::new(right_node),
                predicate: pred,
            };
        }

        let join_selectivity = self.estimate_join_selectivity(
            left_cost.estimated_rows as u64,
            right_cost.estimated_rows as u64,
        );
        let join_cost = self.cost_model.estimate_join(
            left_cost.estimated_rows,
            right_cost.estimated_rows,
            join_selectivity,
        );

        QueryPlan {
            root: PlanNode::Join {
                left: Box::new(left_node),
                right: Box::new(right_node),
                join_column,
            },
            total_cost: join_cost,
        }
    }
}
