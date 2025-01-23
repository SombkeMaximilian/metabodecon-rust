//! The Metabodecon deconvolution algorithm.

mod deconvoluter;
mod deconvolution;

pub use deconvoluter::Deconvoluter;
pub use deconvolution::Deconvolution;

mod fitting;
mod peak_selection;
mod smoothing;

pub use fitting::FittingAlgo;
pub use peak_selection::{ScoringAlgo, SelectionAlgo};
pub use smoothing::SmoothingAlgo;

pub use fitting::Lorentzian;

pub mod error;

mod settings;

pub(crate) use settings::Settings;
