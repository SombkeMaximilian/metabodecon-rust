use crate::Result;
use crate::deconvolution::Settings;
use crate::deconvolution::error::{Error, Kind};
use crate::deconvolution::peak_selection::{Peak, ScoringMethod};

/// Trait interface for peak selection algorithms.
pub(crate) trait Selector {
    /// Detects peaks in a spectrum and returns the ones that pass a filter.
    fn select_peaks(
        &self,
        intensities: &[f64],
        signal_boundaries: (usize, usize),
        ignore_regions: Option<&[(usize, usize)]>,
    ) -> Result<Vec<Peak>>;
}

/// Peak selection methods.
#[non_exhaustive]
#[derive(Copy, Clone, Debug)]
pub enum SelectionSettings {
    /// Filter based on the score of peaks found in the signal free region.
    ///
    /// Finds peaks in the spectrum by analyzing the curvature of the signal
    /// through the second derivative and scores them based on the selected
    /// scoring method. Mean and standard deviation are calculated for the
    /// scores of peaks in the signal free region (where only noise is present).
    /// Finally, peaks in the signal region are filtered according to the
    /// following criterion:
    ///
    /// ```text
    /// score > mean + threshold * std_dev
    /// ```
    NoiseScoreFilter {
        /// The scoring method to use.
        scoring_method: ScoringMethod,
        /// The threshold to apply to the scores.
        threshold: f64,
    },
}

impl Default for SelectionSettings {
    fn default() -> Self {
        SelectionSettings::NoiseScoreFilter {
            scoring_method: ScoringMethod::default(),
            threshold: 6.4,
        }
    }
}

impl std::fmt::Display for SelectionSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SelectionSettings::NoiseScoreFilter {
                scoring_method,
                threshold,
            } => write!(
                f,
                "Noise Score Filter [scoring method: {}, score threshold: {}]",
                scoring_method, threshold
            ),
        }
    }
}

impl Settings for SelectionSettings {
    fn validate(&self) -> Result<()> {
        match self {
            SelectionSettings::NoiseScoreFilter { threshold, .. } => {
                if *threshold < 0.0 {
                    return Err(
                        Error::new(Kind::InvalidSelectionSettings { settings: *self }).into(),
                    );
                }
            }
        }

        Ok(())
    }
}
