use kcm_core::types::*;
use kcm_storage::column::Schema;

pub mod adaptive;
pub mod cost_model;
pub mod planner;
pub mod rewriting;
pub mod statistics;

pub use cost_model::{CostModel, OperatorCost};
pub use planner::PlanNode as PlannerPlanNode;
pub use planner::{Planner, PlannerFilterPredicate, QueryPlan};
pub use statistics::{ColumnStatistics, Statistics};

#[derive(Debug, Clone)]
pub enum PlanNode {
    Scan {
        estimated_rows: usize,
    },
    Filter {
        input: Box<PlanNode>,
        selectivity: f64,
        estimated_rows: usize,
    },
    Project {
        input: Box<PlanNode>,
        estimated_rows: usize,
    },
    Join {
        left: Box<PlanNode>,
        right: Box<PlanNode>,
        estimated_rows: usize,
    },
    Aggregate {
        input: Box<PlanNode>,
        estimated_rows: usize,
    },
    Sort {
        input: Box<PlanNode>,
        column: ColumnID,
        ascending: bool,
        estimated_rows: usize,
    },
    Limit {
        input: Box<PlanNode>,
        count: usize,
        estimated_rows: usize,
    },
}

impl PlanNode {
    pub fn estimated_rows(&self) -> usize {
        match self {
            PlanNode::Scan { estimated_rows } => *estimated_rows,
            PlanNode::Filter { estimated_rows, .. } => *estimated_rows,
            PlanNode::Project { estimated_rows, .. } => *estimated_rows,
            PlanNode::Join { estimated_rows, .. } => *estimated_rows,
            PlanNode::Aggregate { estimated_rows, .. } => *estimated_rows,
            PlanNode::Sort { estimated_rows, .. } => *estimated_rows,
            PlanNode::Limit { estimated_rows, .. } => *estimated_rows,
        }
    }

    pub fn estimated_cost(&self) -> f64 {
        match self {
            PlanNode::Scan { estimated_rows } => *estimated_rows as f64,
            PlanNode::Filter {
                input,
                selectivity: _selectivity,
                estimated_rows,
            } => input.estimated_cost() + *estimated_rows as f64 * 0.01,
            PlanNode::Project { input, .. } => input.estimated_cost() * 1.05,
            PlanNode::Join {
                left,
                right,
                estimated_rows,
            } => left.estimated_cost() + right.estimated_cost() + *estimated_rows as f64 * 0.1,
            PlanNode::Aggregate {
                input,
                estimated_rows,
            } => input.estimated_cost() + *estimated_rows as f64 * 0.05,
            PlanNode::Sort {
                input,
                estimated_rows,
                ..
            } => input.estimated_cost() + (*estimated_rows as f64).log2() * *estimated_rows as f64,
            PlanNode::Limit { input, .. } => input.estimated_cost() * 0.5,
        }
    }

    pub fn depth(&self) -> usize {
        match self {
            PlanNode::Scan { .. } => 1,
            PlanNode::Filter { input, .. }
            | PlanNode::Project { input, .. }
            | PlanNode::Aggregate { input, .. }
            | PlanNode::Sort { input, .. }
            | PlanNode::Limit { input, .. } => 1 + input.depth(),
            PlanNode::Join { left, right, .. } => 1 + left.depth().max(right.depth()),
        }
    }

    pub fn explain(&self, indent: usize) -> String {
        let prefix = "  ".repeat(indent);
        match self {
            PlanNode::Scan { estimated_rows } => {
                format!("{}Scan (est. {} rows)", prefix, estimated_rows)
            }
            PlanNode::Filter {
                selectivity,
                estimated_rows,
                input,
            } => {
                format!(
                    "{}Filter (sel={:.2}, est. {} rows)\n{}",
                    prefix,
                    selectivity,
                    estimated_rows,
                    input.explain(indent + 1)
                )
            }
            PlanNode::Project {
                estimated_rows,
                input,
            } => {
                format!(
                    "{}Project (est. {} rows)\n{}",
                    prefix,
                    estimated_rows,
                    input.explain(indent + 1)
                )
            }
            PlanNode::Join {
                estimated_rows,
                left,
                right,
            } => {
                format!(
                    "{}NestedLoopJoin (est. {} rows)\n{}\n{}",
                    prefix,
                    estimated_rows,
                    left.explain(indent + 1),
                    right.explain(indent + 1)
                )
            }
            PlanNode::Aggregate {
                estimated_rows,
                input,
            } => {
                format!(
                    "{}Aggregate (est. {} groups)\n{}",
                    prefix,
                    estimated_rows,
                    input.explain(indent + 1)
                )
            }
            PlanNode::Sort {
                column,
                ascending,
                estimated_rows,
                input,
            } => {
                format!(
                    "{}Sort {:?} {} (est. {} rows)\n{}",
                    prefix,
                    column,
                    if *ascending { "ASC" } else { "DESC" },
                    estimated_rows,
                    input.explain(indent + 1)
                )
            }
            PlanNode::Limit {
                count,
                estimated_rows,
                input,
            } => {
                format!(
                    "{}Limit {} (est. {} rows)\n{}",
                    prefix,
                    count,
                    estimated_rows,
                    input.explain(indent + 1)
                )
            }
        }
    }
}

pub struct QueryOptimizer {
    enable_filter_pushdown: bool,
    enable_join_reorder: bool,
    #[allow(dead_code)]
    enable_projection_pushdown: bool,
}

impl QueryOptimizer {
    pub fn new() -> Self {
        QueryOptimizer {
            enable_filter_pushdown: true,
            enable_join_reorder: true,
            enable_projection_pushdown: true,
        }
    }

    pub fn with_filter_pushdown(mut self, enable: bool) -> Self {
        self.enable_filter_pushdown = enable;
        self
    }

    pub fn with_join_reorder(mut self, enable: bool) -> Self {
        self.enable_join_reorder = enable;
        self
    }

    pub fn optimize(&self, plan: PlanNode) -> PlanNode {
        let mut optimized = plan;

        if self.enable_filter_pushdown {
            optimized = self.push_down_filters(optimized);
        }

        if self.enable_join_reorder {
            optimized = self.reorder_joins(optimized);
        }

        optimized
    }

    fn push_down_filters(&self, plan: PlanNode) -> PlanNode {
        match plan {
            PlanNode::Filter {
                input,
                selectivity,
                estimated_rows,
            } => match *input {
                PlanNode::Join {
                    left,
                    right,
                    estimated_rows: join_rows,
                } => {
                    let left_est = (selectivity * left.estimated_rows() as f64) as usize;
                    let right_est = (selectivity * right.estimated_rows() as f64) as usize;

                    PlanNode::Join {
                        left: Box::new(self.push_down_filters(PlanNode::Filter {
                            input: left,
                            selectivity,
                            estimated_rows: left_est,
                        })),
                        right: Box::new(self.push_down_filters(PlanNode::Filter {
                            input: right,
                            selectivity,
                            estimated_rows: right_est,
                        })),
                        estimated_rows: estimated_rows.min(join_rows),
                    }
                }
                PlanNode::Project {
                    input,
                    estimated_rows: proj_rows,
                } => PlanNode::Project {
                    input: Box::new(self.push_down_filters(PlanNode::Filter {
                        input,
                        selectivity,
                        estimated_rows,
                    })),
                    estimated_rows: proj_rows.min(estimated_rows),
                },
                other => PlanNode::Filter {
                    input: Box::new(self.push_down_filters(other)),
                    selectivity,
                    estimated_rows,
                },
            },
            PlanNode::Join {
                left,
                right,
                estimated_rows,
            } => PlanNode::Join {
                left: Box::new(self.push_down_filters(*left)),
                right: Box::new(self.push_down_filters(*right)),
                estimated_rows,
            },
            PlanNode::Project {
                input,
                estimated_rows,
            } => PlanNode::Project {
                input: Box::new(self.push_down_filters(*input)),
                estimated_rows,
            },
            PlanNode::Aggregate {
                input,
                estimated_rows,
            } => PlanNode::Aggregate {
                input: Box::new(self.push_down_filters(*input)),
                estimated_rows,
            },
            PlanNode::Sort {
                input,
                column,
                ascending,
                estimated_rows,
            } => PlanNode::Sort {
                input: Box::new(self.push_down_filters(*input)),
                column,
                ascending,
                estimated_rows,
            },
            PlanNode::Limit {
                input,
                count,
                estimated_rows,
            } => PlanNode::Limit {
                input: Box::new(self.push_down_filters(*input)),
                count,
                estimated_rows,
            },
            other => other,
        }
    }

    fn reorder_joins(&self, plan: PlanNode) -> PlanNode {
        match plan {
            PlanNode::Join {
                left,
                right,
                estimated_rows,
            } => {
                let left_cost = left.estimated_cost();
                let right_cost = right.estimated_cost();

                if right_cost < left_cost {
                    PlanNode::Join {
                        left: Box::new(self.reorder_joins(*right)),
                        right: Box::new(self.reorder_joins(*left)),
                        estimated_rows,
                    }
                } else {
                    PlanNode::Join {
                        left: Box::new(self.reorder_joins(*left)),
                        right: Box::new(self.reorder_joins(*right)),
                        estimated_rows,
                    }
                }
            }
            PlanNode::Filter {
                input,
                selectivity,
                estimated_rows,
            } => PlanNode::Filter {
                input: Box::new(self.reorder_joins(*input)),
                selectivity,
                estimated_rows,
            },
            other => other,
        }
    }

    /// Estimate selectivity for a column filter.
    /// Currently uses a simplified model based on column cardinality.
    /// The `value` parameter is reserved for future histogram-based estimation.
    #[allow(dead_code)]
    pub fn estimate_selectivity(&self, schema: &Schema, column: ColumnID, _value: &str) -> f64 {
        let total = schema.len() as f64;
        if total == 0.0 {
            return 0.0;
        }

        let matching = match column {
            ColumnID::Subject => schema.subject_col.iter().filter(|&&v| v == 0).count(),
            ColumnID::Predicate => schema.predicate_col.iter().filter(|&&v| v == 0).count(),
            ColumnID::Object => schema.object_col.iter().filter(|&&v| v == 0).count(),
            _ => 0,
        };

        matching as f64 / total
    }
}

impl Default for QueryOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plan_node_scan() {
        let plan = PlanNode::Scan {
            estimated_rows: 1000,
        };
        assert_eq!(plan.estimated_rows(), 1000);
        assert_eq!(plan.estimated_cost(), 1000.0);
        assert_eq!(plan.depth(), 1);
    }

    #[test]
    fn test_plan_node_filter() {
        let plan = PlanNode::Filter {
            input: Box::new(PlanNode::Scan {
                estimated_rows: 1000,
            }),
            selectivity: 0.1,
            estimated_rows: 100,
        };
        assert_eq!(plan.estimated_rows(), 100);
        assert_eq!(plan.depth(), 2);
    }

    #[test]
    fn test_plan_node_join() {
        let plan = PlanNode::Join {
            left: Box::new(PlanNode::Scan {
                estimated_rows: 100,
            }),
            right: Box::new(PlanNode::Scan {
                estimated_rows: 200,
            }),
            estimated_rows: 50,
        };
        assert_eq!(plan.estimated_rows(), 50);
        assert_eq!(plan.depth(), 2);
    }

    #[test]
    fn test_optimizer_push_down_filters() {
        let optimizer = QueryOptimizer::new();
        let plan = PlanNode::Filter {
            input: Box::new(PlanNode::Join {
                left: Box::new(PlanNode::Scan {
                    estimated_rows: 1000,
                }),
                right: Box::new(PlanNode::Scan {
                    estimated_rows: 500,
                }),
                estimated_rows: 100,
            }),
            selectivity: 0.1,
            estimated_rows: 10,
        };

        let optimized = optimizer.optimize(plan);
        match optimized {
            PlanNode::Join { left, right, .. } => {
                assert!(matches!(*left, PlanNode::Filter { .. }));
                assert!(matches!(*right, PlanNode::Filter { .. }));
            }
            _ => panic!("Expected Join after filter pushdown"),
        }
    }

    #[test]
    fn test_optimizer_reorder_joins() {
        let optimizer = QueryOptimizer::new().with_filter_pushdown(false);
        let plan = PlanNode::Join {
            left: Box::new(PlanNode::Scan {
                estimated_rows: 10000,
            }),
            right: Box::new(PlanNode::Scan {
                estimated_rows: 100,
            }),
            estimated_rows: 50,
        };

        let optimized = optimizer.optimize(plan);
        match optimized {
            PlanNode::Join { left, right, .. } => {
                let left_rows = left.estimated_rows();
                let right_rows = right.estimated_rows();
                assert!(left_rows <= right_rows);
            }
            _ => panic!("Expected Join after reorder"),
        }
    }

    #[test]
    fn test_optimizer_explain() {
        let plan = PlanNode::Filter {
            input: Box::new(PlanNode::Scan {
                estimated_rows: 1000,
            }),
            selectivity: 0.1,
            estimated_rows: 100,
        };
        let explain = plan.explain(0);
        assert!(explain.contains("Filter"));
        assert!(explain.contains("Scan"));
    }

    #[test]
    fn test_optimizer_sort() {
        let plan = PlanNode::Sort {
            input: Box::new(PlanNode::Scan {
                estimated_rows: 1000,
            }),
            column: ColumnID::Subject,
            ascending: true,
            estimated_rows: 1000,
        };
        assert_eq!(plan.estimated_rows(), 1000);
        assert!(plan.estimated_cost() > 1000.0);
    }

    #[test]
    fn test_optimizer_limit() {
        let plan = PlanNode::Limit {
            input: Box::new(PlanNode::Scan {
                estimated_rows: 10000,
            }),
            count: 100,
            estimated_rows: 100,
        };
        assert_eq!(plan.estimated_rows(), 100);
        assert!(plan.estimated_cost() < 10000.0);
    }
}
