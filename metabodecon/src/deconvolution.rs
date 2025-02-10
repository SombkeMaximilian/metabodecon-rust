//! The Metabodecon deconvolution algorithm.

mod deconvoluter;
pub use deconvoluter::Deconvoluter;

mod deconvolution;
pub use deconvolution::Deconvolution;

mod lorentzian;
pub use lorentzian::Lorentzian;

mod fitting;
pub use fitting::FittingSettings;

mod peak_selection;
pub use peak_selection::{ScoringMethod, SelectionSettings};

mod smoothing;
pub use smoothing::SmoothingSettings;

pub mod error;

mod settings;
pub(crate) use settings::Settings;
