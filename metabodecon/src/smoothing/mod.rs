mod circular_buffer;
mod ma;
mod ma_simple;
mod ma_sum_cache;
mod sm;
mod sm_ma;

pub use ma::MovingAverage;
pub use sm::Smoother;
pub use sm_ma::MovingAverageAlgo;
pub use sm_ma::MovingAverageSmoother;
pub use ma_simple::SimpleMA;
pub use ma_sum_cache::SumCacheMA;
pub use circular_buffer::CircularBuffer;
