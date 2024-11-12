use metabodecon::{Lorentzian, Spectrum};

#[allow(dead_code)]
pub fn read_hdf5_data(_file_path: &str) -> Result<(Vec<Spectrum>, Vec<Lorentzian>), hdf5::Error> {
    Ok((read_hdf5_spectra(_file_path)?, read_hdf5_deconvolution(_file_path)?))
}

#[allow(dead_code)]
pub fn read_hdf5_spectra(_file_path: &str) -> Result<Vec<Spectrum>, hdf5::Error> {
    unimplemented!()
}

#[allow(dead_code)]
pub fn read_hdf5_deconvolution(_file_path: &str) -> Result<Vec<Lorentzian>, hdf5::Error> {
    unimplemented!()
}
