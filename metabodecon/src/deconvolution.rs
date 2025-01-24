//! The Metabodecon deconvolution algorithm.

mod deconvoluter;
pub use deconvoluter::Deconvoluter;

mod deconvolution;
pub use deconvolution::Deconvolution;

mod fitting;
pub use fitting::FittingAlgo;
pub use fitting::Lorentzian;

mod peak_selection;
pub use peak_selection::{ScoringAlgo, SelectionAlgo};

mod smoothing;
pub use smoothing::SmoothingAlgo;

pub mod error;

mod settings;
pub(crate) use settings::Settings;
