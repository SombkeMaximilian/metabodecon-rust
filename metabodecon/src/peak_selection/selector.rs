use crate::error::Result;
use crate::peak_selection::peak::Peak;
use crate::peak_selection::scorer::ScoringAlgo;
use crate::spectrum::Spectrum;

/// Peak selection methods.
#[non_exhaustive]
#[derive(Copy, Clone, Debug)]
pub enum SelectionAlgo {
    /// Filter based on the score of peaks found in the signal free region.
    ///
    /// Finds peaks in the spectrum by analyzing the curvature of the signal
    /// through the second derivative and scores them based on the selected
    /// scoring algorithm. Mean and standard deviation are calculated for the
    /// scores of peaks in the signal free region (where only noise is present).
    /// Finally, peaks in the signal region are filtered according to the
    /// following criterion:
    ///
    /// ```text
    /// score > mean + threshold * std_dev
    /// ```
    NoiseScoreFilter {
        /// The scoring algorithm to use.
        scoring_algo: ScoringAlgo,
        /// The threshold to apply to the scores.
        threshold: f64,
    },
}

/// Trait interface for peak selection algorithms.
pub trait Selector {
    /// Detects peaks in a spectrum and returns the ones that pass a filter.
    fn select_peaks(&self, spectrum: &Spectrum) -> Result<Vec<Peak>>;
}
