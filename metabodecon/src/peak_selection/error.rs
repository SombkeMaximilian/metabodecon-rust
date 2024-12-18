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
    NoPeaksDetected,
    NoPeaksInSignalRegion,
    NoPeaksInSignalFreeRegion,
    NoPeaksInWaterRegion,
}

impl std::error::Error for Error {}

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

impl Error {
    pub fn kind(&self) -> &Kind {
        &self.kind
    }
}
