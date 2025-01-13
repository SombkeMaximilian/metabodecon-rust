//! Error types for the deconvolution process.

/// An `Error` that occurred during the deconvolution process.
///
/// This type of error is generally unrecoverable and indicates a problem with
/// the input data. For example, if no peaks are detected in the input data,
/// or there are no signals in the part of the spectrum where they would be
/// expected.
///
/// See the [`Kind`] enum for the different kinds of errors that can occur.
#[derive(Clone, Debug)]
pub struct Error {
    /// The `Kind` of error that occurred.
    kind: Kind,
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

/// The kind of `Error` that can occur during the [`deconvolution`] process.
///
/// Marked as non-exhaustive to allow for new variants to be added in the future
/// without breaking compatibility.
///
/// [`deconvolution´]: crate::deconvolution
#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum Kind {
    /// No peaks were detected in the input data.
    ///
    /// Most of the time this will happen if the intensities of the [`Spectrum`]
    /// were not read correctly or if something went wrong during the smoothing
    /// process.
    ///
    /// [`Spectrum`]: crate::spectrum::Spectrum
    NoPeaksDetected,
    /// No peaks were found in the part of the spectrum where signals would be
    /// expected.
    ///
    /// This can happen if there were no signals in the signal region or if all
    /// the signals were filtered out by the peak selection algorithm if the
    /// threshold was set too high.
    EmptySignalRegion,
    /// No peaks were found in the part of the spectrum where random signals due
    /// to noise would be expected.
    ///
    /// This is an error because the deconvolution process uses the noise peaks
    /// to estimate the noise level in the spectrum and filter out peaks that
    /// are likely to be random noise.
    EmptySignalFreeRegion,
}

impl std::error::Error for Error {}

impl From<Kind> for Error {
    fn from(kind: Kind) -> Self {
        Self { kind }
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        use self::Kind::*;
        let description = match &self.kind {
            NoPeaksDetected => "no peaks detected in the spectrum",
            EmptySignalRegion => "no peaks found in the signal region of the spectrum",
            EmptySignalFreeRegion => "no peaks found in the signal-free region of the spectrum",
        };
        write!(f, "{description}")
    }
}
