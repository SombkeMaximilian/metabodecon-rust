use crate::smoothing::circular_buffer::CircularBuffer;
use num_traits::FromPrimitive;
use std::ops::{AddAssign, SubAssign, Div, Mul};

pub struct SumCacheMA<Type, const WINDOW_SIZE: usize> {
    buffer: CircularBuffer<Type, WINDOW_SIZE>,
    sum: Type,
    div: Type
}

impl<Type, const WINDOW_SIZE: usize> SumCacheMA<Type, WINDOW_SIZE>
where Type: Copy + FromPrimitive + AddAssign + SubAssign + Div<Output = Type> + Mul<Output = Type> {
    pub fn new(value: Type) -> Self {
        Self {
            buffer: CircularBuffer::new(value),
            sum: value,
            div: Type::from_u8(1).unwrap() / Type::from_usize(WINDOW_SIZE).unwrap()
        }
    }

    pub fn add_value(&mut self, value: Type) {
        self.sum += value;
        if let Some(popped_value) = self.buffer.next(value) {
            self.sum -= popped_value;
        }
    }

    pub fn compute_average(&self) -> Type {
        self.sum * self.div
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new() {
        let sum_cache_ma : SumCacheMA<f32, 3> = SumCacheMA::new(0.0);
        assert_eq!(sum_cache_ma.compute_average(), 0.0);
    }

    #[test]
    fn add_value() {
        let mut sum_cache_ma : SumCacheMA<f32, 3> = SumCacheMA::new(0.0);
        sum_cache_ma.add_value(1.0);
        assert_eq!(sum_cache_ma.compute_average(), 1.0/3.0);
        sum_cache_ma.add_value(2.0);
        assert_eq!(sum_cache_ma.compute_average(), 3.0/3.0);
        sum_cache_ma.add_value(3.0);
        assert_eq!(sum_cache_ma.compute_average(), 6.0/3.0);
        sum_cache_ma.add_value(4.0);
        assert_eq!(sum_cache_ma.compute_average(), 9.0/3.0);
        sum_cache_ma.add_value(5.0);
        assert_eq!(sum_cache_ma.compute_average(), 12.0/3.0);
    }
}
