use crate::Result;
use crate::spectrum::Spectrum;
use std::path::Path;

/// Interface for reading 1D NMR spectra in JCAMP-DX format.
///
/// The JCAMP-DX format is a text-based format for storing 1D NMR spectra. Both
/// the metadata and the data are stored in the same file, which can be divided
/// into the header and the data section. The header contains metadata about the
/// spectrum. The data section contains the actual spectrum data as a table of
/// encoded values.
///
/// # Metadata
///
/// The metadata is stored as key-value pairs, where the lines start with
/// `##$key=`. The values are extracted with regular expressions.
#[derive(Debug)]
pub enum JcampDx {}

impl JcampDx {
    /// Reads the spectrum from a JCAMP-DX file.
    pub fn read_spectrum<P: AsRef<Path>>(
        &self,
        _path: P,
        signal_boundaries: (f64, f64),
    ) -> Result<Spectrum> {
        let spectrum = Spectrum::new(Vec::new(), Vec::new(), signal_boundaries)?;

        Ok(spectrum)
    }
}
