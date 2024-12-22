use crate::smoothing::moving_average_sum_cache::SumCacheMA;
use crate::smoothing::Smoother;
use num_traits::{FromPrimitive, Zero};
use std::marker::PhantomData;
use std::ops::{AddAssign, Div, Mul, SubAssign};

pub struct MovingAverageSmoother<Type> {
    algo: SumCacheMA<Type>,
    iterations: usize,
    right: usize,
    type_marker: PhantomData<Type>,
}

impl<Type> Smoother<Type> for MovingAverageSmoother<Type>
where
    Type: Copy
    + FromPrimitive
    + Zero
    + AddAssign
    + SubAssign
    + Div<Output = Type>
    + Mul<Output = Type>,
{
    fn smooth_values(&mut self, values: &mut [Type]) {
        let len = values.len();
        for _ in 0..self.iterations {
            values
                .iter()
                .take(self.right)
                .for_each(|value| self.algo.add_value(*value));
            for i in 0..(len - self.right) {
                self.algo.add_value(values[i + self.right]);
                values[i] = self.algo.compute_average();
            }
            values[(len - self.right)..]
                .iter_mut()
                .for_each(|value| {
                    self.algo.pop_last();
                    *value = self.algo.compute_average();
                });
            self.algo.clear();
        }
    }
}

impl<Type> MovingAverageSmoother<Type>
where
    Type: Copy
        + Zero
        + FromPrimitive
        + 'static
        + AddAssign
        + SubAssign
        + Div<Output = Type>
        + Mul<Output = Type>,
{
    pub fn new(iterations: usize, window_size: usize) -> Self {
        Self {
            algo: SumCacheMA::new(window_size),
            iterations,
            right: window_size / 2,
            type_marker: PhantomData,
        }
    }
}
