use crate::deconvolution::smoothing::{CircularBuffer, Smoother, SmoothingSettings};
use num_traits::{FromPrimitive, One, Zero};
use std::marker::PhantomData;
use std::ops::{AddAssign, Div, Mul, SubAssign};

/// Moving average filter that smooths a sequence of values by averaging them
/// over a sliding window.
///
/// # Edge Handling
///
/// The window is centered around the current value. To handle the edges of the
/// input sequence, the window starts at half its size (rounded up) at the left
/// edge and grows to its full size when there are enough values to the left of
/// the current value. For example, with a window size of 5 (window center
/// marked by `x`, extent marked by `-`):
///
/// | Index  | 0   | 1   | 2   | 3   | 4   | 5   | 6   |
/// | ------ | --- | --- | --- | --- | --- | --- | --- |
/// | Step 1 | x   | -   | -   |     |     |     |     |
/// | Step 2 | -   | x   | -   | -   |     |     |     |
/// | Step 3 | -   | -   | x   | -   | -   |     |     |
/// | Step 4 |     | -   | -   | x   | -   | -   |     |
/// | Step 5 |     |     | -   | -   | x   | -   | -   |
/// | Step 6 |     |     |     | -   | -   | x   | -   |
/// | Step 7 |     |     |     |     | -   | -   | x   |
#[derive(Debug)]
pub(crate) struct MovingAverage<Type> {
    /// The number of iterations to apply the filter.
    iterations: usize,
    /// The size of the sliding window.
    window_size: usize,
    /// The number of values to the right of the current value in the window.
    right: usize,
    /// Marker for the type of the values.
    _marker: PhantomData<Type>,
}

impl<Type> Smoother<Type> for MovingAverage<Type>
where
    Type: Copy
        + FromPrimitive
        + One
        + Zero
        + AddAssign
        + SubAssign
        + Mul<Output = Type>
        + Div<Output = Type>
        + Send
        + Sync
        + std::fmt::Debug
        + 'static,
{
    /// Smooths the given sequence of values in place using the moving average
    /// filter.
    fn smooth_values(&self, values: &mut [Type]) {
        let mut cache = CircularBuffer::<Type>::new(self.window_size);
        let values_len = values.len();
        for _ in 0..self.iterations {
            let mut div = Type::one();
            let mut sum = Type::zero();
            values.iter().take(self.right).for_each(|value| {
                cache.push(*value);
                sum += *value;
            });
            for i in 0..(values_len - self.right) {
                sum += values[i + self.right];
                if let Some(popped) = cache.push(values[i + self.right]) {
                    sum -= popped;
                } else {
                    div = Type::one() / Type::from_usize(cache.len()).unwrap();
                };
                values[i] = sum * div;
            }
            values[(values_len - self.right)..]
                .iter_mut()
                .for_each(|value| {
                    if let Some(popped) = cache.pop() {
                        sum -= popped;
                        div = Type::one() / Type::from_usize(cache.len()).unwrap();
                        *value = sum * div;
                    }
                });
            cache.clear();
        }
    }

    fn settings(&self) -> SmoothingSettings {
        SmoothingSettings::MovingAverage {
            iterations: self.iterations,
            window_size: self.window_size,
        }
    }
}

impl<Type> MovingAverage<Type>
where
    Type: Copy
        + FromPrimitive
        + One
        + Zero
        + AddAssign
        + SubAssign
        + Mul<Output = Type>
        + Div<Output = Type>
        + Send
        + Sync
        + std::fmt::Debug
        + 'static,
{
    /// Creates a new `MovingAverage` filter with the given number of iterations
    /// and window size.
    ///
    /// # Panics
    ///
    /// Panics if the window size is zero.
    pub(crate) fn new(iterations: usize, window_size: usize) -> Self {
        Self {
            iterations,
            window_size,
            right: window_size / 2,
            _marker: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_send, assert_sync};

    #[test]
    fn thread_safety() {
        assert_send!(MovingAverage<f64>);
        assert_sync!(MovingAverage<f64>);
    }
}
