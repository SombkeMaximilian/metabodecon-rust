use crate::{deconvolution, spectrum};

#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum Error {
    Spectrum(spectrum::Error),
    Deconvolution(deconvolution::Error),
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

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match *self {
            Error::Spectrum(ref e) => e.fmt(f),
            Error::Deconvolution(ref e) => e.fmt(f),
        }
    }
}
