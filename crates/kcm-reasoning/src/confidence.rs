pub struct ConfidenceCalculator;

impl ConfidenceCalculator {
    pub fn conjunction(a: f64, b: f64) -> f64 {
        (a * b).clamp(0.0, 1.0)
    }

    pub fn disjunction(a: f64, b: f64) -> f64 {
        (a + b - (a * b)).clamp(0.0, 1.0)
    }

    pub fn negation(a: f64) -> f64 {
        (1.0 - a).clamp(0.0, 1.0)
    }

    pub fn chain(values: &[f64]) -> f64 {
        values.iter().copied().fold(1.0, Self::conjunction)
    }

    pub fn weighted(values: &[f64], weights: &[f64]) -> f64 {
        if values.len() != weights.len() || values.is_empty() {
            return 0.0;
        }
        let numerator: f64 = values.iter().zip(weights.iter()).map(|(v, w)| v * w).sum();
        let denominator: f64 = weights.iter().sum();

        if denominator == 0.0 {
            0.0
        } else {
            (numerator / denominator).clamp(0.0, 1.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_confidence_operations() {
        assert!((ConfidenceCalculator::conjunction(0.5, 0.6) - 0.3).abs() < 0.0001);
        assert!((ConfidenceCalculator::disjunction(0.5, 0.6) - 0.8).abs() < 0.0001);
        assert!((ConfidenceCalculator::negation(0.7) - 0.3).abs() < 0.0001);
    }
}
