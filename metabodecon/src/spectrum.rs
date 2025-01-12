//! The Metabodecon 1D NMR [`Spectrum`] data structure and related utilities.

mod bruker_reader;
mod hdf5_reader;
mod macros;
#[rustfmt::skip] #[allow(dead_code)] mod jdx_reader;
mod spectrum;

pub use spectrum::{Monotonicity, Spectrum};

pub use bruker_reader::BrukerReader;
pub use hdf5_reader::Hdf5Reader;
pub use jdx_reader::JdxReader;

pub mod error;
