//! The Metabodecon deconvolution algorithm.

mod deconvoluter;
pub use deconvoluter::Deconvoluter;

mod deconvolution;
pub use deconvolution::Deconvolution;

mod lorentzian;
pub use lorentzian::Lorentzian;

#[cfg(feature = "serde")]
mod serialized_representations;
#[cfg(feature = "serde")]
pub(crate) use serialized_representations::{SerializedDeconvolution, SerializedLorentzian};

mod fitting;
pub use fitting::FittingSettings;

mod peak_selection;
pub use peak_selection::{ScoringMethod, SelectionSettings};

mod smoothing;
pub use smoothing::SmoothingSettings;

pub mod error;
