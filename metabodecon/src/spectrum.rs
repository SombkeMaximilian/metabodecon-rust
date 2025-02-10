//! A module for representing and parsing 1D NMR spectra from various file
//! formats.
//!
//! # Key Components
//!
//! - [`Spectrum`]: A container for spectral 1D NMR data and metadata.
//! - [`Bruker`]: An interface for parsing Bruker TopSpin NMR data.
//! - [`JcampDx`]: An interface for parsing spectra from JCAMP-DX files. (WIP)
//! - [`Hdf5`]: An interface for reading from and writing to HDF5 files in the
//!   format used by this library.
//!
//! # Example: Reading multiple spectra from Bruker TopSpin format
//!
//! This library supports reading spectra from Bruker TopSpin, a proprietary NMR
//! format. [Read more](Bruker)
//!
//! ```
//! use metabodecon::spectrum::Bruker;
//!
//! # fn main() -> metabodecon::Result<()> {
//! let path = "path/to/root";
//! # let path = "../data/bruker/blood";
//!
//! // Read all spectra from Bruker TopSpin format directories within the root.
//! let spectra = Bruker::read_spectra(
//!     path,
//!     // Experiment number
//!     10,
//!     // Processing number
//!     10,
//!     // Signal boundaries
//!     (-2.2, 11.8),
//! )?;
//! # Ok(())
//! # }
//! ```
//!
//! # Example: Reading multiple spectra from JCAMP-DX files
//!
//! JCAMP-DX is a common open format for NMR data. Support for this format is
//! currently a work in progress. [Read more](JcampDx)
//!
//! # Example: Constructing a `Spectrum` manually
//!
//! The following example demonstrates how to create a `Spectrum` object from
//! scratch. This is typically not how spectra are created, as they are usually
//! parsed from files. However, it can be useful for generating synthetic data
//! or handling custom formats. [Read more](Spectrum)
//!
//! ```
//! use metabodecon::spectrum::Spectrum;
//!
//! # fn main() -> metabodecon::Result<()> {
//! // Generate 2^15 chemical shifts between 0 and 10 ppm.
//! let chemical_shifts = (0..2_u32.pow(15))
//!     .map(|i| i as f64 * 10.0 / (2_f64.powi(15) - 1.0))
//!     .collect::<Vec<f64>>();
//!
//! // Generate intensities using 2 Lorentzian peaks.
//! let intensities = chemical_shifts
//!     .iter()
//!     .map(|x| {
//!         // Left signal centered at 3 ppm.
//!         1.0 * 0.25 / (0.25_f64.powi(2) + (x - 3.0).powi(2))
//!             // Right signal centered at 7 ppm.
//!             + 1.0 * 0.25 / (0.25_f64.powi(2) + (x - 7.0).powi(2))
//!     })
//!     .collect::<Vec<f64>>();
//!
//! // Define the signal region.
//! let signal_boundaries = (1.0, 9.0);
//!
//! // Create a Spectrum object.
//! let spectrum =
//!     Spectrum::new(chemical_shifts, intensities, signal_boundaries)?;
//! # Ok(())
//! # }
//! ```

mod spectrum;
pub use spectrum::Spectrum;

#[cfg(feature = "serde")]
mod serialized_spectrum;
#[cfg(feature = "serde")]
pub(crate) use serialized_spectrum::SerializedSpectrum;

pub mod meta;

mod formats;
#[cfg(feature = "bruker")]
pub use formats::Bruker;
#[cfg(feature = "hdf5")]
pub use formats::Hdf5;
#[cfg(feature = "jdx")]
pub use formats::JcampDx;

pub mod error;
