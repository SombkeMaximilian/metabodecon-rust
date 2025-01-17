use crate::deconvolution::Settings;
use crate::deconvolution::error::{Error, Kind};
use crate::error::Result;

/// Trait interface for smoothing algorithms.
pub trait Smoother<Type> {
    /// Smooths the given sequence of values in place.
    fn smooth_values(&mut self, values: &mut [Type]);
}

/// Smoothing methods for the signal intensities.
#[non_exhaustive]
#[derive(Copy, Clone, Debug)]
pub enum SmoothingAlgo {
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

impl Default for SmoothingAlgo {
    fn default() -> Self {
        SmoothingAlgo::MovingAverage {
            iterations: 2,
            window_size: 5,
        }
    }
}

impl Settings for SmoothingAlgo {
    fn validate(&self) -> Result<()> {
        match self {
            SmoothingAlgo::MovingAverage {
                iterations,
                window_size,
            } => {
                if *iterations == 0 || *window_size == 0 {
                    return Err(Error::new(Kind::InvalidSmoothingSettings { algo: *self }).into());
                }
            }
        }

        Ok(())
    }
}
