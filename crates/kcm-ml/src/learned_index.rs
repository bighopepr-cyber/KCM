pub struct RegressionModel {
    slope: f64,
    intercept: f64,
}

impl RegressionModel {
    pub fn new() -> Self {
        RegressionModel {
            slope: 0.0,
            intercept: 0.0,
        }
    }

    pub fn train(&mut self, x_values: &[u32], y_positions: &[usize]) {
        let n = x_values.len();
        if n == 0 {
            return;
        }
        let x_mean = x_values.iter().map(|&x| x as f64).sum::<f64>() / n as f64;
        let y_mean = y_positions.iter().map(|&y| y as f64).sum::<f64>() / n as f64;
        let mut cov = 0.0;
        let mut var = 0.0;
        for (x, y) in x_values.iter().zip(y_positions.iter()) {
            let xf = *x as f64;
            let yf = *y as f64;
            cov += (xf - x_mean) * (yf - y_mean);
            var += (xf - x_mean) * (xf - x_mean);
        }
        self.slope = if var > 0.0 { cov / var } else { 0.0 };
        self.intercept = y_mean - self.slope * x_mean;
    }

    pub fn predict(&self, value: u32) -> usize {
        let y = self.slope * value as f64 + self.intercept;
        y.max(0.0) as usize
    }
}

impl Default for RegressionModel {
    fn default() -> Self {
        Self::new()
    }
}

pub struct LearnedIndex {
    models: Vec<RegressionModel>,
    ranges: Vec<(u32, u32)>,
    model_count: usize,
}

impl LearnedIndex {
    pub fn new(model_count: usize) -> Self {
        LearnedIndex {
            models: (0..model_count).map(|_| RegressionModel::new()).collect(),
            ranges: Vec::new(),
            model_count,
        }
    }

    pub fn train(&mut self, values: &[u32], positions: &[usize]) {
        if self.model_count == 0 || values.is_empty() {
            return;
        }
        let chunk_size = values.len().div_ceil(self.model_count);
        self.ranges.clear();
        for (i, model) in self.models.iter_mut().enumerate() {
            let start = i * chunk_size;
            let end = ((i + 1) * chunk_size).min(values.len());
            if start < end {
                model.train(&values[start..end], &positions[start..end]);
                self.ranges.push((values[start], values[end - 1]));
            }
        }
    }

    pub fn search(&self, value: u32) -> (usize, usize) {
        let model_idx = self
            .ranges
            .partition_point(|&(start, _)| start <= value)
            .saturating_sub(1)
            .min(self.models.len().saturating_sub(1));
        let predicted = self.models[model_idx].predict(value);
        let lower = predicted.saturating_sub(100);
        let upper = predicted.saturating_add(100);
        (lower, upper)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regression_model_linear() {
        let mut model = RegressionModel::new();
        let x = vec![0u32, 1, 2, 3, 4];
        let y = vec![0usize, 10, 20, 30, 40];
        model.train(&x, &y);
        assert!((model.predict(2) as f64 - 20.0).abs() < 1.0);
    }

    #[test]
    fn test_regression_model_empty() {
        let mut model = RegressionModel::new();
        model.train(&[], &[]);
        assert_eq!(model.predict(0), 0);
    }

    #[test]
    fn test_learned_index_search() {
        let mut index = LearnedIndex::new(2);
        let values: Vec<u32> = (0..1000).collect();
        let positions: Vec<usize> = (0..1000).collect();
        index.train(&values, &positions);
        let (lower, upper) = index.search(500);
        assert!(lower <= 500);
        assert!(upper >= 500);
    }

    #[test]
    fn test_learned_index_empty() {
        let index = LearnedIndex::new(2);
        let (lower, upper) = index.search(0);
        assert_eq!(lower, 0);
        assert!(upper >= lower);
    }
}
