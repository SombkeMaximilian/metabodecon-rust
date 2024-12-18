use crate::spectrum::Monotonicity;
use std::path::PathBuf;
use std::sync::Arc;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub struct Error {
    kind: Kind,
    source: Option<Arc<dyn std::error::Error>>,
}

#[non_exhaustive]
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Kind {
    EmptyData {
        chemical_shifts: usize,
        intensities: usize,
    },
    DataLengthMismatch {
        chemical_shifts: usize,
        intensities: usize,
    },
    MonotonicityMismatch {
        chemical_shifts: Monotonicity,
        signal_boundaries: Monotonicity,
        water_boundaries: Monotonicity,
    },
    NonUniformSpacing {
        positions: (usize, usize),
    },

    MissingAcqus {
        path: PathBuf,
    },
    MissingProcs {
        path: PathBuf,
    },
    Missing1r {
        path: PathBuf,
    },
    MissingJdx {
        path: PathBuf,
    },
    MissingMetadata {
        path: PathBuf,
        regex: String,
    },

    IoError,
    Hdf5Error,
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

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self {
            kind: Kind::IoError,
            source: Some(Arc::new(error)),
        }
    }
}

impl From<hdf5::Error> for Error {
    fn from(error: hdf5::Error) -> Self {
        Self {
            kind: Kind::Hdf5Error,
            source: Some(Arc::new(error)),
        }
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
            NonUniformSpacing { positions } => format!(
                "chemical shifts are not uniformly spaced \
                 step size at indices ({}, {}) differs from previous step",
                positions.0, positions.1
            ),
            IoError => format!("I/O error: {}", self.source.as_ref().unwrap()),
            Hdf5Error => format!("HDF5 error: {}", self.source.as_ref().unwrap()),
        };
        write!(f, "{description}")
    }
}

impl Error {
    pub fn new(kind: Kind) -> Self {
        kind.into()
    }

    pub fn kind(&self) -> &Kind {
        &self.kind
    }
}
