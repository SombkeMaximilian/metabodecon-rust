use crate::smoothing::circular_buffer::CircularBuffer;
use crate::smoothing::Smoother;
use num_traits::{FromPrimitive, Zero};
use std::ops::{AddAssign, Div, Mul, SubAssign};

#[derive(Debug)]
pub struct MovingAverage<Type> {
    buffer: CircularBuffer<Type>,
    sum: Type,
    div: Type,
    one: Type,
    iterations: usize,
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
        + 'static
        + AddAssign
        + SubAssign
        + Mul<Output = Type>
        + Div<Output = Type>,
{
    pub fn new(iterations: usize, window_size: usize) -> Self {
        Self {
            buffer: CircularBuffer::new(window_size),
            sum: Type::zero(),
            div: Type::from_u8(1).unwrap(),
            one: Type::from_u8(1).unwrap(),
            iterations,
            right: window_size / 2,
        }
    }

    fn add_value(&mut self, value: Type) {
        self.sum += value;
        if let Some(popped_value) = self.buffer.next(value) {
            self.sum -= popped_value;
        } else {
            self.div = self.one / Type::from_usize(self.buffer.num_elements()).unwrap();
        }
    }

    fn pop_last(&mut self) -> Option<Type> {
        if let Some(popped_value) = self.buffer.pop() {
            self.div = self.one / Type::from_usize(self.buffer.num_elements()).unwrap();
            self.sum -= popped_value;
            Some(popped_value)
        } else {
            None
        }
    }

    fn compute_average(&self) -> Type {
        self.sum * self.div
    }

    fn clear(&mut self) {
        self.buffer.clear();
        self.sum = Type::zero();
        self.div = self.one;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn new() {
        let smoother = MovingAverage::<f64>::new(1, 3);
        assert_approx_eq!(smoother.compute_average(), 0.0);
    }

    #[test]
    fn add_value() {
        let mut smoother = MovingAverage::<f64>::new(1, 3);
        smoother.add_value(1.0);
        assert_approx_eq!(smoother.compute_average(), 1.0 / 1.0);
        smoother.add_value(2.0);
        assert_approx_eq!(smoother.compute_average(), 3.0 / 2.0);
        smoother.add_value(3.0);
        assert_approx_eq!(smoother.compute_average(), 6.0 / 3.0);
        smoother.add_value(4.0);
        assert_approx_eq!(smoother.compute_average(), 9.0 / 3.0);
        smoother.add_value(5.0);
        assert_approx_eq!(smoother.compute_average(), 12.0 / 3.0);
        smoother.add_value(6.0);
        assert_approx_eq!(smoother.compute_average(), 15.0 / 3.0);
    }

    #[test]
    fn pop_last() {
        let mut smoother = MovingAverage::<f64>::new(1, 3);
        for i in 1..6 {
            smoother.add_value(i as f64);
        }
        assert_approx_eq!(smoother.compute_average(), 12.0 / 3.0);
        assert_approx_eq!(smoother.pop_last().unwrap(), 3.0);
        assert_approx_eq!(smoother.compute_average(), 9.0 / 2.0);
        assert_approx_eq!(smoother.pop_last().unwrap(), 4.0);
        assert_approx_eq!(smoother.compute_average(), 5.0 / 1.0);
        assert_approx_eq!(smoother.pop_last().unwrap(), 5.0);
        assert!(smoother.compute_average().is_nan());
        assert!(smoother.pop_last().is_none());
        assert!(smoother.compute_average().is_nan());
        assert!(smoother.pop_last().is_none());
        assert!(smoother.compute_average().is_nan());
        assert!(smoother.pop_last().is_none());
        smoother.add_value(1.0);
        assert_approx_eq!(smoother.compute_average(), 1.0 / 1.0);
    }

    #[test]
    fn clear() {
        let mut smoother = MovingAverage::<f64>::new(1, 3);
        for i in 1..4 {
            smoother.add_value(i as f64);
        }
        assert_approx_eq!(smoother.compute_average(), 6.0 / 3.0);
        smoother.clear();
        assert_approx_eq!(smoother.compute_average(), 0.0);
    }
}
