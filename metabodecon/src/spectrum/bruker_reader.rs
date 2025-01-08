use crate::error::Result;
use crate::spectrum::{Error, Kind, Spectrum};
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use regex::Regex;
use std::fs::{read_to_string, File};
use std::io::Read;
use std::path::{Path, PathBuf};

/// Unit struct for reading 1D NMR spectra in the Bruker TopSpin format.
///
/// The Bruker TopSpin file format stores metadata and data in various files.
/// Most of the stored information is not used in this implementation, but the
/// following files are required to read a spectrum:
///
/// ```markdown
/// name
/// └── name_01
///     └── experiment
///         ├── pdata
///         │   └── processing
///         │       ├── 1r
///         │       └── procs
///         └── acqus
/// ```
///
/// `name` is the name of the dataset, which can be any string. `name_01` is
/// the name of the sample. `experiment` is an integer that represents the
/// type of experiment. Usually a lab will have a convention for which number
/// corresponds to which type of experiment. For example 10 being a 1D NMR
/// experiment. `pdata` is the processing data directory and `processing` is
/// the processing number, which is an arbitrary integer.
///
/// # Metadata
///
/// The `acqus` and `procs` files contain the acquisition and processing
/// parameters, respectively. They are plain text files with key-value pairs,
/// where each line starts with `##$key=`. The values are extracted with
/// regular expressions.
///
/// From the `acqus` file, the following keys are required:
/// * `SW`: The spectral width in ppm as a floating point number.
///
/// From the `procs` file, the following keys are required:
/// * `OFFSET`: The maximum chemical shift in ppm as a floating point number.
/// * `SI`: The size of the data. 2^15 and 2^17 are the expected values.
/// * `BYTORDP`: The endianness of the data, encoded as an integer.
///
///   | Value | Endianness |
///   | ----- | ---------- |
///   | 0     | Little     |
///   | 1     | Big        |
///
/// * `DTYPP`: The data type the raw signal intensities are stored as, encoded
///   as an integer.
///
///   | Value | Type |
///   | ----- | ---- |
///   | 0     | i32  |
///   | 1     | f64  |
///
/// * `NC_proc`: The scaling exponent of the data. If the data is stored as
///   integers, it is scaled by 2 to the power of this value. If the data is
///   stored as floats, this value is unused.
///
/// # Raw Data
///
/// The raw data is stored in the `1r` file in binary format. The metadata
/// specifies how the data has to be read.
#[derive(Default)]
pub struct BrukerReader;

/// Endianness of the raw data. Extracted from the `procs` file.
///
/// | BYTORDP | Endianness |
/// | ------- | ---------- |
/// | 0       | Little     |
/// | 1       | Big        |
#[derive(Debug)]
enum Endian {
    /// Little-endian byte order.
    Little,
    /// Big-endian byte order.
    Big,
}

/// Data type of the raw data. Extracted from the `procs` file.
///
/// | DTYPP | Type |
/// | ----- | ---- |
/// | 0     | i32  |
/// | 1     | f64  |
#[derive(Debug)]
enum Type {
    /// Data stored as 32-bit integers.
    I32,
    /// Data stored as 64-bit floating point numbers.
    F64,
}

/// Acquisition parameters extracted from the `acqus` file.
#[derive(Debug)]
struct AcquisitionParameters {
    /// The spectral width in ppm.
    pub spectrum_width: f64,
}

/// Processing parameters extracted from the `procs` file.
#[derive(Debug)]
struct ProcessingParameters {
    /// The maximum chemical shift in ppm.
    pub spectrum_maximum: f64,
    /// The size of the data, expected to be 2^15 or 2^17.
    pub data_size: usize,
    /// The endianness of the data.
    pub endian: Endian,
    /// The data type of the raw signal intensities.
    pub data_type: Type,
    /// The scaling exponent of the data, if the data is stored as integers.
    pub scaling_exponent: i32,
}

impl BrukerReader {
    /// Constructs a new `BrukerReader`.
    pub fn new() -> Self {
        Self
    }

    /// Reads the spectrum in the provided dataset from a Bruker TopSpin
    /// directory at the provided path and returns it. Any errors are
    /// propagated to the caller.
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

    /// Reads all spectra in Bruker TopSpin format in subdirectories of the
    /// provided path and returns them. Any errors are propagated to the caller.
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

    /// Internal helper function to read the acquisition parameters from the
    /// `acqus` file and return them.
    fn read_acquisition_parameters<P: AsRef<Path>>(
        &self,
        path: P,
    ) -> Result<AcquisitionParameters> {
        let acqus = read_to_string(path.as_ref())?;
        let width_re = Regex::new(r"(##\$SW=\s*)(?P<width>\d+(\.\d+)?)").unwrap();

        let spectrum_width = extract_capture!(width_re, &acqus, "width", path);

        Ok(AcquisitionParameters { spectrum_width })
    }

    /// Internal helper function to read the processing parameters from the
    /// `procs` file and return them.
    fn read_processing_parameters<P: AsRef<Path>>(&self, path: P) -> Result<ProcessingParameters> {
        let procs = read_to_string(path.as_ref())?;
        let maximum_re = Regex::new(r"(##\$OFFSET=\s*)(?P<maximum>\d+(\.\d+)?)").unwrap();
        let data_size_re = Regex::new(r"(##\$SI=\s*)(?P<data_size>\d+)").unwrap();
        let endian_re = Regex::new(r"(##\$BYTORDP=\s*)(?P<endian>\d)").unwrap();
        let data_type_re = Regex::new(r"(##\$DTYPP=\s*)(?P<data_type>\d)").unwrap();
        let exponent_re = Regex::new(r"(##\$NC_proc=\s*)(?P<exponent>-?\d+)").unwrap();

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
            data_size,
            endian,
            data_type,
            scaling_exponent,
        })
    }

    /// Internal helper function to read the raw data from the `1r` file and
    /// return it as a vector of floating point numbers. As working with
    /// chemical shifts in increasing order is generally simpler, the vector is
    /// reversed before being returned.
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
                let mut temp = vec![0_i32; procs.data_size];
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
                    .map(|value| (value as f64) * 2_f64.powi(procs.scaling_exponent))
                    .collect::<Vec<f64>>())
            }
            Type::F64 => {
                let mut temp = vec![0_f64; procs.data_size];
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

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::assert_approx_eq;

    #[test]
    fn read_spectrum() {
        let path = "../data/bruker/sim/sim_01";
        let reader = BrukerReader::new();
        let spectrum = reader
            .read_spectrum(path, 10, 10, (3.339007, 3.553942), (3.444939, 3.448010))
            .unwrap();
        check_sim_spectrum!(spectrum);
    }

    #[test]
    fn read_spectra() {
        let path = "../data/bruker/sim";
        let reader = BrukerReader::new();
        let spectra = reader
            .read_spectra(path, 10, 10, (3.339007, 3.553942), (3.444939, 3.448010))
            .unwrap();
        assert_eq!(spectra.len(), 16);
        spectra.iter().for_each(|spectrum| {
            check_sim_spectrum!(spectrum);
        })
    }
}
