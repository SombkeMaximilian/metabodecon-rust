mod data_structures;
mod deconvoluter;
mod fitting;
mod peak_selection;
mod preprocessing;
mod smoothing;

pub use data_structures::{Deconvolution, Lorentzian, Spectrum};
pub use deconvoluter::Deconvoluter;
pub use fitting::{Fitter, FitterAnalytical, FittingAlgo};
pub use smoothing::{MovingAverageAlgo, SmoothingAlgo};
