use crate::smoothing::circular_buffer::CircularBuffer;
use num_traits::FromPrimitive;
use std::ops::{AddAssign, SubAssign, Div, Mul};

pub struct SumCacheMA<Type, const WINDOW_SIZE: usize> {
    buffer: CircularBuffer<Type, WINDOW_SIZE>,
    sum: Type,
    div: Type
}

impl<Type, const WINDOW_SIZE: usize> SumCacheMA<Type, WINDOW_SIZE>
where Type: Copy + AddAssign + SubAssign + Div<Output = Type> + Mul<Output = Type> + FromPrimitive {
    pub fn new(value: Type) -> Self {
        Self {
            buffer: CircularBuffer::new(value),
            sum: value,
            div: Type::from_u8(1).unwrap() / Type::from_usize(WINDOW_SIZE).unwrap()
        }
    }

    pub fn add_value(&mut self, value: Type) -> Type {
        self.sum += value;
        if let Some(popped_value) = self.buffer.next(value) {
            self.sum -= popped_value;
        }
    }

    pub fn compute_average(&self) -> Type {
        self.sum * self.div
    }
}
