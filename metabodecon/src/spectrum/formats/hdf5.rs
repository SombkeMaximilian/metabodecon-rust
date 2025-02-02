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
/// groups, which contain the raw data and metadata, respectively. The
/// `spectrum` group contains the `data` and `meta` groups, which contain the
/// raw data and metadata, respectively.
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
        let datasets: Vec<String> = file.member_names()?.into_iter().collect();
        let spectra = datasets
            .into_iter()
            .map(|dataset| Self::read_from_file(&file, &dataset))
            .collect::<Result<Vec<Spectrum>>>()?;

        Ok(spectra)
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spectrum::macros::check_sim_spectrum;
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
