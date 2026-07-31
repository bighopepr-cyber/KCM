#[derive(Debug, Clone)]
pub struct OperatorCost {
    pub cpu_cost: f64,
    pub io_cost: f64,
    pub memory_cost: f64,
    pub estimated_rows: usize,
}

impl OperatorCost {
    pub fn total(&self) -> f64 {
        self.cpu_cost * 1.0 + self.io_cost * 10.0 + self.memory_cost * 0.1
    }
}

pub struct CostModel {
    row_count: usize,
}

impl CostModel {
    pub fn new(row_count: usize) -> Self {
        CostModel { row_count }
    }

    pub fn estimate_scan(&self, selectivity: f64) -> OperatorCost {
        let estimated_rows = (self.row_count as f64 * selectivity) as usize;
        let cpu_cost = self.row_count as f64 / 1_000_000.0;
        OperatorCost {
            cpu_cost,
            io_cost: 0.0,
            memory_cost: estimated_rows as f64 / 1_000_000.0,
            estimated_rows,
        }
    }

    pub fn estimate_filter(&self, input_rows: usize, selectivity: f64) -> OperatorCost {
        let output_rows = (input_rows as f64 * selectivity) as usize;
        OperatorCost {
            cpu_cost: input_rows as f64 / 2_000_000.0,
            io_cost: 0.0,
            memory_cost: 0.0,
            estimated_rows: output_rows,
        }
    }

    pub fn estimate_join(
        &self,
        left_rows: usize,
        right_rows: usize,
        join_selectivity: f64,
    ) -> OperatorCost {
        let output_rows = (left_rows as f64 * right_rows as f64 * join_selectivity) as usize;
        let cpu_cost = (left_rows + right_rows) as f64 / 1_000_000.0;
        OperatorCost {
            cpu_cost,
            io_cost: 0.0,
            memory_cost: (left_rows + right_rows) as f64 / 1_000_000.0,
            estimated_rows: output_rows,
        }
    }

    pub fn estimate_aggregate(&self, input_rows: usize, num_groups: usize) -> OperatorCost {
        OperatorCost {
            cpu_cost: input_rows as f64 / 1_000_000.0,
            io_cost: 0.0,
            memory_cost: num_groups as f64 / 1_000_000.0,
            estimated_rows: num_groups,
        }
    }

    pub fn estimate_infer(&self, input_rows: usize, rule_complexity: f64) -> OperatorCost {
        let cpu_cost = input_rows as f64 * rule_complexity / 100_000.0;
        OperatorCost {
            cpu_cost,
            io_cost: 0.0,
            memory_cost: 0.0,
            estimated_rows: (input_rows as f64 * 1.5) as usize,
        }
    }
}
