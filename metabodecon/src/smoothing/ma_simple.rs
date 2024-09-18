use crate::smoothing::MovingAverage;
use crate::smoothing::circular_buffer::CircularBuffer;
use num_traits::{FromPrimitive, Zero};
use std::ops::{AddAssign, SubAssign, Div, Mul};

pub struct SimpleMA<Type, const WINDOW_SIZE: usize> {
    buffer: CircularBuffer<Type, WINDOW_SIZE>,
    num: usize,
    div: Type,
    one: Type
}

impl<Type, const WINDOW_SIZE: usize> MovingAverage<Type, WINDOW_SIZE>
for
    SimpleMA<Type, WINDOW_SIZE>
where
    Type: Copy + FromPrimitive + Zero +
          AddAssign + SubAssign + Div<Output = Type> + Mul<Output = Type>
{
    fn add_value(&mut self, value: Type) {
        self.buffer.next(value);
        if self.num != self.buffer.num_elements() {
            self.num += 1;
            self.div = self.one / Type::from_usize(self.num).unwrap();
        }
    }

    fn pop_last(&mut self) -> Option<Type> {
        if let Some(popped_value) = self.buffer.pop() {
            self.num -= 1;
            self.div = self.one / Type::from_usize(self.num).unwrap();
            Some(popped_value)
        } else {
            None
        }
    }

    fn compute_average(&self) -> Type {
        if self.buffer.num_elements() == 0 {
            return Type::zero();
        }
        let mut sum : Type = Type::zero();
        for value in self.buffer.iter() {
            sum += *value;
        }
        sum * self.div
    }

    fn clear(&mut self) {
        self.buffer.clear();
        self.div = self.one;
        self.num = 0;
    }
}

impl<Type, const WINDOW_SIZE: usize> SimpleMA<Type, WINDOW_SIZE>
where
    Type: Copy + Zero + FromPrimitive + Div<Output = Type>
{
    pub fn new() -> Self {
        Self {
            buffer: CircularBuffer::new(Type::zero()),
            num: 0,
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
        let simple_ma: SimpleMA<i32, 3> = SimpleMA::new();
        assert_eq!(simple_ma.compute_average(), 0);
    }

    #[test]
    fn add_value() {
        let mut simple_ma: SimpleMA<f32, 3> = SimpleMA::new();
        simple_ma.add_value(1.0);
        assert_eq!(simple_ma.compute_average(), 1.0/1.0);
        simple_ma.add_value(2.0);
        assert_eq!(simple_ma.compute_average(), 3.0/2.0);
        simple_ma.add_value(3.0);
        assert_eq!(simple_ma.compute_average(), 6.0/3.0);
        simple_ma.add_value(4.0);
        assert_eq!(simple_ma.compute_average(), 9.0/3.0);
        simple_ma.add_value(5.0);
        assert_eq!(simple_ma.compute_average(), 12.0/3.0);
    }

    #[test]
    fn pop_last() {
        let mut simple_ma: SimpleMA<f32, 3> = SimpleMA::new();
        simple_ma.add_value(1.0);
        simple_ma.add_value(2.0);
        simple_ma.add_value(3.0);
        simple_ma.add_value(4.0);
        simple_ma.add_value(5.0);
        assert_eq!(simple_ma.compute_average(), 12.0/3.0);
        assert_eq!(simple_ma.pop_last(), Some(3.0));
        assert_eq!(simple_ma.compute_average(), 9.0/2.0);
        assert_eq!(simple_ma.pop_last(), Some(4.0));
        assert_eq!(simple_ma.compute_average(), 5.0/1.0);
        assert_eq!(simple_ma.pop_last(), Some(5.0));
        assert_eq!(simple_ma.compute_average(), 0.0);
        assert_eq!(simple_ma.pop_last(), None);
        assert_eq!(simple_ma.compute_average(), 0.0);
        assert_eq!(simple_ma.pop_last(), None);
        assert_eq!(simple_ma.compute_average(), 0.0);
        assert_eq!(simple_ma.pop_last(), None);
    }

    #[test]
    fn clear() {
        let mut simple_ma: SimpleMA<f32, 3> = SimpleMA::new();
        simple_ma.add_value(1.0);
        simple_ma.add_value(2.0);
        simple_ma.add_value(3.0);
        assert_eq!(simple_ma.compute_average(), 6.0/3.0);
        simple_ma.clear();
        assert_eq!(simple_ma.compute_average(), 0.0);
    }
}
