use crate::Result;
use crate::spectrum::Spectrum;
use std::path::Path;

/// Interface for reading 1D NMR spectra from HDF5 files.
///
/// # Format
///
/// The HDF5 files are expected to have the following structure:
///
/// ```text
/// file.h5
/// └── dataset_01
///     └── spectrum
///         ├── data
///         │   ├── chemical_shifts
///         │   └── signal_intensities
///         └── meta
///             └── signal_boundaries
/// ```
///
/// `file.h5` and `dataset_01` are the file and dataset names, respectively.
/// There can be any number of datasets in one file and their names do not need
/// to follow a pattern. For example, blood_01 and sim_01 would be valid dataset
/// names in the same file. The `spectrum` group contains the `data` and `meta`
/// groups, which contain the raw data and metadata, respectively.
///
/// # Example: Reading a Spectrum
///
/// ```
/// use metabodecon::spectrum::Hdf5;
///
/// # fn main() -> metabodecon::Result<()> {
/// let path = "path/to/file.h5";
/// # let path = "../data/hdf5/blood.h5";
/// let dataset = "dataset_01";
/// # let dataset = "blood_01";
///
/// // Read a single spectrum from the HDF5 file.
/// let spectrum = Hdf5::read_spectrum(path, dataset)?;
/// # Ok(())
/// # }
/// ```
///
/// # Example: Reading Multiple Spectra
///
/// ```
/// use metabodecon::spectrum::Hdf5;
///
/// # fn main() -> metabodecon::Result<()> {
/// let path = "path/to/file.h5";
/// # let path = "../data/hdf5/blood.h5";
///
/// // Read all spectra from the HDF5 file.
/// let spectra = Hdf5::read_spectra(path)?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub enum Hdf5 {}

impl Hdf5 {
    /// Reads the spectrum in the provided dataset from an HDF5 file.
    ///
    /// ```text
    /// file.h5 ← the path needs to point to this file
    /// └── dataset_01
    ///     └── spectrum
    ///         ├── data
    ///         │   ├── chemical_shifts
    ///         │   └── signal_intensities
    ///         └── meta
    ///             └── signal_boundaries
    /// ```
    ///
    /// # Errors
    ///
    /// The read data is checked for validity to ensure that the `Spectrum` is
    /// well-formed and in a consistent state. The following conditions are
    /// checked:
    /// - The chemical shifts and intensities are not empty.
    /// - The lengths of the chemical shifts and intensities match.
    /// - All chemical shift values are finite and uniformly spaced.
    /// - All intensity values are finite.
    /// - The signal region boundaries are within the range of the chemical
    ///   shifts.
    ///
    /// Additionally, if any errors from the [hdf5 crate] occur, an error
    /// variant containing the original error is returned.
    ///
    /// [hdf5 crate]: https://docs.rs/crate/hdf5/latest
    ///
    /// # Example
    ///
    /// ```
    /// use metabodecon::spectrum::Hdf5;
    ///
    /// # fn main() -> metabodecon::Result<()> {
    /// let path = "path/to/file.h5";
    /// # let path = "../data/hdf5/blood.h5";
    /// let dataset = "dataset_01";
    /// # let dataset = "blood_01";
    ///
    /// // Read a single spectrum from the HDF5 file.
    /// let spectrum = Hdf5::read_spectrum(path, dataset)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn read_spectrum<P: AsRef<Path>>(path: P, dataset: &str) -> Result<Spectrum> {
        let file = hdf5::File::open(path.as_ref())?;

        Self::read_from_file(&file, dataset)
    }

    /// Reads all spectra from an HDF5 file.
    ///
    /// ```text
    /// file.h5 ← the path needs to point to this file
    /// ├── dataset_01
    /// │   └── spectrum
    /// │       ├── data
    /// │       │   ├── chemical_shifts
    /// │       │   └── signal_intensities
    /// │       └── meta
    /// │           └── signal_boundaries
    /// ├── dataset_02
    /// │   └── spectrum
    /// │       ├── data
    /// │       │   ├── chemical_shifts
    /// │       │   └── signal_intensities
    /// │       └── meta
    /// │           └── signal_boundaries
    /// ·
    /// ·
    /// ·
    /// ```
    ///
    /// # Errors
    ///
    /// The read data is checked for validity to ensure that the `Spectrum` is
    /// well-formed and in a consistent state. The following conditions are
    /// checked:
    /// - The chemical shifts and intensities are not empty.
    /// - The lengths of the chemical shifts and intensities match.
    /// - All chemical shift values are finite and uniformly spaced.
    /// - All intensity values are finite.
    /// - The signal region boundaries are within the range of the chemical
    ///   shifts.
    ///
    /// Additionally, if any errors from the [hdf5 crate] occur, an error
    /// variant containing the original error is returned.
    ///
    /// [hdf5 crate]: https://docs.rs/crate/hdf5/latest
    ///
    /// # Example
    ///
    /// ```
    /// use metabodecon::spectrum::Hdf5;
    ///
    /// # fn main() -> metabodecon::Result<()> {
    /// let path = "path/to/file.h5";
    /// # let path = "../data/hdf5/blood.h5";
    ///
    /// // Read all spectra from the HDF5 file.
    /// let spectra = Hdf5::read_spectra(path)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn read_spectra<P: AsRef<Path>>(path: P) -> Result<Vec<Spectrum>> {
        let file = hdf5::File::open(path.as_ref())?;
        let datasets = file.member_names()?;
        let spectra = datasets
            .into_iter()
            .map(|dataset| Self::read_from_file(&file, &dataset))
            .collect::<Result<Vec<Spectrum>>>()?;

        Ok(spectra)
    }

    /// Writes the given spectrum to the HDF5 file at the specified path.
    ///
    /// If the file does not exist, it will be created. If it already exists,
    /// the spectrum will be appended as `filename_length+1`. For consistency,
    /// the dataset names will match the filename with a numerical suffix,
    /// starting from 1 and using 0-padding to ensure that the datasets are
    /// sorted correctly. This is not required to be compatible with the
    /// [`Hdf5::read_spectrum`] and [`Hdf5::read_spectra`] functions, however,
    /// and any dataset names can be used when creating the HDF5 files with
    /// another tool.
    ///
    /// ```text
    /// filename.h5 ← the path needs to point to this file
    /// ├── filename_01 ← existing dataset
    /// │   └── spectrum
    /// │       ├── data
    /// │       │   ├── chemical_shifts
    /// │       │   └── signal_intensities
    /// │       └── meta
    /// │           └── signal_boundaries
    /// ├── filename_02 ← existing dataset
    /// │   └── spectrum
    /// │       ├── data
    /// │       │   ├── chemical_shifts
    /// │       │   └── signal_intensities
    /// │       └── meta
    /// │           └── signal_boundaries
    /// └── filename_03 ← the new dataset gets appended here
    /// ```
    ///
    /// # Errors
    ///
    /// If any errors from the [hdf5 crate] occur, an error variant containing
    /// the original error is returned.
    ///
    /// [hdf5 crate]: https://docs.rs/crate/hdf5/latest
    ///
    /// # Example
    ///
    /// ```no_run
    /// use metabodecon::spectrum::{Bruker, Hdf5, Spectrum};
    ///
    /// # fn main() -> metabodecon::Result<()> {
    /// let source_path = "path/to/root";
    /// let target_path = "path/to/file.h5";
    ///
    /// // Read all spectra from Bruker TopSpin format directories within the root.
    /// let spectra = Bruker::read_spectra(source_path, 10, 10, (-2.2, 11.8))?;
    ///
    /// // Create a new HDF5 file and write the spectra to it.
    /// Hdf5::write_spectra(target_path, &spectra)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn write_spectrum<P: AsRef<Path>, S: AsRef<Spectrum>>(path: P, spectrum: S) -> Result<()> {
        let file = if path.as_ref().exists() {
            hdf5::File::open_rw(path.as_ref())?
        } else {
            hdf5::File::create(path.as_ref())?
        };
        let basename = path
            .as_ref()
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap();
        let next = file.member_names()?.len() + 1;
        let dataset = format!("{}_{}", basename, next);
        Self::write_to_file(&file, dataset.as_str(), spectrum.as_ref())?;

        Ok(())
    }

    /// Creates a new HDF5 file at the specified path and writes the given
    /// spectra to it.
    ///
    /// This will always create a new file, overwriting any existing file at the
    /// path, as it is meant for batch processing of spectra. The dataset names
    /// will match the filename with a numerical suffix, starting from 1,
    /// and using 0-padding to ensure that the datasets are sorted
    /// correctly. This is not required, however, and any dataset names can
    /// be used if you need to create the HDF5 files with another tool,
    /// while still being compatible with the [`Hdf5::read_spectrum`] and
    /// [`Hdf5::read_spectra`] functions.
    ///
    /// ```text
    /// file.h5
    /// ├── dataset_01
    /// │   └── spectrum
    /// │       ├── data
    /// │       │   ├── chemical_shifts
    /// │       │   └── signal_intensities
    /// │       └── meta
    /// │           └── signal_boundaries
    /// ├── dataset_02
    /// │   └── spectrum
    /// │       ├── data
    /// │       │   ├── chemical_shifts
    /// │       │   └── signal_intensities
    /// │       └── meta
    /// │           └── signal_boundaries
    /// .
    /// .
    /// .
    /// ```
    ///
    /// # Errors
    ///
    /// If any errors from the [hdf5 crate] occur, an error variant containing
    /// the original error is returned.
    ///
    /// [hdf5 crate]: https://docs.rs/crate/hdf5/latest
    ///
    /// # Example
    ///
    /// ```no_run
    /// use metabodecon::spectrum::{Bruker, Hdf5, Spectrum};
    ///
    /// # fn main() -> metabodecon::Result<()> {
    /// let source_path = "path/to/spectrum";
    /// let target_path = "path/to/file.h5";
    ///
    /// // Read a single spectrum from a Bruker TopSpin format directory.
    /// let spectrum = Bruker::read_spectrum(source_path, 10, 10, (-2.2, 11.8))?;
    ///
    /// // Write the spectrum to the HDF5 file (creates the file if it does not exist).
    /// Hdf5::write_spectrum(target_path, &spectrum)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn write_spectra<P: AsRef<Path>, S: AsRef<Spectrum>>(path: P, spectra: &[S]) -> Result<()> {
        let file = hdf5::File::create(path.as_ref())?;
        let basename = path
            .as_ref()
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap();
        let digits = (spectra.len() as f64).log10().ceil() as usize;
        spectra
            .iter()
            .enumerate()
            .try_for_each(|(i, spectrum)| {
                let dataset = format!("{basename}_{:0digits$}", i + 1, digits = digits);
                Self::write_to_file(&file, dataset.as_str(), spectrum.as_ref())
            })?;

        Ok(())
    }

    /// Internal helper function to read the spectrum in the provided dataset
    /// from the provided HDF5 file handle and return it.
    ///
    /// # Errors
    ///
    /// The following errors are possible:
    /// - [`EmptyData`](crate::spectrum::error::Kind::EmptyData)
    /// - [`DataLengthMismatch`](crate::spectrum::error::Kind::DataLengthMismatch)
    /// - [`NonUniformSpacing`](crate::spectrum::error::Kind::NonUniformSpacing)
    /// - [`InvalidIntensities`](crate::spectrum::error::Kind::InvalidIntensities)
    /// - [`InvalidSignalBoundaries`](crate::spectrum::error::Kind::InvalidSignalBoundaries)
    /// - [`Error:Hdf5Error`](crate::Error::Hdf5Error)
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
        let spectrum = Spectrum::new(
            chemical_shifts,
            intensities,
            (signal_boundaries[0], signal_boundaries[1]),
        )?;

        Ok(spectrum)
    }

    fn write_to_file(file: &hdf5::File, dataset: &str, spectrum: &Spectrum) -> Result<()> {
        let spectrum_group = file
            .create_group(dataset)?
            .create_group("spectrum")?;
        let data_group = spectrum_group.create_group("data")?;
        let meta_group = spectrum_group.create_group("meta")?;
        data_group
            .new_dataset_builder()
            .with_data(spectrum.chemical_shifts())
            .create("chemical_shifts")?;
        data_group
            .new_dataset_builder()
            .with_data(spectrum.intensities())
            .create("signal_intensities")?;
        meta_group
            .new_dataset_builder()
            .with_data(&[
                spectrum.signal_boundaries().0,
                spectrum.signal_boundaries().1,
            ])
            .create("signal_boundaries")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::check_sim_spectrum;
    use float_cmp::assert_approx_eq;

    #[test]
    fn read_spectrum() {
        let path = "../data/hdf5/sim.h5";
        let dataset = "sim_01";
        let spectrum = Hdf5::read_spectrum(path, dataset).unwrap();
        check_sim_spectrum!(spectrum);
    }

    #[test]
    fn read_spectra() {
        let path = "../data/hdf5/sim.h5";
        let spectra = Hdf5::read_spectra(path).unwrap();
        assert_eq!(spectra.len(), 16);
        spectra.iter().for_each(|spectrum| {
            check_sim_spectrum!(spectrum);
        })
    }
}
