use crate::smoothing::Smoother;
use crate::smoothing::MovingAverage;
use num_traits::Zero;
use std::marker::PhantomData;

pub struct MovingAverageSmoother<Type, Algo, const WINDOW_SIZE: usize>
where
    Type: Copy + Zero,
    Algo: MovingAverage<Type, WINDOW_SIZE>
{
    algo: Algo,
    left: usize,
    right: usize,
    type_marker: PhantomData<Type>
}

impl<Type, Algo, const WINDOW_SIZE: usize> Smoother<Type, Algo, WINDOW_SIZE>
for
    MovingAverageSmoother<Type, Algo, WINDOW_SIZE>
where
    Type: Copy + Zero,
    Algo: MovingAverage<Type, WINDOW_SIZE>
{
    fn new(value: Type) -> Self {
        Self {
            algo: Algo::new(value),
            left: WINDOW_SIZE / 2,
            right: if WINDOW_SIZE % 2 == 1 {WINDOW_SIZE / 2} else {WINDOW_SIZE / 2 - 1},
            type_marker: PhantomData
        }
    }

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
