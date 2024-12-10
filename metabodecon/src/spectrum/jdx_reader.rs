use crate::spectrum::Spectrum;
use regex::Regex;
use std::fs::read_to_string;
use std::io::{self};
use std::path::Path;

#[derive(Debug, Clone, Copy)]
enum Type {
    I32,
    F64,
}

#[derive(Debug, Clone, Copy)]
struct MetaData {
    pub spectrum_width: f64,
    pub spectrum_maximum: f64,
    pub scaling_exponent: i32,
    pub data_type: Type,
}

#[derive(Debug, Clone, Copy)]
pub struct JdxReader;

impl JdxReader {
    pub fn new() -> Self {
        JdxReader
    }

    pub fn read_spectrum<P: AsRef<Path>>(&self, path: P) -> io::Result<Spectrum> {
        let _meta = self.read_meta_data(path.as_ref())?;

        Ok(Spectrum::new(Vec::new(), Vec::new(), (0., 0.), (0., 0.)))
    }

    fn read_meta_data<P: AsRef<Path>>(&self, path: P) -> io::Result<MetaData> {
        let meta = read_to_string(path)?;
        let width_re = Regex::new(r"(##\$SW=\s*)(?P<width>\d+(\.\d+)?)").unwrap();
        let maximum_re = Regex::new(r"(##\$OFFSET=\s*)(?P<maximum>\d+(\.\d+)?)").unwrap();
        let exponent_re = Regex::new(r"(##\$NC_proc=\s*)(?P<exponent>-?\d+)").unwrap();
        let data_type_re = Regex::new(r"(##\$DTYPP=\s*)(?P<data_type>\d)").unwrap();

        Ok(MetaData {
            spectrum_width: extract_capture!(width_re, &meta, width),
            spectrum_maximum: extract_capture!(maximum_re, &meta, maximum),
            scaling_exponent: extract_capture!(exponent_re, &meta, exponent),
            data_type: match extract_capture!(data_type_re, &meta, data_type) {
                0 => Type::I32,
                _ => Type::F64,
            },
        })
    }
}

impl Default for JdxReader {
    fn default() -> Self {
        Self::new()
    }
}
