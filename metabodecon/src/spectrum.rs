#[macro_use]
mod macros;
mod bruker_reader;
mod hdf5_reader;
#[rustfmt::skip] #[allow(dead_code)] mod jdx_reader;
mod error;
mod spectrum;

pub use spectrum::{Monotonicity, Spectrum};

pub use bruker_reader::BrukerReader;
pub use hdf5_reader::Hdf5Reader;
pub use jdx_reader::JdxReader;

pub use error::{Error, Kind};
