use crate::spectrum::Spectrum;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use regex::Regex;
use std::fs::{read_to_string, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy)]
enum Endian {
    Little,
    Big,
}

#[derive(Debug, Clone, Copy)]
enum Type {
    I32,
    F64,
}

#[derive(Debug, Clone, Copy)]
struct AcquisitionParameters {
    pub spectrum_width: f64,
}

#[derive(Debug, Clone, Copy)]
struct ProcessingParameters {
    pub spectrum_maximum: f64,
    pub scaling_exponent: i32,
    pub endian: Endian,
    pub data_type: Type,
    pub data_size: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct BrukerReader<P: AsRef<Path>> {
    path: P,
}

impl<P: AsRef<Path>> BrukerReader<P> {
    pub fn new(path: P) -> Self {
        BrukerReader { path }
    }

    pub fn read_spectrum(&self, experiment: u32, processing: u32) -> io::Result<Spectrum> {
        let acqus_path = self
            .path
            .as_ref()
            .join(format!("{}/acqus", experiment));
        let procs_path = self
            .path
            .as_ref()
            .join(format!("{}/pdata/{}/procs", experiment, processing));
        let one_r_path = self
            .path
            .as_ref()
            .join(format!("{}/pdata/{}/1r", experiment, processing));
        let acqus = self.read_acquisition_parameters(acqus_path)?;
        let procs = self.read_processing_parameters(procs_path)?;
        let chemical_shifts = (0..procs.data_size)
            .map(|i| {
                procs.spectrum_maximum - acqus.spectrum_width
                    + (i as f64) * acqus.spectrum_width / (procs.data_size as f64)
            })
            .collect::<Vec<f64>>();
        let intensities = self.read_one_r(one_r_path, procs)?;

        Ok(Spectrum::new(
            chemical_shifts,
            intensities,
            (0., 0.),
            (0., 0.),
        ))
    }

    fn read_acquisition_parameters(&self, path: PathBuf) -> io::Result<AcquisitionParameters> {
        let acqus = read_to_string(path)?;
        let width_re = Regex::new(r"(##\$SW=\s*)(?P<width>\d+(\.\d+)?)").unwrap();

        Ok(AcquisitionParameters {
            spectrum_width: extract_capture!(width_re, &acqus, width),
        })
    }

    fn read_processing_parameters(&self, path: PathBuf) -> io::Result<ProcessingParameters> {
        let procs = read_to_string(path)?;
        let maximum_re = Regex::new(r"(##\$OFFSET=\s*)(?P<maximum>\d+(\.\d+)?)").unwrap();
        let endian_re = Regex::new(r"(##\$BYTORDP=\s*)(?P<endian>\d)").unwrap();
        let exponent_re = Regex::new(r"(##\$NC_proc=\s*)(?P<exponent>-?\d+)").unwrap();
        let data_type_re = Regex::new(r"(##\$DTYPP=\s*)(?P<data_type>\d)").unwrap();
        let data_size_re = Regex::new(r"(##\$SI=\s*)(?P<data_size>\d+)").unwrap();

        Ok(ProcessingParameters {
            spectrum_maximum: extract_capture!(maximum_re, &procs, maximum),
            scaling_exponent: extract_capture!(exponent_re, &procs, exponent),
            endian: match extract_capture!(endian_re, &procs, endian) {
                0 => Endian::Little,
                _ => Endian::Big,
            },
            data_type: match extract_capture!(data_type_re, &procs, data_type) {
                0 => Type::I32,
                _ => Type::F64,
            },
            data_size: extract_capture!(data_size_re, &procs, data_size),
        })
    }

    fn read_one_r(&self, path: PathBuf, procs: ProcessingParameters) -> io::Result<Vec<f64>> {
        let mut one_r = File::open(path)?;
        let mut buffer = vec![
            0;
            procs.data_size
                * match procs.data_type {
                    Type::I32 => 4,
                    Type::F64 => 8,
                }
        ];
        one_r.read_exact(&mut buffer)?;

        match procs.data_type {
            Type::I32 => {
                let mut temp = vec![0i32; procs.data_size];
                match procs.endian {
                    Endian::Little => buffer
                        .as_slice()
                        .read_i32_into::<LittleEndian>(&mut temp)?,
                    Endian::Big => buffer
                        .as_slice()
                        .read_i32_into::<BigEndian>(&mut temp)?,
                }
                temp.reverse();
                Ok(temp
                    .into_iter()
                    .map(|value| (value as f64) * 2f64.powi(procs.scaling_exponent))
                    .collect::<Vec<f64>>())
            }
            Type::F64 => {
                let mut temp = vec![0f64; procs.data_size];
                match procs.endian {
                    Endian::Little => buffer
                        .as_slice()
                        .read_f64_into::<LittleEndian>(&mut temp)?,
                    Endian::Big => buffer
                        .as_slice()
                        .read_f64_into::<BigEndian>(&mut temp)?,
                }
                temp.reverse();
                Ok(temp)
            }
        }
    }
}
