//! Data structures containing meta information about a 1D NMR spectrum.
//!
//! [`Spectrum`]: crate::spectrum::Spectrum

mod acquisition_mode;
pub use acquisition_mode::AcquisitionMode;

mod monotonicity;
pub use monotonicity::Monotonicity;

mod nucleus;
pub use nucleus::Nucleus;

mod reference;
pub use reference::{ReferenceCompound, ReferencingMethod};

mod solvent;
pub use solvent::Solvent;
