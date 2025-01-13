//! A module for representing and parsing 1D NMR spectra from various file
//! formats.
//!
//! This module provides a number of types for handling 1D NMR data.
//! * [`Spectrum`] is a container for the spectral data as well as metadata.
//! * [`BrukerReader`] is an interface for parsing Bruker TopSpin NMR data.
//! * [`JdxReader`] is an interface for reading spectra from JCAMP-DX files.
//!   (WIP)
//! * [`Hdf5Reader`] is an interface for reading spectra from HDF5 files in the
//!   format used by this library.
//!
//! # Example: Constructing a `Spectrum` manually
//!
//! The following example demonstrates how to create a `Spectrum` object from
//! scratch. This is generally not how spectra should be created, as they are
//! usually parsed from files. However, if you need to generate synthetic data
//! or have data in a custom format, this is how you can do it.
//! [Read more](Spectrum)
//!
//! ```
//! use metabodecon::spectrum::Spectrum;
//!
//! # fn main() -> metabodecon::Result<()> {
//! // Generate 2^15 chemical shifts between 0 and 10 ppm.
//! let chemical_shifts = (0..2_u32.pow(15))
//!     .into_iter()
//!     .map(|i| i as f64 * 10.0 / (2_f64.powi(15) - 1.0))
//!     .collect::<Vec<f64>>();
//!
//! // Generate intensities using 3 Lorentzian peaks.
//! let intensities = chemical_shifts
//!     .iter()
//!     .map(|x| {
//!         // Left signal centered at 3 ppm.
//!         1.0 * 0.25 / (0.25_f64.powi(2) + (x - 3.0).powi(2))
//!             // Mock water artifact centered at 5 ppm (not a realistic shape).
//!             + 0.1 * 0.05 / (0.05_f64.powi(2) + (x - 5.0).powi(2))
//!             // Right signal centered at 7 ppm.
//!             + 1.0 * 0.25 / (0.25_f64.powi(2) + (x - 7.0).powi(2))
//!     })
//!     .collect::<Vec<f64>>();
//!
//! // Define the signal region and water artifact boundaries.
//! let signal_boundaries = (1.0, 9.0);
//! let water_boundaries = (4.5, 5.5);
//!
//! // Create a Spectrum object.
//! let spectrum = Spectrum::new(
//!     chemical_shifts,
//!     intensities,
//!     signal_boundaries,
//!     water_boundaries,
//! )?;
//! # Ok(())
//! # }
//! ```
//!
//! # Example: Reading multiple spectra from Bruker TopSpin format
//!
//! One of the proprietary formats that this library can read is the one used by
//! Bruker TopSpin. [Read more](BrukerReader)
//!
//! ```
//! use metabodecon::spectrum::BrukerReader;
//!
//! # fn main() -> metabodecon::Result<()> {
//! let reader = BrukerReader::new();
//! let path = "path/to/root";
//! # let path = "../data/bruker/blood";
//!
//! // Read all spectra from Bruker TopSpin format directories within the root.
//! let spectra = reader.read_spectra(
//!     path,
//!     // Experiment number
//!     10,
//!     // Processing number
//!     10,
//!     // Signal boundaries
//!     (-2.2, 11.8),
//!     // Water boundaries
//!     (4.7, 4.9),
//! )?;
//! # Ok(())
//! # }
//! ```
//!
//! # Example: Reading multiple spectra from JCAMP-DX files
//!
//! WIP
//!
//! # Example: Reading multiple spectra from an HDF5 file
//!
//! HDF5 offers a simple way to store hierarchical data. This library uses a
//! specific structure to store 1D NMR spectra in HDF5 files.
//! [Read more](Hdf5Reader)
//!
//! ```
//! use metabodecon::spectrum::Hdf5Reader;
//!
//! # fn main() -> metabodecon::Result<()> {
//! let reader = Hdf5Reader::new();
//! let path = "path/to/file.h5";
//! # let path = "../data/hdf5/blood.h5";
//!
//! // Read all spectra from the HDF5 file.
//! let spectra = reader.read_spectra(path)?;
//! # Ok(())
//! # }
//! ```
mod bruker_reader;
mod hdf5_reader;
mod macros;
#[rustfmt::skip] #[allow(dead_code)] mod jdx_reader;
mod spectrum;

pub use spectrum::{Monotonicity, Spectrum};

pub use bruker_reader::BrukerReader;
pub use hdf5_reader::Hdf5Reader;
pub use jdx_reader::JdxReader;

pub mod error;
