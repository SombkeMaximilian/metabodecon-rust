use crate::error::Result;
use crate::spectrum::Spectrum;
use std::path::Path;

#[derive(Default)]
pub struct Hdf5Reader;

impl Hdf5Reader {
    pub fn new() -> Self {
        Self
    }

    pub fn read_spectrum<P: AsRef<Path>>(&self, path: P, dataset: &str) -> Result<Spectrum> {
        let file = hdf5::File::open(path.as_ref())?;
        Self::read_from_file(&file, dataset)
    }

    pub fn read_spectra<P: AsRef<Path>>(&self, path: P) -> Result<Vec<Spectrum>> {
        let file = hdf5::File::open(path.as_ref())?;
        let datasets: Vec<String> = file.member_names()?.into_iter().collect();
        let spectra = datasets
            .into_iter()
            .map(|dataset| Self::read_from_file(&file, &dataset))
            .collect::<Result<Vec<Spectrum>>>()?;

        Ok(spectra)
    }

    fn read_from_file(file: &hdf5::File, dataset: &str) -> Result<Spectrum> {
        let spectrum_group = file.group(dataset)?.group("spectrum")?;
        let data_group = spectrum_group.group("data")?;
        let meta_group = spectrum_group.group("meta")?;

        let chemical_shifts = data_group
            .dataset("chemical_shifts")?
            .read_1d()?
            .to_vec();
        let intensities = data_group
            .dataset("signal_intensities")?
            .read_1d()?
            .to_vec();
        let signal_boundaries = meta_group
            .dataset("signal_boundaries")?
            .read_1d()?
            .to_vec();
        let water_boundaries = meta_group
            .dataset("water_boundaries")?
            .read_1d()?
            .to_vec();
        let spectrum = Spectrum::new(
            chemical_shifts,
            intensities,
            (signal_boundaries[0], signal_boundaries[1]),
            (water_boundaries[0], water_boundaries[1]),
        )?;

        Ok(spectrum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn read_spectrum() {
        let path = "../data/hdf5/sim.h5";
        let dataset = "sim_01";
        let reader = Hdf5Reader::new();
        let spectrum = reader.read_spectrum(path, dataset).unwrap();
        check_sim_spectrum!(spectrum);
    }

    #[test]
    fn read_spectra() {
        let path = "../data/hdf5/sim.h5";
        let reader = Hdf5Reader::new();
        let spectra = reader.read_spectra(path).unwrap();
        assert_eq!(spectra.len(), 16);
        spectra.iter().for_each(|spectrum| {
            check_sim_spectrum!(spectrum);
        })
    }
}
