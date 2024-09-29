use crate::smoothing::MovingAverage;
use crate::smoothing::circular_buffer::CircularBuffer;
use num_traits::{FromPrimitive, Zero};
use std::ops::{AddAssign, SubAssign, Div, Mul};

pub struct SimpleMA<Type> {
    buffer: CircularBuffer<Type>,
    num: usize,
    div: Type,
    one: Type
}

impl<Type> MovingAverage<Type>
for
    SimpleMA<Type>
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

impl<Type> SimpleMA<Type>
where
    Type: Copy + Zero + FromPrimitive
{
    pub fn new(window_size: usize) -> Self {
        Self {
            buffer: CircularBuffer::new(window_size),
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
        let simple_ma: SimpleMA<i32> = SimpleMA::new(3);
        assert_eq!(simple_ma.compute_average(), 0);
    }

    #[test]
    fn add_value() {
        let mut simple_ma: SimpleMA<f32> = SimpleMA::new(3);
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
        let mut simple_ma: SimpleMA<f32> = SimpleMA::new(3);
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
        let mut simple_ma: SimpleMA<f32> = SimpleMA::new(3);
        simple_ma.add_value(1.0);
        simple_ma.add_value(2.0);
        simple_ma.add_value(3.0);
        assert_eq!(simple_ma.compute_average(), 6.0/3.0);
        simple_ma.clear();
        assert_eq!(simple_ma.compute_average(), 0.0);
    }
}
