use crate::smoothing::moving_average_simple::SimpleMA;
use crate::smoothing::moving_average_sum_cache::SumCacheMA;
use crate::smoothing::MovingAverage;
use crate::smoothing::Smoother;
use num_traits::{FromPrimitive, Zero};
use std::marker::PhantomData;
use std::ops::{AddAssign, Div, Mul, SubAssign};

#[derive(Clone, Copy, Debug)]
pub enum MovingAverageAlgo {
    Simple,
    SumCache,
}

pub struct MovingAverageSmoother<Type> {
    algo: Box<dyn MovingAverage<Type>>,
    iterations: usize,
    right: usize,
    type_marker: PhantomData<Type>,
}

impl<Type: Copy + Zero> Smoother<Type> for MovingAverageSmoother<Type> {
    fn smooth_values(&mut self, values: &mut [Type]) {
        for _ in 0..self.iterations {
            for i in 0..self.right {
                self.algo.add_value(values[i]);
            }
            for i in 0..(values.len() - self.right) {
                self.algo.add_value(values[i + self.right]);
                values[i] = self.algo.compute_average();
            }
            for i in (values.len() - self.right)..values.len() {
                self.algo.pop_last();
                values[i] = self.algo.compute_average();
            }
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
    pub fn new(algo: MovingAverageAlgo, iterations: usize, window_size: usize) -> Self {
        let algo: Box<dyn MovingAverage<Type>> = match algo {
            MovingAverageAlgo::Simple => Box::new(SimpleMA::new(window_size)),
            MovingAverageAlgo::SumCache => Box::new(SumCacheMA::new(window_size)),
        };
        Self {
            algo,
            iterations,
            right: window_size / 2,
            type_marker: PhantomData,
        }
    }
}
