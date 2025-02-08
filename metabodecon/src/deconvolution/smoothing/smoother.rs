use crate::deconvolution::Settings;
use crate::deconvolution::error::{Error, Kind};
use crate::error::Result;

/// Trait interface for smoothing algorithms.
pub(crate) trait Smoother<Type> {
    /// Smooths the given sequence of values in place.
    fn smooth_values(&mut self, values: &mut [Type]);
}

/// Smoothing methods for the signal intensities.
#[non_exhaustive]
#[derive(Copy, Clone, Debug)]
pub enum SmoothingSettings {
    /// Moving average low-pass filter.
    ///
    /// The moving average filter is a low-pass filter that replaces each value
    /// in the sequence with the average of the values in a sliding window
    /// centered around the value.
    MovingAverage {
        /// The number of iterations to apply the filter.
        iterations: usize,
        /// The number of values in the sliding window.
        window_size: usize,
    },
}

impl Default for SmoothingSettings {
    fn default() -> Self {
        SmoothingSettings::MovingAverage {
            iterations: 2,
            window_size: 5,
        }
    }
}

impl std::fmt::Display for SmoothingSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SmoothingSettings::MovingAverage {
                iterations,
                window_size,
            } => write!(
                f,
                "Moving Average Filter [number of iterations: {}, window size: {}]",
                iterations, window_size
            ),
        }
    }
}

impl Settings for SmoothingSettings {
    fn validate(&self) -> Result<()> {
        match self {
            SmoothingSettings::MovingAverage {
                iterations,
                window_size,
            } => {
                if *iterations == 0 || *window_size == 0 {
                    return Err(
                        Error::new(Kind::InvalidSmoothingSettings { settings: *self }).into(),
                    );
                }
            }
        }

        Ok(())
    }

    #[cfg(test)]
    fn compare(&self, other: &Self) -> bool {
        match (self, other) {
            (
                SmoothingSettings::MovingAverage {
                    iterations: iterations1,
                    window_size: window_size1,
                },
                SmoothingSettings::MovingAverage {
                    iterations: iterations2,
                    window_size: window_size2,
                },
            ) => *iterations1 == *iterations2 && *window_size1 == *window_size2,
        }
    }
}
