use crate::peak_selection::peak::Peak;

/// Scoring methods for the peaks.
#[non_exhaustive]
#[derive(Copy, Clone, Debug)]
pub enum ScoringAlgo {
    /// Minimum Sum of the absolute second derivative.
    ///
    /// The minimum sum scorer computes the sum of the absolute second
    /// derivative values of the peak at the left and right sides of the
    /// peak center. The smaller of the two sums is the score of the peak.
    MinimumSum,
}

/// Trait interface for peak scoring algorithms.
pub trait Scorer {
    /// Scores the given peak.
    fn score_peak(&self, peak: &Peak) -> f64;
}

/// Peak scoring algorithm based on the sum of second derivative values.
#[derive(Debug)]
pub struct ScorerMinimumSum<'a> {
    /// The absolute second derivative of the signal as a reference.
    abs_second_derivative: &'a [f64],
}

impl Scorer for ScorerMinimumSum<'_> {
    /// Scores the given peak by computing the sum of the absolute second
    /// derivative values at the left and right sides of the peak center.
    /// Returns the smaller of the two sums as the score of the peak.
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
    /// Creates a new `ScorerMinimumSum` with the given absolute second
    /// derivative.
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
