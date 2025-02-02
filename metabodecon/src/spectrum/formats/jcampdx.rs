use crate::Result;
use crate::spectrum::Spectrum;
use crate::spectrum::error::{Error, Kind};
use crate::spectrum::macros::extract_capture;
use regex::Regex;
use std::fs::read_to_string;
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

#[derive(Debug)]
enum Type {
    I32,
    F64,
}

#[derive(Debug)]
struct MetaData {
    pub spectrum_width: f64,
    pub spectrum_maximum: f64,
    pub scaling_exponent: i32,
    pub data_type: Type,
}

impl JcampDx {
    /// Reads the spectrum from a JCAMP-DX file.
    pub fn read_spectrum<P: AsRef<Path>>(
        &self,
        path: P,
        signal_boundaries: (f64, f64),
    ) -> Result<Spectrum> {
        let _meta = self.read_meta_data(path.as_ref())?;
        let spectrum = Spectrum::new(Vec::new(), Vec::new(), signal_boundaries)?;

        Ok(spectrum)
    }

    fn read_meta_data<P: AsRef<Path>>(&self, path: P) -> Result<MetaData> {
        let meta = read_to_string(path.as_ref())?;
        let width_re = Regex::new(r"(##\$SW=\s*)(?P<width>\d+(\.\d+)?)").unwrap();
        let maximum_re = Regex::new(r"(##\$OFFSET=\s*)(?P<maximum>\d+(\.\d+)?)").unwrap();
        let exponent_re = Regex::new(r"(##\$NC_proc=\s*)(?P<exponent>-?\d+)").unwrap();
        let data_type_re = Regex::new(r"(##\$DTYPP=\s*)(?P<data_type>\d)").unwrap();

        let spectrum_width = extract_capture!(width_re, &meta, "width", path);
        let spectrum_maximum = extract_capture!(maximum_re, &meta, "maximum", path);
        let scaling_exponent = extract_capture!(exponent_re, &meta, "exponent", path);
        let data_type = match extract_capture!(data_type_re, &meta, "data_type", path) {
            0 => Type::I32,
            _ => Type::F64,
        };

        Ok(MetaData {
            spectrum_width,
            spectrum_maximum,
            scaling_exponent,
            data_type,
        })
    }
}
