//! The Metabodecon alignment algorithm.

mod aligner;
pub use aligner::Aligner;

mod alignment;
pub use alignment::Alignment;

mod assignment;
pub use assignment::{FilteringSettings, SimilarityMetric};

mod feature;

mod solving;
mod error;

pub use solving::SolvingSettings;
