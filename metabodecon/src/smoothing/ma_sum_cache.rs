use crate::smoothing::MovingAverage;
use crate::smoothing::circular_buffer::CircularBuffer;
use num_traits::{FromPrimitive, Zero};
use std::ops::{AddAssign, SubAssign, Div, Mul};

pub struct SumCacheMA<Type, const WINDOW_SIZE: usize> {
    buffer: CircularBuffer<Type, WINDOW_SIZE>,
    num: usize,
    sum: Type,
    div: Type,
    one: Type
}

impl<Type, const WINDOW_SIZE: usize> MovingAverage<Type, WINDOW_SIZE>
for
    SumCacheMA<Type, WINDOW_SIZE>
where
    Type: Copy + FromPrimitive + AddAssign + SubAssign + Div<Output = Type> + Mul<Output = Type>
{
    fn add_value(&mut self, value: Type) {
        self.sum += value;
        if let Some(popped_value) = self.buffer.next(value) {
            self.sum -= popped_value;
        } else {
            self.num += 1;
            self.div = self.one / Type::from_usize(self.num).unwrap();
        }
    }

    fn pop_last(&mut self) -> Option<Type> {
        if let Some(popped_value) = self.buffer.pop() {
            self.num -= 1;
            self.div = self.one / Type::from_usize(self.num).unwrap();
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
        self.sum = Type::from_u8(0).unwrap();
    }
}

impl<Type, const WINDOW_SIZE: usize> SumCacheMA<Type, WINDOW_SIZE>
where
    Type: Copy + Zero + FromPrimitive + Div<Output = Type>
{
    pub fn new() -> Self {
        Self {
            buffer: CircularBuffer::new(Type::zero()),
            num: 0,
            sum: Type::zero(),
            div: Type::from_u8(1).unwrap(),
            one: Type::from_u8(1).unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new() {
        let sum_cache_ma : SumCacheMA<f32, 3> = SumCacheMA::new();
        assert_eq!(sum_cache_ma.compute_average(), 0.0);
    }

    #[test]
    fn add_value() {
        let mut sum_cache_ma : SumCacheMA<f32, 3> = SumCacheMA::new();
        sum_cache_ma.add_value(1.0);
        assert_eq!(sum_cache_ma.compute_average(), 1.0/1.0);
        sum_cache_ma.add_value(2.0);
        assert_eq!(sum_cache_ma.compute_average(), 3.0/2.0);
        sum_cache_ma.add_value(3.0);
        assert_eq!(sum_cache_ma.compute_average(), 6.0/3.0);
        sum_cache_ma.add_value(4.0);
        assert_eq!(sum_cache_ma.compute_average(), 9.0/3.0);
        sum_cache_ma.add_value(5.0);
        assert_eq!(sum_cache_ma.compute_average(), 12.0/3.0);
    }
}
