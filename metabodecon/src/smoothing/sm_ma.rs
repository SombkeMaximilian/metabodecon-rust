use crate::smoothing::Smoother;
use crate::smoothing::MovingAverage;
use crate::smoothing::ma_sum_cache::SumCacheMA;
use num_traits::{FromPrimitive, Zero};
use std::ops::{AddAssign, SubAssign, Div, Mul};
use std::marker::PhantomData;

pub enum MovingAverageAlgo {
    SumCache
}

pub struct MovingAverageSmoother<Type, const WINDOW_SIZE: usize>
{
    algo: Box<dyn MovingAverage<Type, WINDOW_SIZE>>,
    right: usize,
    type_marker: PhantomData<Type>
}

impl<Type: Copy + Zero, const WINDOW_SIZE: usize> Smoother<Type, WINDOW_SIZE>
for
    MovingAverageSmoother<Type, WINDOW_SIZE>
{
    fn compute_smoothed(&mut self, values: Vec<Type>) -> Vec<Type> {
        let mut smoothed_values : Vec<Type> = vec![Type::zero(); values.len()];

        for i in 0..self.right {
            self.algo.add_value(values[i]);
        }
        for i in 0..(values.len() - self.right) {
            self.algo.add_value(values[i + self.right]);
            smoothed_values[i] = self.algo.compute_average();
        }
        for i in (values.len() - self.right)..values.len() {
            self.algo.pop_last();
            smoothed_values[i] = self.algo.compute_average();
        }
        self.algo.clear();

        smoothed_values
    }
}

impl<Type, const WINDOW_SIZE: usize> MovingAverageSmoother<Type, WINDOW_SIZE>
where
    Type: Copy + Zero + FromPrimitive + 'static +
          AddAssign + SubAssign + Div<Output = Type> + Mul<Output = Type>
{
    pub fn new(algo: MovingAverageAlgo) -> Self {
        let algo : Box<dyn MovingAverage<Type, WINDOW_SIZE>> = match algo {
            MovingAverageAlgo::SumCache => Box::new(SumCacheMA::new())
        };
        Self {
            algo,
            right: WINDOW_SIZE / 2,
            type_marker: PhantomData
        }
    }
}
