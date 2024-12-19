use crate::{deconvolution, spectrum};

pub type Result<T> = std::result::Result<T, Error>;

#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    Spectrum(spectrum::Error),
    Deconvolution(deconvolution::Error),
    IoError(std::io::Error),
    Hdf5Error(hdf5::Error),
}

impl std::error::Error for Error {}

impl From<spectrum::Error> for Error {
    fn from(error: spectrum::Error) -> Self {
        Error::Spectrum(error)
    }
}

impl From<deconvolution::Error> for Error {
    fn from(error: deconvolution::Error) -> Self {
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
