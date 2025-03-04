use crate::Result;
use crate::spectrum::Spectrum;
use crate::spectrum::error::{Error, Kind};
use crate::spectrum::formats::{extract_capture, extract_row};
use crate::spectrum::meta::{Nucleus, ReferenceCompound, ReferencingMethod};
use regex::{Captures, Regex};
use std::fs::read_to_string;
use std::path::Path;
use std::sync::LazyLock;

/// Interface for reading 1D NMR spectra in JCAMP-DX format.
///
/// The JCAMP-DX format is a text-based format for storing 1D NMR spectra. Both
/// the metadata and the data are stored in the same file, which can be divided
/// into the header and the data section. Since the format allows for a lot of
/// optionally included information, this implementation is restricted to the
/// minimally required sets to construct a 1D NMR spectrum from.
///
/// # Supported Versions and Formats
///
/// This implementation can currently only handle version 5.xx and 6.xx of the
/// format. Due to the fact that version 4.xx was not designed for NMR spectra
/// but for other kinds of spectroscopy data, it is not possible to construct
/// meaningful NMR spectra from it consistently. As a result, version 4.xx will
/// never be supported, but it may be possible to read in such files by changing
/// the version to 5.00 or 6.00, as long as the relevant metadata is present.
///
/// NMR data can be stored in a few different ways. The `NMR Spectrum` variant
/// is currently supported for the XYDATA and NTUPLES format. `NMR FID` may be
/// supported in the future. `NMR PEAK TABLE` and `NMR PEAK ASSIGNMENT` will
/// never be supported.
///
/// # Header Metadata
///
/// The metadata is stored as key-value pairs, where the lines start with
/// `##key=`. To successfully parse a 1D NMR spectrum, a minimal set of metadata
/// is required. This minimal set does not contain any verification of the data
/// integrity, and likely never will due to how much freedom the format allows.
/// Some additional information will also be extracted from the metadata if it
/// is present. The following information must be present in the header:
///
/// | Key                  | Description                     |
/// |----------------------|---------------------------------|
/// | `JCAMPDX`            | Version of the JCAMP-DX format. |
/// | `DATA_TYPE`          | Type of data in the file.       |
/// | `DATA_CLASS`         | Way the data is stored.         |
/// | `.OBSERVE FREQUENCY` | Frequency of the spectrometer.  |
///
/// The following additional information will only be extracted if present:
///
/// | Key                  | Description                     |
/// |----------------------|---------------------------------|
/// | `.OBSERVE NUCLEUS`   | Nucleus being observed.         |
/// | `.SOLVENT NAME`      | Solvent used in the experiment. |
/// | `.SOLVENT REFERENCE` | Reference for the solvent.      |
/// | `.SHIFT REFERENCE`   | Chemical shift reference.       |
#[derive(Debug)]
pub enum JcampDx {}

#[derive(Debug)]
enum DataType {
    Spectrum,
}

#[derive(Debug)]
enum Format {
    XYData,
    NTuples,
}

#[derive(Debug)]
enum XUnits {
    Hz,
    Ppm,
}

#[derive(Debug)]
struct Header {
    data_type: DataType,
    format: Format,
    frequency: f64,
    nucleus: Nucleus,
    reference_compound: Option<ReferenceCompound>,
}

static HEADER_RE: LazyLock<[Regex; 11]> = LazyLock::new(|| {
    [
        Regex::new(r"(##JCAMPDX=\s*)(?P<version>\d+(\.\d+)?)").unwrap(),
        Regex::new(r"(##DATA(\s|_)TYPE=\s*)(?P<type>\w+\s\w+)").unwrap(),
        Regex::new(r"(##DATA(\s|_)CLASS=\s*)(?P<format>\w+(\s\w+)?)").unwrap(),
        Regex::new(r"(##\.OBSERVE(\s|_)FREQUENCY=\s*)(?P<frequency>\d+(\.\d+)?)").unwrap(),
        Regex::new(r"(##\.OBSERVE(\s|_)NUCLEUS=\s*)(?P<nucleus>\^\w+)").unwrap(),
        Regex::new(r"(##\.SOLVENT(\s|_)NAME=\s*)(?P<name>.*)").unwrap(),
        Regex::new(r"(##\.SOLVENT(\s|_)REFERENCE=\s*)(?P<shift>\d+(\.\d+))?").unwrap(),
        Regex::new(r"(##\.SHIFT(\s|_)REFERENCE=\s*)(?P<method>[^,]*)").unwrap(),
        Regex::new(r"(##\.SHIFT(\s|_)REFERENCE=[^,]*,\s*)(?P<name>[^,]*)").unwrap(),
        Regex::new(r"(##\.SHIFT(\s|_)REFERENCE=[^,]*,[^,]*,\s*)(?P<index>\d+)").unwrap(),
        Regex::new(r"(##\.SHIFT(\s|_)REFERENCE=[^,]*,[^,]*,[^,]*,\s*)(?P<shift>\d+(\.\d+)?)")
            .unwrap(),
    ]
});
static HEADER_KEYS: LazyLock<[&str; 11]> = LazyLock::new(|| {
    [
        "JCAMPDX",
        "DATA_TYPE",
        "DATA_CLASS",
        ".OBSERVE FREQUENCY",
        ".OBSERVE NUCLEUS",
        ".SOLVENT NAME",
        ".SOLVENT REFERENCE",
        ".SHIFT REFERENCE [METHOD]",
        ".SHIFT REFERENCE [COMPOUND]",
        ".SHIFT REFERENCE [INDEX]",
        ".SHIFT REFERENCE [SHIFT]",
    ]
});

#[derive(Debug)]
struct DataBlock {
    x_units: XUnits,
    factor: f64,
    first: f64,
    last: f64,
    data_size: usize,
    data: String,
}

static XY_DATA_RE: LazyLock<[Regex; 6]> = LazyLock::new(|| {
    [
        Regex::new(r"(##XUNITS=\s*)(?P<xunits>\w+)").unwrap(),
        Regex::new(r"(##YFACTOR=\s*)(?P<factor>\d+(\.\d+)?)").unwrap(),
        Regex::new(r"(##FIRSTX=\s*)(?P<first>\d+(\.\d+)?)").unwrap(),
        Regex::new(r"(##LASTX=\s*)(?P<last>\d+(\.\d+)?)").unwrap(),
        Regex::new(r"(##NPOINTS=\s*)(?P<data_size>\d+(\.\d+)?)").unwrap(),
        Regex::new(r"(##XYDATA=\(X\+\+\(Y\.\.Y\)\)(.*)?)(?P<data>[^#$]*)").unwrap(),
    ]
});
static XY_DATA_KEYS: LazyLock<[&str; 6]> =
    LazyLock::new(|| ["XUNITS", "YFACTOR", "FIRSTX", "LASTX", "NPOINTS", "XYDATA"]);

static N_TUPLES_RE: LazyLock<[Regex; 7]> = LazyLock::new(|| {
    [
        Regex::new(r"(##SYMBOL=\s*)(?P<symbols>.*)(\r\n|\n|\r)").unwrap(),
        Regex::new(r"(##VAR(\s*|_)DIM=\s*)(?P<data_sizes>.*)(\r\n|\n|\r)").unwrap(),
        Regex::new(r"(##UNITS=\s*)(?P<units>.*)(\r\n|\n|\r)").unwrap(),
        Regex::new(r"(##FIRST=\s*)(?P<first>.*)(\r\n|\n|\r)").unwrap(),
        Regex::new(r"(##LAST=\s*)(?P<last>.*)(\r\n|\n|\r)").unwrap(),
        Regex::new(r"(##FACTOR=\s*)(?P<factor>.*)(\r\n|\n|\r)").unwrap(),
        Regex::new(r"(##DATA(\s|_)TABLE=\s*\(X\+\+\(([RY])\.\.[RY]\)\)(.*)?)(?P<data>[^#$]*)")
            .unwrap(),
    ]
});
static N_TUPLES_KEYS: LazyLock<[&str; 7]> = LazyLock::new(|| {
    [
        "SYMBOL",
        "VAR DIM",
        "UNITS",
        "FIRST",
        "LAST",
        "FACTOR",
        "DATA TABLE",
    ]
});

static ENCODING: LazyLock<[Regex; 6]> = LazyLock::new(|| {
    [
        Regex::new(r"(?P<asdf>[@%A-Za-z+-])").unwrap(), // ASDF
        Regex::new(r"(?P<pac>[+-]\d)").unwrap(),        // PAC
        Regex::new(r"(?P<sqz>[@A-Ia-i])").unwrap(),     // SQZ
        Regex::new(r"\s+([%J-Rj-r]\d*\s*(?P<new>\r\n|\n|\r)(?P<next>\d+))").unwrap(), // Checkpoints
        Regex::new(r"\s+(?P<val>[+-]*\d+)\s+(?P<dif>[%J-Rj-r]\d*)").unwrap(), // DIF
        Regex::new(r"\s+(?P<val>[+-]*\d+)\s+(?P<dup>[S-Zs]\d*)").unwrap(), // DUP
    ]
});

impl JcampDx {
    /// Reads the spectrum from a JCAMP-DX file.
    pub fn read_spectrum<P: AsRef<Path>>(
        path: P,
        signal_boundaries: (f64, f64),
    ) -> Result<Spectrum> {
        let path = path.as_ref();
        let dx = read_to_string(path)?;
        let header = Self::read_header(&dx, path)?;
        let block = match header.format {
            Format::XYData => Self::read_xydata(&dx, path)?,
            Format::NTuples => Self::read_ntuples(&dx, path)?,
        };
        let conversion = match block.x_units {
            XUnits::Hz => 1.0 / header.frequency,
            XUnits::Ppm => 1.0,
        };
        let step = (block.last - block.first) * conversion / (block.data_size as f64 - 1.0);
        let offset = match header.reference_compound {
            Some(reference) => match reference.index() {
                Some(index) => reference.chemical_shift() - index as f64 * step,
                None => block.first * conversion,
            },
            None => block.first * conversion,
        };
        let chemical_shifts = (0..block.data_size)
            .map(|i| offset + (i as f64) * step)
            .collect();
        let intensities = match ENCODING[0].is_match(block.data.as_str()) {
            true => Self::decode_asdf(&block.data, block.factor, path)?,
            false => Self::decode_affn(&block.data, block.factor, path)?,
        };
        let spectrum = Spectrum::new(chemical_shifts, intensities, signal_boundaries)?;

        Ok(spectrum)
    }

    fn read_header<P: AsRef<Path>>(dx: &str, path: P) -> Result<Header> {
        let re = &*HEADER_RE;
        let keys = &*HEADER_KEYS;

        match extract_capture::<f64, _>(&re[0], "version", dx, &path, keys[0])?.trunc() {
            5.0 | 6.0 => (),
            _ => return Err(Error::new(Kind::UnsupportedJcampDxFile).into()),
        };
        let data_type = match extract_capture::<String, _>(&re[1], "type", dx, &path, keys[1])?
            .to_uppercase()
            .as_str()
        {
            "NMR SPECTRUM" => DataType::Spectrum,
            _ => return Err(Error::new(Kind::UnsupportedJcampDxFile).into()),
        };
        let format = match extract_capture::<String, _>(&re[2], "format", dx, &path, keys[2])?
            .to_uppercase()
            .as_str()
        {
            "XYDATA" => Format::XYData,
            "NTUPLES" => Format::NTuples,
            _ => return Err(Error::new(Kind::UnsupportedJcampDxFile).into()),
        };
        let frequency = extract_capture::<f64, _>(&re[3], "frequency", dx, &path, keys[3])?;
        let nucleus = match extract_capture::<String, _>(&re[4], "nucleus", dx, &path, keys[4])?
            .to_uppercase()
            .as_str()
        {
            "^1H" => Nucleus::Hydrogen1,
            "^13C" => Nucleus::Carbon13,
            "^15N" => Nucleus::Nitrogen15,
            "^19F" => Nucleus::Fluorine19,
            "^29SI" => Nucleus::Silicon29,
            "^31P" => Nucleus::Phosphorus31,
            name => Nucleus::Other(name.to_string()),
        };
        let reference_compound = {
            let method = extract_capture::<String, _>(&re[7], "method", dx, &path, keys[7]).ok();
            let name = extract_capture::<String, _>(&re[8], "name", dx, &path, keys[8]).ok();
            let index = extract_capture(&re[9], "index", dx, &path, keys[9]).ok();
            let shift = extract_capture(&re[10], "shift", dx, &path, keys[10]).ok();

            if let (Some(shift), Some(index)) = (shift, index) {
                let referencing_method = match method.as_deref() {
                    Some("INTERNAL") => Some(ReferencingMethod::Internal),
                    Some("EXTERNAL") => Some(ReferencingMethod::External),
                    _ => None,
                };

                Some(ReferenceCompound::new(
                    shift,
                    name,
                    Some(index),
                    referencing_method,
                ))
            } else {
                let name = extract_capture::<String, _>(&re[5], "name", dx, &path, keys[5]).ok();
                let shift = extract_capture(&re[6], "shift", dx, &path, keys[6]).ok();

                if let Some(shift) = shift {
                    Some(ReferenceCompound::new(shift, name, Some(0), None))
                } else {
                    name.map(|name| ReferenceCompound::new(0.0, Some(name), None, None))
                }
            }
        };

        Ok(Header {
            data_type,
            format,
            frequency,
            nucleus,
            reference_compound,
        })
    }

    fn read_xydata<P: AsRef<Path>>(dx: &str, path: P) -> Result<DataBlock> {
        let re = &*XY_DATA_RE;
        let keys = &*XY_DATA_KEYS;

        let x_units = match extract_capture::<String, _>(&re[0], "xunits", dx, &path, keys[0])?
            .to_uppercase()
            .as_str()
        {
            "HZ" => XUnits::Hz,
            "PPM" => XUnits::Ppm,
            _ => return Err(Error::new(Kind::UnsupportedJcampDxFile).into()),
        };
        let factor = extract_capture(&re[1], "factor", dx, &path, keys[1])?;
        let first = extract_capture(&re[2], "first", dx, &path, keys[2])?;
        let last = extract_capture(&re[3], "last", dx, &path, keys[3])?;
        let data_size = extract_capture(&re[4], "data_size", dx, &path, keys[4])?;
        let data = extract_capture::<String, _>(&re[5], "data", dx, &path, keys[5])?
            .as_str()
            .trim()
            .to_string();

        if data.is_empty() {
            return Err(Error::new(Kind::MissingData {
                path: path.as_ref().to_path_buf(),
            })
            .into());
        }

        Ok(DataBlock {
            x_units,
            factor,
            first,
            last,
            data_size,
            data,
        })
    }

    fn read_ntuples<P: AsRef<Path>>(dx: &str, path: P) -> Result<DataBlock> {
        let re = &*N_TUPLES_RE;
        let keys = &*N_TUPLES_KEYS;

        let symbols = extract_capture::<String, _>(&re[0], "symbols", dx, &path, keys[0])?
            .split(",")
            .map(|symbol| symbol.trim().to_string())
            .collect::<Vec<_>>();
        let x_column = symbols
            .iter()
            .position(|symbol| symbol.to_uppercase() == "X")
            .ok_or_else(|| {
                Error::new(Kind::MissingMetadata {
                    key: keys[0].to_string(),
                    path: path.as_ref().to_path_buf(),
                })
            })?;
        let r_column = symbols
            .iter()
            .position(|symbol| symbol.to_uppercase() == "R")
            .ok_or_else(|| {
                Error::new(Kind::MissingMetadata {
                    key: keys[0].to_string(),
                    path: path.as_ref().to_path_buf(),
                })
            })?;

        let data_size = extract_row::<usize, _>(&re[1], "data_sizes", dx, &path, keys[1])?
            .get(x_column)
            .copied()
            .ok_or_else(|| {
                Error::new(Kind::MalformedMetadata {
                    key: keys[1].to_string(),
                    path: path.as_ref().to_path_buf(),
                    details: "Could not find X column".to_string(),
                })
            })?;
        let x_units = match extract_row::<String, _>(&re[2], "units", dx, &path, keys[2])?
            .get(x_column)
            .ok_or_else(|| {
                Error::new(Kind::MalformedMetadata {
                    key: keys[2].to_string(),
                    path: path.as_ref().to_path_buf(),
                    details: "Could not find X column".to_string(),
                })
            })?
            .to_uppercase()
            .as_str()
        {
            "HZ" => XUnits::Hz,
            "PPM" => XUnits::Ppm,
            unit => {
                return Err(Error::new(Kind::MalformedMetadata {
                    key: keys[2].to_string(),
                    path: path.as_ref().to_path_buf(),
                    details: format!("Unsupported x unit: {}", unit),
                })
                .into());
            }
        };
        let first = extract_row::<f64, _>(&re[3], "first", dx, &path, keys[3])?
            .get(x_column)
            .copied()
            .ok_or_else(|| {
                Error::new(Kind::MalformedMetadata {
                    key: keys[3].to_string(),
                    path: path.as_ref().to_path_buf(),
                    details: "Could not find X column".to_string(),
                })
            })?;
        let last = extract_row::<f64, _>(&re[4], "last", dx, &path, keys[4])?
            .get(x_column)
            .copied()
            .ok_or_else(|| {
                Error::new(Kind::MalformedMetadata {
                    key: keys[4].to_string(),
                    path: path.as_ref().to_path_buf(),
                    details: "Could not find X column".to_string(),
                })
            })?;
        let factor = extract_row::<f64, _>(&re[5], "factor", dx, &path, keys[5])?
            .get(r_column)
            .copied()
            .ok_or_else(|| {
                Error::new(Kind::MalformedMetadata {
                    key: keys[5].to_string(),
                    path: path.as_ref().to_path_buf(),
                    details: "Could not find R column".to_string(),
                })
            })?;
        let data = extract_capture::<String, _>(&re[6], "data", dx, &path, keys[6])?
            .as_str()
            .trim()
            .to_string();

        if data.is_empty() {
            return Err(Error::new(Kind::MissingData {
                path: path.as_ref().to_path_buf(),
            })
            .into());
        }

        Ok(DataBlock {
            x_units,
            factor,
            first,
            last,
            data_size,
            data,
        })
    }

    fn decode_affn<P: AsRef<Path>>(data: &str, factor: f64, path: P) -> Result<Vec<f64>> {
        let intensities = data
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .skip(1)
                    .map(|value| {
                        value.parse::<f64>().map_err(|error| {
                            Error::new(Kind::MalformedData {
                                path: path.as_ref().to_path_buf(),
                                details: format!("{} ({})", value, error),
                            })
                            .into()
                        })
                    })
                    .collect::<Result<Vec<f64>>>()
            })
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .flatten()
            .map(|intensity| intensity * factor)
            .collect();

        Ok(intensities)
    }

    fn decode_asdf<P: AsRef<Path>>(data: &str, factor: f64, path: P) -> Result<Vec<f64>> {
        let re = &*ENCODING;

        let data = re[0].replace_all(data, " $asdf");
        let data = re[1].replace_all(&data, " $pac");
        let data = re[2].replace_all(&data, |captures: &Captures| {
            Self::undo_sqz(captures.name("sqz").unwrap().as_str())
        });
        let mut data = re[3].replace_all(&data, "$new$next").to_string();
        loop {
            let tmp_data_dif = re[4].replace_all(&data, |captures: &Captures| {
                let value = captures.name("val").unwrap().as_str();
                let encoded = captures.name("dif").unwrap().as_str();

                Self::undo_dif(value, encoded)
            });
            let tmp_data_dup = re[5].replace_all(&tmp_data_dif, |captures: &Captures| {
                let value = captures.name("val").unwrap().as_str();
                let encoded = captures.name("dup").unwrap().as_str();

                Self::undo_dup(value, encoded)
            });
            data = tmp_data_dup.to_string();

            if !re[4].is_match(&data) && !re[5].is_match(&data) {
                break;
            }
        }

        Self::decode_affn(&data, factor, path)
    }

    fn undo_sqz(character: &str) -> String {
        match character {
            "@" => "0",
            "A" => "1",
            "B" => "2",
            "C" => "3",
            "D" => "4",
            "E" => "5",
            "F" => "6",
            "G" => "7",
            "H" => "8",
            "I" => "9",
            "a" => "-1",
            "b" => "-2",
            "c" => "-3",
            "d" => "-4",
            "e" => "-5",
            "f" => "-6",
            "g" => "-7",
            "h" => "-8",
            "i" => "-9",
            _ => unreachable!("Invalid SQZ character: {}", character),
        }
        .to_string()
    }

    fn undo_dup(value: &str, encoded: &str) -> String {
        let mut decoded = match encoded.chars().next().unwrap() {
            'S' => "1",
            'T' => "2",
            'U' => "3",
            'V' => "4",
            'W' => "5",
            'X' => "6",
            'Y' => "7",
            'Z' => "8",
            's' => "9",
            _ => unreachable!("Invalid DUP character: {}", encoded),
        }
        .to_string();
        decoded.extend(encoded.chars().skip(1));
        let duplicates = decoded.parse::<usize>().unwrap();

        format!(" {}", value).repeat(duplicates)
    }

    fn undo_dif(value: &str, encoded: &str) -> String {
        let mut decoded = match encoded.chars().next().unwrap() {
            '%' => "0",
            'J' => "1",
            'K' => "2",
            'L' => "3",
            'M' => "4",
            'N' => "5",
            'O' => "6",
            'P' => "7",
            'Q' => "8",
            'R' => "9",
            'j' => "-1",
            'k' => "-2",
            'l' => "-3",
            'm' => "-4",
            'n' => "-5",
            'o' => "-6",
            'p' => "-7",
            'q' => "-8",
            'r' => "-9",
            _ => unreachable!("Invalid DIF character: {}", encoded),
        }
        .to_string();
        decoded.extend(encoded.chars().skip(1));
        let value = value.parse::<i64>().unwrap();
        let difference = decoded.parse::<i64>().unwrap();

        format!(" {} {}", value, value + difference)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::assert_approx_eq;

    #[test]
    fn read_affn_spectrum() {
        let path = "../data/jcamp-dx/BRUKAFFN.dx";
        let spectrum = JcampDx::read_spectrum(path, (20.0, 220.0)).unwrap();
    }

    #[test]
    fn read_pac_spectrum() {
        let path = "../data/jcamp-dx/BRUKPAC.dx";
        let spectrum = JcampDx::read_spectrum(path, (20.0, 220.0)).unwrap();
    }

    #[test]
    fn read_sqz_spectrum() {
        let path = "../data/jcamp-dx/BRUKSQZ.dx";
        let spectrum = JcampDx::read_spectrum(path, (20.0, 220.0)).unwrap();
    }

    #[test]
    fn read_dif_dup_spectrum() {
        let path = "../data/jcamp-dx/BRUKDIF.dx";
        let spectrum = JcampDx::read_spectrum(path, (20.0, 220.0)).unwrap();
    }

    #[test]
    fn read_ntuples_spectrum() {
        let path = "../data/jcamp-dx/BRUKNTUP.dx";
        let spectrum = JcampDx::read_spectrum(path, (20.0, 220.0)).unwrap();
    }

    #[test]
    fn read_header() {
        let path = "../data/jcamp-dx/BRUKNTUP.dx";
        let dx = read_to_string(path).unwrap();
        let header = JcampDx::read_header(&dx, path).unwrap();
        match header.data_type {
            DataType::Spectrum => (),
        };
        match header.format {
            Format::XYData => panic!("Expected NTuples"),
            Format::NTuples => (),
        };
        assert_approx_eq!(f64, header.frequency, 100.4);
        match header.nucleus {
            Nucleus::Carbon13 => (),
            _ => panic!("Expected Carbon13"),
        };
        if header.reference_compound.is_some() {
            panic!("Expected None");
        }
    }

    #[test]
    fn read_xydata() {
        let path = "../data/jcamp-dx/BRUKDIF.dx";
        let dx = read_to_string(path).unwrap();
        let xy_data = JcampDx::read_xydata(&dx, path).unwrap();
        match xy_data.x_units {
            XUnits::Hz => (),
            XUnits::Ppm => panic!("Expected Hz"),
        }
        assert_approx_eq!(f64, xy_data.factor, 1.0);
        assert_approx_eq!(f64, xy_data.first, 24038.5);
        assert_approx_eq!(f64, xy_data.last, 0.0);
        assert_eq!(xy_data.data_size, 16384);
    }

    #[test]
    fn read_ntuples() {
        let path = "../data/jcamp-dx/BRUKNTUP.dx";
        let dx = read_to_string(path).unwrap();
        let n_tuples = JcampDx::read_ntuples(&dx, path).unwrap();
        match n_tuples.x_units {
            XUnits::Hz => (),
            XUnits::Ppm => panic!("Expected Hz"),
        }
        assert_approx_eq!(f64, n_tuples.factor, 1.0);
        assert_approx_eq!(f64, n_tuples.first, 24038.5);
        assert_approx_eq!(f64, n_tuples.last, 0.0);
        assert_eq!(n_tuples.data_size, 16384);
    }

    #[test]
    fn decode_affn() {
        let data = "\
            19        482       -763        215       -632\n\
            15       -924        357       -678        841\n\
            11        512       -194        321       -467\n\
            7        -689        278        278        732\n\
            3         835       -619        247       -193";
        let expected = [
            482.0, -763.0, 215.0, -632.0, -924.0, 357.0, -678.0, 841.0, 512.0, -194.0, 321.0,
            -467.0, -689.0, 278.0, 278.0, 732.0, 835.0, -619.0, 247.0, -193.0,
        ];
        let decoded = JcampDx::decode_affn(data, 1.0, "decode_affn_test").unwrap();
        decoded
            .into_iter()
            .zip(expected)
            .for_each(|(decoded, expected)| {
                assert_approx_eq!(f64, decoded, expected);
            });
    }

    #[test]
    fn decode_pac() {
        let data = "\
            19 +482-763+215-632-924+357-678+841+512-194\n\
            9  +321-467-689+278+278+732+835-619+247-193";
        let expected = [
            482.0, -763.0, 215.0, -632.0, -924.0, 357.0, -678.0, 841.0, 512.0, -194.0, 321.0,
            -467.0, -689.0, 278.0, 278.0, 732.0, 835.0, -619.0, 247.0, -193.0,
        ];
        let decoded = JcampDx::decode_asdf(data, 1.0, "decode_pac_test").unwrap();
        decoded
            .into_iter()
            .zip(expected)
            .for_each(|(decoded, expected)| {
                assert_approx_eq!(f64, decoded, expected);
            });
    }

    #[test]
    fn decode_sqz() {
        let data = "\
            19 D82g63B15f32i24C57f78H41E12a94\n\
            9  C21d67f89B78B78G32H35f19B47a93";
        let expected = [
            482.0, -763.0, 215.0, -632.0, -924.0, 357.0, -678.0, 841.0, 512.0, -194.0, 321.0,
            -467.0, -689.0, 278.0, 278.0, 732.0, 835.0, -619.0, 247.0, -193.0,
        ];
        let decoded = JcampDx::decode_asdf(data, 1.0, "decode_sqz_test").unwrap();
        decoded
            .into_iter()
            .zip(expected)
            .for_each(|(decoded, expected)| {
                assert_approx_eq!(f64, decoded, expected);
            });
    }

    #[test]
    fn decode_dif_dup() {
        let data = "\
            19 D82j245R78q47k92J281j035J519l29p06\n\
            10 a94N15p88k22R67TM54J03j454Q66m40";
        let expected = [
            482.0, -763.0, 215.0, -632.0, -924.0, 357.0, -678.0, 841.0, 512.0, -194.0, 321.0,
            -467.0, -689.0, 278.0, 278.0, 732.0, 835.0, -619.0, 247.0, -193.0,
        ];
        let decoded = JcampDx::decode_asdf(data, 1.0, "decode_sqz_test").unwrap();
        decoded
            .into_iter()
            .zip(expected)
            .for_each(|(decoded, expected)| {
                assert_approx_eq!(f64, decoded, expected);
            });
    }
}
