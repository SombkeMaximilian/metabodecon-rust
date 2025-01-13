use crate::deconvolution::error::{Error, Kind};
use crate::error::Result;
use crate::peak_selection::detector::Detector;
use crate::peak_selection::peak::Peak;
use crate::peak_selection::scorer::{Scorer, ScorerMinimumSum, ScoringAlgo};
use crate::peak_selection::selector::Selector;
use crate::spectrum::Spectrum;

/// Peak selection algorithm based on the score of peaks found in the signal
/// free region.
#[derive(Debug)]
pub struct NoiseScoreFilter {
    /// The scoring method to use.
    scoring_algo: ScoringAlgo,
    /// The threshold for filtering peaks.
    threshold: f64,
}

impl Selector for NoiseScoreFilter {
    /// Detects peaks in a spectrum and returns the ones that pass a filter.
    ///
    /// Peaks are detected using the curvature of the signal through the second
    /// derivative. The scores of the peaks are computed using the selected
    /// scoring algorithm. The mean and standard deviation of the scores in the
    /// signal free region (where only noise is present) are calculated, and
    /// peaks in the signal region are filtered according to the following
    /// criterion:
    ///
    /// ```text
    /// score > mean + threshold * std_dev
    /// ```
    fn select_peaks(&self, spectrum: &Spectrum) -> Result<Vec<Peak>> {
        let signal_boundaries = spectrum.signal_boundaries_indices();
        let mut second_derivative = Self::second_derivative(spectrum.intensities());
        let peaks = {
            let detector = Detector::new(&second_derivative);
            detector.detect_peaks()?
        };
        second_derivative
            .iter_mut()
            .for_each(|d| *d = d.abs());

        self.filter_peaks(peaks, &second_derivative, signal_boundaries)
    }
}

impl NoiseScoreFilter {
    /// Constructs a new `NoiseScoreFilter` with the given scoring algorithm and
    /// threshold.
    pub fn new(scoring_algo: ScoringAlgo, threshold: f64) -> Self {
        Self {
            scoring_algo,
            threshold,
        }
    }

    /// Computes the second derivative of a signal with 3-point finite
    /// differences.
    fn second_derivative(intensities: &[f64]) -> Vec<f64> {
        intensities
            .windows(3)
            .map(|w| w[0] - 2.0 * w[1] + w[2])
            .collect()
    }

    /// Filters peaks based on their scores.
    ///
    /// The scores are computed using the selected scoring algorithm, and then
    /// divided into signal free region (SFR) and signal region. The mean and
    /// standard deviation of the scores in the SFR, where only noise is
    /// present, are calculated, and peaks in the signal region are filtered
    /// according to the following criterion:
    ///
    /// ```text
    /// score > mean + threshold * std_dev
    /// ```
    fn filter_peaks(
        &self,
        mut peaks: Vec<Peak>,
        abs_second_derivative: &[f64],
        signal_boundaries: (usize, usize),
    ) -> Result<Vec<Peak>> {
        let scorer = match self.scoring_algo {
            ScoringAlgo::MinimumSum => ScorerMinimumSum::new(abs_second_derivative),
        };
        let boundaries = Self::peak_region_boundaries(&peaks, signal_boundaries)?;

        if peaks[..boundaries.0].is_empty() && peaks[boundaries.1..].is_empty() {
            return Err(Error::new(Kind::EmptySignalFreeRegion).into());
        }
        if peaks[boundaries.0..boundaries.1].is_empty() {
            return Err(Error::new(Kind::EmptySignalRegion).into());
        }

        let scores_sfr: Vec<f64> = peaks[0..boundaries.0]
            .iter()
            .chain(peaks[boundaries.1..].iter())
            .map(|peak| scorer.score_peak(peak))
            .collect();
        let (mean, sd) = Self::mean_sd_scores(scores_sfr);

        peaks = peaks
            .drain(boundaries.0..boundaries.1)
            .filter(|peak| scorer.score_peak(peak) >= mean + self.threshold * sd)
            .collect();

        if peaks.is_empty() {
            return Err(Error::new(Kind::EmptySignalRegion).into());
        }

        Ok(peaks)
    }

    /// Computes the indices in the slice of `Peak`s that delimit the signal
    /// region.
    fn peak_region_boundaries(
        peaks: &[Peak],
        signal_boundaries: (usize, usize),
    ) -> Result<(usize, usize)> {
        let left = peaks
            .iter()
            .position(|peak| peak.center() > signal_boundaries.0)
            .map_or(0, |i| i);
        let right = peaks[left..]
            .iter()
            .position(|peak| peak.center() > signal_boundaries.1)
            .map_or(peaks.len() - 1, |i| left + i);
        Ok((left, right))
    }

    /// Computes the mean and standard deviation of a vector of scores.
    fn mean_sd_scores(scores: Vec<f64>) -> (f64, f64) {
        let mean: f64 = scores.iter().sum::<f64>() / scores.len() as f64;
        let variance: f64 = scores
            .iter()
            .map(|score| (score - mean).powi(2))
            .sum::<f64>()
            / scores.len() as f64;
        (mean, variance.sqrt())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::assert_approx_eq;

    #[test]
    fn second_derivative() {
        let intensities = vec![1.0, 2.0, 3.0, 2.0, 1.0];
        let expected_second_derivative = [0.0, -2.0, 0.0];
        let computed_second_derivative = NoiseScoreFilter::second_derivative(&intensities);
        computed_second_derivative
            .iter()
            .zip(expected_second_derivative.iter())
            .for_each(|(&sdc, &sde)| assert_approx_eq!(f64, sdc, sde));
    }

    #[test]
    fn peak_region_boundaries() {
        let signal_region_boundaries: (usize, usize) = (3, 7);
        let peaks: Vec<Peak> = vec![2, 4, 5, 8]
            .into_iter()
            .map(|i| Peak::new(i - 1, i, i + 1))
            .collect();
        assert_eq!(
            NoiseScoreFilter::peak_region_boundaries(&peaks, signal_region_boundaries).unwrap(),
            (1, 3)
        );
    }

    #[test]
    fn mean_sd_scores() {
        let peaks: Vec<Peak> = vec![2, 4, 5, 8]
            .into_iter()
            .map(|i| Peak::new(i - 1, i, i + 1))
            .collect();
        let abs_second_derivative = vec![1.0, 2.0, 4.0, 2.0, 2.0, 5.0, 4.0, 3.0, 2.0];
        let scorer = ScorerMinimumSum::new(&abs_second_derivative);
        let peak_region_boundaries = (1, 3);
        let scores_sfr: Vec<f64> = peaks[0..peak_region_boundaries.0]
            .iter()
            .chain(peaks[peak_region_boundaries.1..].iter())
            .map(|peak| scorer.score_peak(peak))
            .collect();
        let (mean, sd) = NoiseScoreFilter::mean_sd_scores(scores_sfr);
        assert_approx_eq!(f64, mean, 4.0);
        assert_approx_eq!(f64, sd, 1.0);
    }
}
