#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

//! A library for reading, deconvolution, alignment and postprocessing of 1D NMR
//! spectra.
//!
//! Metabodecon is a collection of algorithms proposed in various scientific
//! papers, as well as our own work, that have been implemented in Rust.
//!
//! * Tools for working with 1D NMR data, Lorentzian peak shapes, and
//!   deconvolution results
//! * Functionality for reading data from the various NMR storage formats
//! * An implementation of the automated feature extraction algorithm for the
//!   deconvolution of 1D NMR spectra described in [Koh et al. (2009)]
//! * An implementation of the alignment algorithm described in [Vu et al.
//!   (2011)] and [Beirnaert et al. (2018)]
//!
//! [Koh et al. (2009)]: https://doi.org/10.1016/j.jmr.2009.09.003.
//! [Vu et al. (2011)]: https://doi.org/10.1186/1471-2105-12-405
//! [Beirnaert et al. (2018)]: https://doi.org/10.1371/journal.pcbi.1006018

#[macro_use]
pub(crate) mod macros;
pub(crate) const CHECK_PRECISION: f64 = 1.0e+3 * f64::EPSILON;

pub mod spectrum;

pub mod deconvolution;

mod error;
pub use error::{Error, Result};
