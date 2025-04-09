//! The Metabodecon alignment algorithm.

mod aligner;
pub use aligner::Aligner;

mod alignment;
pub use alignment::Alignment;

mod assignment;
pub use assignment::{FilteringSettings, SimilarityMetric};

mod feature;
pub use feature::AlignmentStrategy;

mod solving;
pub use solving::SolvingSettings;

pub mod error;
