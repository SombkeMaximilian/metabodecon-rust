use crate::error::Result;
use crate::spectrum::{Error, Kind, Spectrum};
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use regex::Regex;
use std::fs::{read_to_string, File};
use std::io::Read;
use std::path::{Path, PathBuf};

#[derive(Debug)]
enum Endian {
    Little,
    Big,
}

#[derive(Debug)]
enum Type {
    I32,
    F64,
}

#[derive(Debug)]
struct AcquisitionParameters {
    pub spectrum_width: f64,
}

#[derive(Debug)]
struct ProcessingParameters {
    pub spectrum_maximum: f64,
    pub scaling_exponent: i32,
    pub endian: Endian,
    pub data_type: Type,
    pub data_size: usize,
}

#[derive(Default)]
pub struct BrukerReader;

impl BrukerReader {
    pub fn new() -> Self {
        Self
    }

    pub fn read_spectrum<P: AsRef<Path>>(
        &self,
        path: P,
        experiment: u32,
        processing: u32,
        signal_boundaries: (f64, f64),
        water_boundaries: (f64, f64),
    ) -> Result<Spectrum> {
        let acqus_path = path
            .as_ref()
            .join(format!("{}/acqus", experiment));
        if !acqus_path.is_file() {
            return Err(Error::new(Kind::MissingAcqus { path: acqus_path }).into());
        }
        let procs_path = path
            .as_ref()
            .join(format!("{}/pdata/{}/procs", experiment, processing));
        if !procs_path.is_file() {
            return Err(Error::new(Kind::MissingProcs { path: procs_path }).into());
        }
        let one_r_path = path
            .as_ref()
            .join(format!("{}/pdata/{}/1r", experiment, processing));
        if !one_r_path.is_file() {
            return Err(Error::new(Kind::Missing1r { path: one_r_path }).into());
        }

        let acqus = self.read_acquisition_parameters(acqus_path)?;
        let procs = self.read_processing_parameters(procs_path)?;
        let chemical_shifts = (0..procs.data_size)
            .map(|i| {
                procs.spectrum_maximum - acqus.spectrum_width
                    + (i as f64) * acqus.spectrum_width / (procs.data_size as f64)
            })
            .collect::<Vec<f64>>();
        let intensities = self.read_one_r(one_r_path, procs)?;
        let spectrum = Spectrum::new(
            chemical_shifts,
            intensities,
            signal_boundaries,
            water_boundaries,
        )?;

        Ok(spectrum)
    }

    pub fn read_spectra<P: AsRef<Path>>(
        &self,
        path: P,
        experiment: u32,
        processing: u32,
        signal_boundaries: (f64, f64),
        water_boundaries: (f64, f64),
    ) -> Result<Vec<Spectrum>> {
        let spectra_roots = path
            .as_ref()
            .read_dir()?
            .filter(|entry| entry.is_ok())
            .filter(|entry| entry.as_ref().unwrap().path().is_dir())
            .map(|entry| entry.unwrap().path())
            .collect::<Vec<PathBuf>>();
        let spectra = spectra_roots
            .into_iter()
            .map(|root| {
                self.read_spectrum(
                    root,
                    experiment,
                    processing,
                    signal_boundaries,
                    water_boundaries,
                )
            })
            .collect::<Result<Vec<Spectrum>>>()?;

        Ok(spectra)
    }

    fn read_acquisition_parameters<P: AsRef<Path>>(
        &self,
        path: P,
    ) -> Result<AcquisitionParameters> {
        let acqus = read_to_string(path.as_ref())?;
        let width_re = Regex::new(r"(##\$SW=\s*)(?P<width>\d+(\.\d+)?)").unwrap();

        let spectrum_width = extract_capture!(width_re, &acqus, "width", path);

        Ok(AcquisitionParameters { spectrum_width })
    }

    fn read_processing_parameters<P: AsRef<Path>>(&self, path: P) -> Result<ProcessingParameters> {
        let procs = read_to_string(path.as_ref())?;
        let maximum_re = Regex::new(r"(##\$OFFSET=\s*)(?P<maximum>\d+(\.\d+)?)").unwrap();
        let endian_re = Regex::new(r"(##\$BYTORDP=\s*)(?P<endian>\d)").unwrap();
        let exponent_re = Regex::new(r"(##\$NC_proc=\s*)(?P<exponent>-?\d+)").unwrap();
        let data_type_re = Regex::new(r"(##\$DTYPP=\s*)(?P<data_type>\d)").unwrap();
        let data_size_re = Regex::new(r"(##\$SI=\s*)(?P<data_size>\d+)").unwrap();

        let spectrum_maximum = extract_capture!(maximum_re, &procs, "maximum", path);
        let scaling_exponent = extract_capture!(exponent_re, &procs, "exponent", path);
        let endian = match extract_capture!(endian_re, &procs, "endian", path) {
            0 => Endian::Little,
            _ => Endian::Big,
        };
        let data_type = match extract_capture!(data_type_re, &procs, "data_type", path) {
            0 => Type::I32,
            _ => Type::F64,
        };
        let data_size = extract_capture!(data_size_re, &procs, "data_size", path);

        Ok(ProcessingParameters {
            spectrum_maximum,
            scaling_exponent,
            endian,
            data_type,
            data_size,
        })
    }

    fn read_one_r(&self, path: PathBuf, procs: ProcessingParameters) -> Result<Vec<f64>> {
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
