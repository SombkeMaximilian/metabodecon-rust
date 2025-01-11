//! The Metabodecon deconvolution algorithm.

mod deconvoluter;
mod deconvolution;
mod error;

pub use deconvoluter::Deconvoluter;
pub use deconvolution::Deconvolution;

pub use crate::fitting::FittingAlgo;
pub use crate::peak_selection::{ScoringAlgo, SelectionAlgo};
pub use crate::smoothing::SmoothingAlgo;

pub use crate::fitting::Lorentzian;

pub use error::{Error, Kind};
