use crate::{deconvolution, spectrum};

/// A type alias for `Result<T, metabodecon::Error>`.
pub type Result<T> = std::result::Result<T, Error>;

/// The error type for the crate.
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// An error occurred during spectrum processing.
    Spectrum(spectrum::error::Error),
    /// An error occurred during deconvolution.
    Deconvolution(deconvolution::error::Error),
    /// An I/O error occurred.
    IoError(std::io::Error),
    /// An error occurred during HDF5 operation.
    Hdf5Error(hdf5::Error),
}

impl std::error::Error for Error {}

impl From<spectrum::error::Error> for Error {
    fn from(error: spectrum::error::Error) -> Self {
        Error::Spectrum(error)
    }
}

impl From<deconvolution::error::Error> for Error {
    fn from(error: deconvolution::error::Error) -> Self {
        Error::Deconvolution(error)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IoError(error)
    }
}

impl From<hdf5::Error> for Error {
    fn from(error: hdf5::Error) -> Self {
        Error::Hdf5Error(error)
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        use Error::*;
        match *self {
            Spectrum(ref e) => e.fmt(f),
            Deconvolution(ref e) => e.fmt(f),
            IoError(ref e) => e.fmt(f),
            Hdf5Error(ref e) => e.fmt(f),
        }
    }
}
