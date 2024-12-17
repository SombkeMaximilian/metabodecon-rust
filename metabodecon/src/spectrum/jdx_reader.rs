use crate::spectrum::{Error, Kind, Result, Spectrum};
use regex::Regex;
use std::fs::read_to_string;
use std::path::Path;

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

#[derive(Default)]
pub struct JdxReader;

impl JdxReader {
    pub fn new() -> Self {
        Self
    }

    pub fn read_spectrum<P: AsRef<Path>>(
        &self,
        path: P,
        signal_boundaries: (f64, f64),
        water_boundaries: (f64, f64)
    ) -> Result<Spectrum> {
        if !path.as_ref().is_file() {
            return Err(Error::new(Kind::MissingJdxFile { path: path.as_ref().to_path_buf() }));
        }
        let _meta = self.read_meta_data(path.as_ref())?;
        let spectrum = Spectrum::new(Vec::new(), Vec::new(), signal_boundaries, water_boundaries)?;

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
