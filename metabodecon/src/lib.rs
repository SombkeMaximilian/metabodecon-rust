mod data_structures;
mod deconvoluter;
mod fitting;
mod peak_selection;
mod smoothing;

pub use data_structures::{Deconvolution, Lorentzian, Spectrum};
pub use deconvoluter::Deconvoluter;
pub use fitting::{Fitter, FitterAnalytical, FittingAlgo};
pub use peak_selection::{ScoringAlgo, SelectionAlgo, Selector, SelectorDefault};
pub use smoothing::{MovingAverageAlgo, SmoothingAlgo};
