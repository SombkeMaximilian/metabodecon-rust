use metabodecon::{Lorentzian, Spectrum};

pub fn read_hdf5_data(_file_path: &str) -> Result<(Vec<Spectrum>, Vec<Lorentzian>), hdf5::Error> {
    Ok((read_hdf5_spectra(_file_path)?, read_hdf5_deconvolution(_file_path)?))
}

pub fn read_hdf5_spectra(_file_path: &str) -> Result<Vec<Spectrum>, hdf5::Error> {
    unimplemented!()
}

pub fn read_hdf5_deconvolution(_file_path: &str) -> Result<Vec<Lorentzian>, hdf5::Error> {
    unimplemented!()
}
