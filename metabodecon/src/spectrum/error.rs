//! Error types for the spectrum module.

use std::path::PathBuf;

/// The `Error` type for constructing a [`Spectrum`] or parsing 1D NMR data from
/// files.
///
/// [`Spectrum`]: crate::spectrum::Spectrum
///
/// This type of error is generally unrecoverable and indicates a problem with
/// the input data itself or the file format it is stored in. For example, the
/// input data is empty, a file of the Bruker TopSpin format is missing, or
/// metadata within one of the files is missing.
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

/// The kind of `Error` that can occur while constructing a [`Spectrum`] or
/// reading 1D NMR data.
///
/// Marked as non-exhaustive to allow for new variants to be added in the future
/// without breaking compatibility.
///
/// [`Spectrum`]: crate::spectrum::Spectrum
#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum Kind {
    /// The input data is empty.
    ///
    /// The length of a [`Spectrum`] is not intended to be changed after it is
    /// constructed, so an empty [`Spectrum`] is simply not useful.
    ///
    /// [`Spectrum`]: crate::spectrum::Spectrum
    EmptyData {
        /// The number of elements in the chemical shifts vector.
        chemical_shifts: usize,
        /// The number of elements in the intensities vector.
        intensities: usize,
    },
    /// The input data lengths are mismatched.
    ///
    /// The length of a [`Spectrum`] is not intended to be changed after it is
    /// constructed. A mismatch in the lengths of the chemical shifts and
    /// intensities vectors would create an inconsistent [`Spectrum`].
    ///
    /// [`Spectrum`]: crate::spectrum::Spectrum
    DataLengthMismatch {
        /// The number of elements in the chemical shifts vector.
        chemical_shifts: usize,
        /// The number of elements in the intensities vector.
        intensities: usize,
    },
    /// The chemical shifts are not uniformly spaced.
    ///
    /// The step size between two consecutive chemical shift values needs to be
    /// consistent throughout the entire [`Spectrum`]. A situation where this is
    /// not the case can arise due to
    /// * an inconsistent step size between two values
    /// * the difference between two values being very close to zero
    /// * non-finite values in the chemical shifts
    ///
    /// [`Spectrum`]: crate::spectrum::Spectrum
    NonUniformSpacing {
        /// The positions of the non-uniformly spaced chemical shifts.
        positions: (usize, usize),
    },
    /// The intensities contain invalid values.
    ///
    /// Non-finite intensity values will lead to problems in further processing
    /// steps. Therefore, this state is considered inconsistent and results in
    /// an error.
    InvalidIntensities {
        /// The position of the first invalid intensity that was found.
        position: usize,
    },
    /// The signal boundaries are invalid.
    ///
    /// A certain structure is expected from a 1D NMR [`Spectrum`] with respect
    /// to the regions of interest. The region where signals are expected to be
    /// found is in the center of the [`Spectrum`], with signal free regions on
    /// either side. The following conditions are checked:
    /// * The signal boundaries are finite values
    /// * The signal boundaries are within the range of the chemical shifts
    /// * The signal region width is not close to zero
    ///
    /// [`Spectrum`]: crate::spectrum::Spectrum
    InvalidSignalBoundaries {
        /// The signal boundaries of the spectrum.
        signal_boundaries: (f64, f64),
        /// The range of the chemical shifts.
        chemical_shifts_range: (f64, f64),
    },

    /// Metadata is missing from a file of the various formats.
    ///
    /// This indicates that the stored data was corrupted or that the format of
    /// the file is not as expected. If you have a dataset that you believe
    /// should be parsable but is not, open an [issue] and provide the dataset.
    ///
    /// [issue]: https://github.com/SombkeMaximilian/metabodecon-rust/issues
    MissingMetadata {
        /// The path to the file where the metadata was expected.
        path: PathBuf,
        /// The regex pattern that was used to search for the metadata.
        regex: String,
    },
}

impl std::error::Error for Error {}

impl From<Kind> for Error {
    fn from(kind: Kind) -> Self {
        Self { kind }
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use self::Kind::*;
        let description = match &self.kind {
            EmptyData {
                chemical_shifts,
                intensities,
            } => format!(
                "input data is empty \
                 chemical shifts has {} elements, \
                 intensities has {} elements",
                chemical_shifts, intensities
            ),
            DataLengthMismatch {
                chemical_shifts,
                intensities,
            } => format!(
                "input data lengths are mismatched \
                 chemical shifts has {} elements, \
                 intensities has {} elements",
                chemical_shifts, intensities
            ),
            NonUniformSpacing { positions } => format!(
                "chemical shifts are not uniformly spaced \
                 values at index {} or {} are either not uniformly spaced, \
                 not finite numbers, or their difference is near zero",
                positions.0, positions.1
            ),
            InvalidIntensities { position } => format!(
                "intensities contain invalid values \
                 value at index {} is not a finite number",
                position
            ),
            InvalidSignalBoundaries {
                signal_boundaries,
                chemical_shifts_range,
            } => format!(
                "signal boundaries are invalid \
                 boundaries are {:?}, \
                 spectrum range is {:?}",
                signal_boundaries, chemical_shifts_range
            ),
            MissingMetadata { path, regex } => format!(
                "missing metadata \
                 expected in file at {:?} \
                 with regex {}",
                path, regex
            ),
        };
        write!(f, "{description}")
    }
}
