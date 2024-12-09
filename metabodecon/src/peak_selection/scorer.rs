use crate::peak_selection::peak::Peak;

#[derive(Clone, Copy, Debug)]
pub enum ScoringAlgo {
    MinimumSum,
}

pub trait Scorer {
    fn score_peak(&self, peak: &Peak) -> f64;
}

#[derive(Debug)]
pub struct ScorerMinimumSum<'a> {
    abs_second_derivative: &'a [f64],
}

impl Scorer for ScorerMinimumSum<'_> {
    fn score_peak(&self, peak: &Peak) -> f64 {
        f64::min(
            self.abs_second_derivative[peak.left() - 1..peak.center()]
                .iter()
                .sum(),
            self.abs_second_derivative[peak.center() - 1..peak.right()]
                .iter()
                .sum(),
        )
    }
}

impl<'a> ScorerMinimumSum<'a> {
    pub fn new(abs_second_derivative: &'a [f64]) -> Self {
        ScorerMinimumSum {
            abs_second_derivative,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::peak_selection::Peak;

    #[test]
    fn minimum_sum() {
        let peaks = [Peak::new(1, 3, 4), Peak::new(5, 6, 9)];
        let abs_second_derivative = vec![1., 2., 4., 2., 2., 5., 4., 3., 2.];
        let scorer = ScorerMinimumSum::new(&abs_second_derivative);
        let scores: Vec<f64> = peaks
            .iter()
            .map(|peak| scorer.score_peak(peak))
            .collect();
        assert_eq!(scores, vec![6., 7.]);
    }
}
