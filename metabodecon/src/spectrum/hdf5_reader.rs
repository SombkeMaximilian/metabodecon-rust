use crate::spectrum::Spectrum;
use std::path::Path;

#[derive(Debug, Clone, Copy)]
pub struct Hdf5Reader<P: AsRef<Path>> {
    path: P,
}

impl<P: AsRef<Path>> Hdf5Reader<P> {
    pub fn new(path: P) -> Self {
        Hdf5Reader { path }
    }

    pub fn path(&self) -> &Path {
        self.path.as_ref()
    }

    pub fn set_path(&mut self, path: P) {
        self.path = path;
    }

    pub fn read_spectrum(&self, dataset: &str) -> hdf5::Result<Spectrum> {
        let file = hdf5::File::open(self.path.as_ref())?;
        Self::read_from_file(&file, dataset)
    }

    pub fn read_spectra(&self) -> hdf5::Result<Vec<Spectrum>> {
        let file = hdf5::File::open(self.path.as_ref())?;
        let datasets: Vec<String> = file
                .member_names()?
                .into_iter()
                .collect();
        let spectra = datasets
            .into_iter()
            .map(|dataset| Self::read_from_file(&file, &dataset))
            .collect::<hdf5::Result<Vec<Spectrum>>>()?;

        Ok(spectra)
    }

    fn read_from_file(file: &hdf5::File, dataset: &str) -> hdf5::Result<Spectrum> {
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

        Ok(Spectrum::new(
            chemical_shifts,
            intensities,
            (signal_boundaries[0], signal_boundaries[1]),
            (water_boundaries[0], water_boundaries[1]),
        ))
    }
}
