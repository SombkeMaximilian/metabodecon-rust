mod spectrum;
mod deconvolution;
mod fitting;
mod peak_selection;
mod smoothing;

pub use spectrum::Spectrum;
pub use deconvolution::{Deconvoluter, Deconvolution};
pub use fitting::{Fitter, FitterAnalytical, FittingAlgo, Lorentzian};
pub use peak_selection::{ScoringAlgo, SelectionAlgo, Selector, SelectorDefault};
pub use smoothing::{MovingAverageAlgo, SmoothingAlgo};
