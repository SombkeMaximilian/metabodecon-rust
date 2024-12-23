use crate::smoothing::circular_buffer::CircularBuffer;
use num_traits::{FromPrimitive, Zero};
use std::ops::{AddAssign, Div, Mul, SubAssign};

#[derive(Debug)]
pub struct MovingAverage<Type> {
    buffer: CircularBuffer<Type>,
    sum: Type,
    div: Type,
    one: Type,
}

impl<Type> MovingAverage<Type>
where
    Type: Copy
        + FromPrimitive
        + Zero
        + AddAssign
        + SubAssign
        + Div<Output = Type>
        + Mul<Output = Type>,
{
    pub fn new(window_size: usize) -> Self {
        Self {
            buffer: CircularBuffer::new(window_size),
            sum: Type::zero(),
            div: Type::from_u8(1).unwrap(),
            one: Type::from_u8(1).unwrap(),
        }
    }

    pub fn add_value(&mut self, value: Type) {
        self.sum += value;
        if let Some(popped_value) = self.buffer.next(value) {
            self.sum -= popped_value;
        } else {
            self.div = self.one / Type::from_usize(self.buffer.num_elements()).unwrap();
        }
    }

    pub fn pop_last(&mut self) -> Option<Type> {
        if let Some(popped_value) = self.buffer.pop() {
            self.div = self.one / Type::from_usize(self.buffer.num_elements()).unwrap();
            self.sum -= popped_value;
            Some(popped_value)
        } else {
            None
        }
    }

    pub fn compute_average(&self) -> Type {
        if self.buffer.num_elements() == 0 {
            return Type::zero();
        }
        self.sum * self.div
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
        self.sum = Type::from_u8(0).unwrap();
        self.div = self.one;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new() {
        let sum_cache_ma: MovingAverage<f32> = MovingAverage::new(3);
        assert_eq!(sum_cache_ma.compute_average(), 0.0);
    }

    #[test]
    fn add_value() {
        let mut sum_cache_ma: MovingAverage<f32> = MovingAverage::new(3);
        sum_cache_ma.add_value(1.0);
        assert_eq!(sum_cache_ma.compute_average(), 1.0 / 1.0);
        sum_cache_ma.add_value(2.0);
        assert_eq!(sum_cache_ma.compute_average(), 3.0 / 2.0);
        sum_cache_ma.add_value(3.0);
        assert_eq!(sum_cache_ma.compute_average(), 6.0 / 3.0);
        sum_cache_ma.add_value(4.0);
        assert_eq!(sum_cache_ma.compute_average(), 9.0 / 3.0);
        sum_cache_ma.add_value(5.0);
        assert_eq!(sum_cache_ma.compute_average(), 12.0 / 3.0);
    }

    #[test]
    fn pop_last() {
        let mut sum_cache_ma: MovingAverage<f32> = MovingAverage::new(3);
        sum_cache_ma.add_value(1.0);
        sum_cache_ma.add_value(2.0);
        sum_cache_ma.add_value(3.0);
        sum_cache_ma.add_value(4.0);
        sum_cache_ma.add_value(5.0);
        assert_eq!(sum_cache_ma.compute_average(), 12.0 / 3.0);
        assert_eq!(sum_cache_ma.pop_last(), Some(3.0));
        assert_eq!(sum_cache_ma.compute_average(), 9.0 / 2.0);
        assert_eq!(sum_cache_ma.pop_last(), Some(4.0));
        assert_eq!(sum_cache_ma.compute_average(), 5.0 / 1.0);
        assert_eq!(sum_cache_ma.pop_last(), Some(5.0));
        assert_eq!(sum_cache_ma.compute_average(), 0.0);
        assert_eq!(sum_cache_ma.pop_last(), None);
        assert_eq!(sum_cache_ma.compute_average(), 0.0);
        assert_eq!(sum_cache_ma.pop_last(), None);
        assert_eq!(sum_cache_ma.compute_average(), 0.0);
        assert_eq!(sum_cache_ma.pop_last(), None);
    }

    #[test]
    fn clear() {
        let mut sum_cache_ma: MovingAverage<f32> = MovingAverage::new(3);
        sum_cache_ma.add_value(1.0);
        sum_cache_ma.add_value(2.0);
        sum_cache_ma.add_value(3.0);
        assert_eq!(sum_cache_ma.compute_average(), 6.0 / 3.0);
        sum_cache_ma.clear();
        assert_eq!(sum_cache_ma.compute_average(), 0.0);
    }
}
