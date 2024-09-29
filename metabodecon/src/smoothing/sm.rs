use crate::smoothing::sm_ma::MovingAverageAlgo;

pub enum SmoothingAlgo {
    MovingAverage { algo: MovingAverageAlgo, iterations: usize, window_size: usize }
}

pub trait Smoother<Type> {
    fn compute_smoothed(&mut self, values: &[Type]) -> Vec<Type>;
}
