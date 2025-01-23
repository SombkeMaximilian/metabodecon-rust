//! Error types for the deconvolution process.

use crate::deconvolution::fitting::FittingAlgo;
use crate::deconvolution::peak_selection::SelectionAlgo;
use crate::deconvolution::smoothing::SmoothingAlgo;

/// An `Error` that occurred during the deconvolution process.
///
/// This type of error is generally unrecoverable and indicates a problem with
/// the input data. For example, if no peaks are detected in the input data,
/// or there are no signals in the part of the spectrum where they would be
/// expected.
///
/// See the [`Kind`] enum for the different kinds of errors that can occur.
#[derive(Clone, Debug)]
pub struct Error {
    /// The `Kind` of error that occurred.
    kind: Kind,
}

impl Error {
    /// Constructs a new `Error` from the given `Kind`.
    pub fn new(kind: Kind) -> Self {
        kind.into()
    }

    /// Returns the `Kind` of the `Error`.
    pub fn kind(&self) -> &Kind {
        &self.kind
    }
}

/// The kind of `Error` that can occur during the [`deconvolution`] process.
///
/// Marked as non-exhaustive to allow for new variants to be added in the future
/// without breaking compatibility.
///
/// [`deconvolution`]: crate::deconvolution
#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum Kind {
    /// The provided smoothing settings are invalid.
    ///
    /// Some configurations, such as a `window_size` of 0 for a moving
    /// average filter, are invalid.
    InvalidSmoothingSettings {
        /// The provided smoothing settings.
        algo: SmoothingAlgo,
    },
    /// The provided peak selection settings are invalid.
    ///
    /// Some configurations, such as a negative `threshold` for a noise score
    /// filter, are invalid.
    InvalidSelectionSettings {
        /// The provided peak selection settings.
        algo: SelectionAlgo,
    },
    /// The provided fitting settings are invalid.
    ///
    /// Some configurations, such as 0 `iterations` for an analytical fitting
    /// algorithm, are invalid.
    InvalidFittingSettings {
        /// The provided fitting settings.
        algo: FittingAlgo,
    },
    /// The provided region to be ignored is invalid.
    ///
    /// The region must be a tuple of two finite floating point numbers, whose
    /// absolute difference is greater than 100 times the floating point
    /// precision.
    InvalidIgnoreRegion {
        /// The provided ignore region.
        region: (f64, f64),
    },
    /// No peaks were detected in the input data.
    ///
    /// Most of the time this will happen if the intensities of the [`Spectrum`]
    /// were not read correctly or if something went wrong during the smoothing
    /// process.
    ///
    /// [`Spectrum`]: crate::spectrum::Spectrum
    NoPeaksDetected,
    /// No peaks were found in the part of the spectrum where signals would be
    /// expected.
    ///
    /// This can happen if there were no signals in the signal region or if all
    /// the signals were filtered out by the peak selection algorithm if the
    /// threshold was set too high.
    EmptySignalRegion,
    /// No peaks were found in the part of the spectrum where random signals due
    /// to noise would be expected.
    ///
    /// This is an error because the deconvolution process uses the noise peaks
    /// to estimate the noise level in the spectrum and filter out peaks that
    /// are likely to be random noise.
    EmptySignalFreeRegion,
}

impl std::error::Error for Error {}

impl From<Kind> for Error {
    fn from(kind: Kind) -> Self {
        Self { kind }
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        use self::Kind::*;
        let description = match &self.kind {
            InvalidSmoothingSettings { algo } => match algo {
                SmoothingAlgo::MovingAverage { .. } => format!(
                    "invalid smoothing settings: {:?} \
                     window_size and iterations must be greater than 0",
                    algo
                ),
            },
            InvalidSelectionSettings { algo } => match algo {
                SelectionAlgo::NoiseScoreFilter { .. } => format!(
                    "invalid peak selection settings: {:?} \
                     threshold must be greater than 0",
                    algo
                ),
            },
            InvalidFittingSettings { algo } => match algo {
                FittingAlgo::Analytical { .. } => format!(
                    "invalid fitting settings: {:?} \
                     iterations must be greater than 0",
                    algo
                ),
            },
            InvalidIgnoreRegion { region } => format!(
                "invalid ignore region: {:?} \
                 the region must be a tuple of two finite floating point numbers \
                 with a difference that is not near 0",
                region
            ),
            NoPeaksDetected => "no peaks detected in the spectrum".to_string(),
            EmptySignalRegion => "no peaks found in the signal region of the spectrum".to_string(),
            EmptySignalFreeRegion => {
                "no peaks found in the signal-free region of the spectrum".to_string()
            }
        };
        write!(f, "{description}")
    }
}
