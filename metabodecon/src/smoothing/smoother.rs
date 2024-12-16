use crate::smoothing::smoother_moving_average::MovingAverageAlgo;

#[non_exhaustive]
#[derive(Copy, Clone, Debug)]
pub enum SmoothingAlgo {
    MovingAverage {
        algo: MovingAverageAlgo,
        iterations: usize,
        window_size: usize,
    },
}

pub trait Smoother<Type> {
    fn smooth_values(&mut self, values: &mut [Type]);
}
