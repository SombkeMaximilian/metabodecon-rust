#[cfg(feature = "bruker")]
mod bruker;
#[cfg(feature = "bruker")]
pub use bruker::Bruker;

#[cfg(feature = "hdf5")]
mod hdf5;
#[cfg(feature = "hdf5")]
pub use hdf5::Hdf5;

#[rustfmt::skip]
#[allow(dead_code)]
#[cfg(feature = "jdx")]
mod jcampdx;
#[cfg(feature = "jdx")]
pub use jcampdx::JcampDx;
