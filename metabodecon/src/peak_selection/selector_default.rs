use crate::peak_selection::detector::Detector;
use crate::peak_selection::error::{Error, Kind, Result};
use crate::peak_selection::peak::Peak;
use crate::peak_selection::scorer::{Scorer, ScorerMinimumSum, ScoringAlgo};
use crate::peak_selection::selector::Selector;
use crate::spectrum::Spectrum;

#[derive(Debug)]
pub struct SelectorDefault {
    scoring_algo: ScoringAlgo,
    threshold: f64,
}

impl Selector for SelectorDefault {
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

        Ok(self.filter_peaks(peaks, &second_derivative, signal_boundaries)?)
    }
}

impl SelectorDefault {
    pub fn new(scoring_algo: ScoringAlgo, threshold: f64) -> Self {
        Self {
            scoring_algo,
            threshold,
        }
    }

    fn second_derivative(intensities: &[f64]) -> Vec<f64> {
        intensities
            .windows(3)
            .map(|w| w[0] - 2. * w[1] + w[2])
            .collect()
    }

    fn filter_peaks(
        &self,
        mut peaks: Vec<Peak>,
        abs_second_derivative: &[f64],
        signal_boundaries: (usize, usize),
    ) -> Result<Vec<Peak>> {
        let scorer = match self.scoring_algo {
            ScoringAlgo::MinimumSum => ScorerMinimumSum::new(abs_second_derivative),
        };
        let boundaries = Self::peak_region_boundaries(&peaks, signal_boundaries);
        if peaks[..boundaries.0].is_empty() && peaks[boundaries.1..].is_empty() {
            return Err(Error::new(Kind::EmptySignalFreeRegion));
        }
        if peaks[boundaries.0..boundaries.1].is_empty() {
            return Err(Error::new(Kind::EmptySignalRegion));
        }

        let scores_sfr: Vec<f64> = peaks[0..boundaries.0]
            .iter()
            .chain(peaks[boundaries.1..].iter())
            .map(|peak| scorer.score_peak(peak))
            .collect();
        let (mean, sd) = Self::mean_sd_scores(scores_sfr);
        Ok(peaks
            .drain(boundaries.0..boundaries.1)
            .filter(|peak| scorer.score_peak(peak) >= mean + self.threshold * sd)
            .collect())
    }

    fn peak_region_boundaries(peaks: &[Peak], signal_boundaries: (usize, usize)) -> (usize, usize) {
        let left = peaks
            .iter()
            .position(|peak| peak.center() > signal_boundaries.0)
            .unwrap();
        let right = peaks[left..]
            .iter()
            .position(|peak| peak.center() > signal_boundaries.1)
            .map(|i| left + i)
            .unwrap();
        (left, right)
    }

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
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn second_derivative() {
        let intensities = vec![1., 2., 3., 2., 1.];
        let expected = vec![0., -2., 0.];
        assert_eq!(SelectorDefault::second_derivative(&intensities), expected);
    }

    #[test]
    fn peak_region_boundaries() {
        let signal_region_boundaries: (usize, usize) = (3, 7);
        let peaks: Vec<Peak> = vec![2, 4, 5, 8]
            .into_iter()
            .map(|i| Peak::new(i - 1, i, i + 1))
            .collect();
        assert_eq!(
            SelectorDefault::peak_region_boundaries(&peaks, signal_region_boundaries),
            (1, 3)
        );
    }

    #[test]
    fn mean_sd_scores() {
        let peaks: Vec<Peak> = vec![2, 4, 5, 8]
            .into_iter()
            .map(|i| Peak::new(i - 1, i, i + 1))
            .collect();
        let abs_second_derivative = vec![1., 2., 4., 2., 2., 5., 4., 3., 2.];
        let scorer = ScorerMinimumSum::new(&abs_second_derivative);
        let peak_region_boundaries = (1, 3);
        let scores_sfr: Vec<f64> = peaks[0..peak_region_boundaries.0]
            .iter()
            .chain(peaks[peak_region_boundaries.1..].iter())
            .map(|peak| scorer.score_peak(peak))
            .collect();
        let (mean, sd) = SelectorDefault::mean_sd_scores(scores_sfr);
        assert_approx_eq!(mean, 4.0);
        assert_approx_eq!(sd, 1.0);
    }
}
