mod deconvolution;
mod fitting;
mod peak_selection;
mod smoothing;
mod spectrum;

pub use deconvolution::{Deconvoluter, Deconvolution};
pub use fitting::{Fitter, FitterAnalytical, FittingAlgo, Lorentzian};
pub use peak_selection::{ScoringAlgo, SelectionAlgo, Selector, SelectorDefault};
pub use smoothing::{MovingAverageAlgo, SmoothingAlgo};
pub use spectrum::{BrukerReader, Hdf5Reader, JdxReader, Spectrum};
