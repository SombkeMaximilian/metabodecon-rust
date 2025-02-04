use crate::{deconvolution, spectrum};

/// A specialized [`Result`] type for the Metabodecon library.
///
/// This type alias is used to avoid writing out `Result<T, metabodecon::Error>`
/// directly, and is broadly used across the library.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors encountered by the Metabodecon library.
///
/// Marked as non-exhaustive because some variants will only be available with
/// certain features enabled.
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// An error that occurred during [`Spectrum`] construction or parsing.
    ///
    /// [`Spectrum`]: spectrum::Spectrum
    Spectrum(spectrum::error::Error),
    /// An error that occurred during the [`deconvolution`] process.
    Deconvolution(deconvolution::error::Error),
    /// Wrapper for errors from [`std::io`].
    #[cfg(any(feature = "bruker", feature = "jdx"))]
    IoError(std::io::Error),
    /// Wrapper for errors from the [hdf5 crate].
    ///
    /// [hdf5 crate]: https://docs.rs/crate/hdf5/latest
    #[cfg(feature = "hdf5")]
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

#[cfg(any(feature = "bruker", feature = "jdx"))]
impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IoError(error)
    }
}

#[cfg(feature = "hdf5")]
impl From<hdf5::Error> for Error {
    fn from(error: hdf5::Error) -> Self {
        Error::Hdf5Error(error)
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match *self {
            Error::Spectrum(ref e) => e.fmt(f),
            Error::Deconvolution(ref e) => e.fmt(f),
            #[cfg(any(feature = "bruker", feature = "jdx"))]
            Error::IoError(ref e) => e.fmt(f),
            #[cfg(feature = "hdf5")]
            Error::Hdf5Error(ref e) => e.fmt(f),
        }
    }
}
