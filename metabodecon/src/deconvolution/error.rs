use std::sync::Arc;

/// An `Error` that occurred during the deconvolution process.
///
/// This type of error is generally unrecoverable and indicates a problem with
/// the input data. For example, if no peaks are detected in the input data,
/// or there are no signals in the part of the spectrum where they would be
/// expected.
#[derive(Clone, Debug)]
pub struct Error {
    /// The `Kind` of error that occurred.
    kind: Kind,
    /// The source of the error, if any. Internal errors have no source, while
    /// I/O errors and HDF5 errors have a source.
    source: Option<Arc<dyn std::error::Error>>,
}

impl Error {
    /// Constructs a new `Error` from the given `Kind`.
    pub fn new(kind: Kind) -> Self {
        kind.into()
    }

    /// Returns the `Kind` of the `Error`.
    pub fn kind(&self) -> &Kind {
        &self.kind
    }
}

/// The kind of `Error` that can occur during the deconvolution process.
///
/// Marked as non-exhaustive to allow for new variants to be added in the future
/// without breaking compatibility.
#[non_exhaustive]
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Kind {
    /// No peaks were detected in the input data.
    NoPeaksDetected,
    /// No peaks were found in the part of the spectrum where signals would be
    /// expected.
    EmptySignalRegion,
    /// No peaks were found in the part of the spectrum where random signals due
    /// to noise would be expected.
    ///
    /// This is an error because the deconvolution process uses the noise peaks
    /// to estimate the noise level in the spectrum and filter out peaks that
    /// are likely to be random noise.
    EmptySignalFreeRegion,
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|s| &**s as _)
    }
}

impl From<Kind> for Error {
    fn from(kind: Kind) -> Self {
        Self { kind, source: None }
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:?}", self.kind)
    }
}
