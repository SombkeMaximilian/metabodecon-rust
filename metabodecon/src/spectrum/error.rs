use crate::spectrum::Monotonicity;
use std::path::PathBuf;

/// An `Error` that occurred while constructing a [`Spectrum`] or reading 1D NMR
/// data.
///
/// This type of error is generally unrecoverable and indicates a problem with
/// the input data itself or the file format it is stored in. For example,
/// if the input data is empty, a file of the Bruker TopSpin format is missing,
/// or metadata within one of these files is missing.
///
/// [`Spectrum`]: crate::spectrum::Spectrum
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
    EmptyData {
        /// The number of elements in the chemical shifts vector.
        chemical_shifts: usize,
        /// The number of elements in the intensities vector.
        intensities: usize,
    },
    /// The input data lengths are mismatched.
    DataLengthMismatch {
        /// The number of elements in the chemical shifts vector.
        chemical_shifts: usize,
        /// The number of elements in the intensities vector.
        intensities: usize,
    },
    /// The chemical shifts are not uniformly spaced.
    ///
    /// This occurs when a step size between two chemical shifts is not
    /// equal to the expected step size. This may indicate that the data is
    /// corrupted (incorrectly computed, duplicate or missing chemical shifts).
    NonUniformSpacing {
        /// The positions of the non-uniformly spaced chemical shifts.
        positions: (usize, usize),
    },
    /// The intensities contain invalid values.
    InvalidIntensities {
        /// The position of the first invalid intensity that was found.
        position: usize,
    },
    /// The input data is not consistently ordered according to the same
    /// [`Monotonicity`].
    ///
    /// This occurs when the chemical shifts, signal boundaries, and water
    /// boundaries are provided with mismatched monotonicity. For example, if
    /// the chemical shifts are in decreasing order but the boundary tuples are
    /// in increasing order, it is likely that the intensities are also ordered
    /// incorrectly relative to the chemical shifts. This is unlikely to be
    /// intentional and is likely a mistake in the input data. Therefore, it is
    /// better to return an error in this case than to silently continue with
    /// potentially incorrect data.
    MonotonicityMismatch {
        /// The ordering of the chemical shifts vector.
        chemical_shifts: Monotonicity,
        /// The ordering of the signal boundaries vector.
        signal_boundaries: Monotonicity,
        /// The ordering of the water boundaries vector.
        water_boundaries: Monotonicity,
    },
    /// The signal boundaries are invalid.
    ///
    /// This occurs when the signal boundaries are either not within the range
    /// of the chemical shifts, or if the difference between the upper and lower
    /// bound is very close to zero or not a number.
    InvalidSignalBoundaries {
        /// The signal boundaries of the spectrum.
        signal_boundaries: (f64, f64),
        /// The range of the chemical shifts.
        chemical_shifts_range: (f64, f64),
    },
    /// The water boundaries are invalid
    ///
    /// This occurs when the water boundaries are either not within the range
    /// of the chemical shifts, or if the difference between the upper and lower
    /// bound is very close to zero or not a number.
    InvalidWaterBoundaries {
        /// The water boundaries of the spectrum.
        water_boundaries: (f64, f64),
        /// The signal boundaries of the spectrum.
        signal_boundaries: (f64, f64),
        /// The range of the chemical shifts.
        chemical_shifts_range: (f64, f64),
    },

    /// The acqus file of the Bruker TopSpin format is missing.
    ///
    /// This indicates corruption or misplacement of the input data, or
    /// that an incorrect path was provided.
    MissingAcqus {
        /// The path where the acqus file was expected.
        path: PathBuf,
    },
    /// The procs file of the Bruker TopSpin format is missing.
    ///
    /// This indicates corruption or misplacement of the input data, or
    /// that an incorrect path was provided.
    MissingProcs {
        /// The path where the procs file was expected.
        path: PathBuf,
    },
    /// The 1r file of the Bruker TopSpin format is missing.
    ///
    /// This indicates corruption or misplacement of the input data, or
    /// that an incorrect path was provided.
    Missing1r {
        /// The path where the 1r file was expected.
        path: PathBuf,
    },
    /// The file of the JDX format is missing.
    ///
    /// This indicates that an incorrect path was provided.
    MissingJdx {
        /// The path where the JDX file was expected.
        path: PathBuf,
    },
    /// Metadata is missing from a file.
    ///
    /// This indicates that the stored data was corrupted or that the
    /// format of the file is not as expected.
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
                 not normal numbers, or their difference is near zero",
                positions.0, positions.1
            ),
            InvalidIntensities { position } => format!(
                "intensities contain invalid values \
                 value at index {} is not a normal number",
                position
            ),
            MonotonicityMismatch {
                chemical_shifts,
                signal_boundaries,
                water_boundaries,
            } => format!(
                "input data is not monotonic (intensities may be incorrect) \
                 chemical shifts is {:?}, \
                 signal boundaries is {:?}, \
                 water boundaries is {:?}",
                chemical_shifts, signal_boundaries, water_boundaries
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
            InvalidWaterBoundaries {
                water_boundaries,
                signal_boundaries,
                chemical_shifts_range,
            } => format!(
                "water boundaries are invalid \
                 boundaries are {:?}, \
                 signal boundaries are {:?} \
                 spectrum range is {:?}",
                water_boundaries, signal_boundaries, chemical_shifts_range
            ),
            MissingAcqus { path } => format!(
                "missing acqus file \
                 expected at {:?}",
                path
            ),
            MissingProcs { path } => format!(
                "missing procs file \
                 expected at {:?}",
                path
            ),
            Missing1r { path } => format!(
                "missing 1r file \
                 expected at {:?}",
                path
            ),
            MissingJdx { path } => format!(
                "missing jdx file \
                 expected at {:?}",
                path
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
