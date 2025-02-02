use crate::deconvolution::smoothing::{CircularBuffer, Smoother};
use num_traits::{FromPrimitive, Zero};
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
    /// The buffer used to store the values in the window.
    buffer: CircularBuffer<Type>,
    /// The cached sum of the values in the window.
    sum: Type,
    /// The cached division factor for computing the average.
    div: Type,
    /// The cached value of one for adjusting the division factor.
    one: Type,
    /// The number of iterations to apply the filter.
    iterations: usize,
    /// The number of values to the right of the current value in the window.
    right: usize,
}

impl<Type> Smoother<Type> for MovingAverage<Type>
where
    Type: Copy
        + FromPrimitive
        + Zero
        + AddAssign
        + SubAssign
        + Mul<Output = Type>
        + Div<Output = Type>
        + 'static,
{
    /// Smooths the given sequence of values in place using the moving average
    /// filter.
    fn smooth_values(&mut self, values: &mut [Type]) {
        let len = values.len();
        for _ in 0..self.iterations {
            values
                .iter()
                .take(self.right)
                .for_each(|value| self.add_value(*value));
            for i in 0..(len - self.right) {
                self.add_value(values[i + self.right]);
                values[i] = self.compute_average();
            }
            values[(len - self.right)..]
                .iter_mut()
                .for_each(|value| {
                    self.pop_last();
                    *value = self.compute_average();
                });
            self.clear();
        }
    }
}

impl<Type> MovingAverage<Type>
where
    Type: Copy
        + Zero
        + FromPrimitive
        + AddAssign
        + SubAssign
        + Mul<Output = Type>
        + Div<Output = Type>
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
            buffer: CircularBuffer::new(window_size),
            sum: Type::zero(),
            div: Type::from_u8(1).unwrap(),
            one: Type::from_u8(1).unwrap(),
            iterations,
            right: window_size / 2,
        }
    }

    /// Adds the given value to the buffer and updates the cached sum and
    /// division factor.
    fn add_value(&mut self, value: Type) {
        self.sum += value;
        if let Some(popped_value) = self.buffer.next(value) {
            self.sum -= popped_value;
        } else {
            self.div = self.one / Type::from_usize(self.buffer.num_elements()).unwrap();
        }
    }

    /// Removes the oldest value from the buffer, updates the cached sum and
    /// division factor, and returns the removed value (if any).
    fn pop_last(&mut self) -> Option<Type> {
        if let Some(popped_value) = self.buffer.pop() {
            self.div = self.one / Type::from_usize(self.buffer.num_elements()).unwrap();
            self.sum -= popped_value;
            Some(popped_value)
        } else {
            None
        }
    }

    /// Returns the average of the values in the buffer.
    fn compute_average(&self) -> Type {
        self.sum * self.div
    }

    /// Clears the buffer and resets the cached sum and division factor.
    fn clear(&mut self) {
        self.buffer.clear();
        self.sum = Type::zero();
        self.div = self.one;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::assert_approx_eq;

    #[test]
    fn new() {
        let smoother = MovingAverage::<f64>::new(1, 3);
        assert_approx_eq!(f64, smoother.compute_average(), 0.0);
    }

    #[test]
    fn add_value() {
        let mut smoother = MovingAverage::<f64>::new(1, 3);
        smoother.add_value(1.0);
        assert_approx_eq!(f64, smoother.compute_average(), 1.0 / 1.0);
        smoother.add_value(2.0);
        assert_approx_eq!(f64, smoother.compute_average(), 3.0 / 2.0);
        smoother.add_value(3.0);
        assert_approx_eq!(f64, smoother.compute_average(), 6.0 / 3.0);
        smoother.add_value(4.0);
        assert_approx_eq!(f64, smoother.compute_average(), 9.0 / 3.0);
        smoother.add_value(5.0);
        assert_approx_eq!(f64, smoother.compute_average(), 12.0 / 3.0);
        smoother.add_value(6.0);
        assert_approx_eq!(f64, smoother.compute_average(), 15.0 / 3.0);
    }

    #[test]
    fn pop_last() {
        let mut smoother = MovingAverage::<f64>::new(1, 3);
        for i in 1..6 {
            smoother.add_value(i as f64);
        }
        assert_approx_eq!(f64, smoother.compute_average(), 12.0 / 3.0);
        assert_approx_eq!(f64, smoother.pop_last().unwrap(), 3.0);
        assert_approx_eq!(f64, smoother.compute_average(), 9.0 / 2.0);
        assert_approx_eq!(f64, smoother.pop_last().unwrap(), 4.0);
        assert_approx_eq!(f64, smoother.compute_average(), 5.0 / 1.0);
        assert_approx_eq!(f64, smoother.pop_last().unwrap(), 5.0);
        assert!(smoother.compute_average().is_nan());
        assert!(smoother.pop_last().is_none());
        assert!(smoother.compute_average().is_nan());
        assert!(smoother.pop_last().is_none());
        assert!(smoother.compute_average().is_nan());
        assert!(smoother.pop_last().is_none());
        smoother.add_value(1.0);
        assert_approx_eq!(f64, smoother.compute_average(), 1.0 / 1.0);
    }

    #[test]
    fn clear() {
        let mut smoother = MovingAverage::<f64>::new(1, 3);
        for i in 1..4 {
            smoother.add_value(i as f64);
        }
        assert_approx_eq!(f64, smoother.compute_average(), 6.0 / 3.0);
        smoother.clear();
        assert_approx_eq!(f64, smoother.compute_average(), 0.0);
    }
}
