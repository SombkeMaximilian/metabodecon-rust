//! The Metabodecon alignment algorithm.

mod aligner;
pub use aligner::Aligner;

mod alignment;
pub use alignment::Alignment;

mod solving;

mod feature;
mod assignment;
