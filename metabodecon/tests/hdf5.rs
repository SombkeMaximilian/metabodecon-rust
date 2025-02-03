use metabodecon::spectrum;

#[cfg(feature = "hdf5")]
#[test]
fn convert_spectrum() {
    let read_path = "../data/bruker/blood/blood_01";
    let spectrum = spectrum::Bruker::read_spectrum(read_path, 10, 10, (-2.2, 11.8)).unwrap();
    let tmp_path = env!("CARGO_TARGET_TMPDIR");
    let write_dir = format!("{}/test-hdf5", tmp_path);
    std::fs::create_dir_all(write_dir.as_str()).unwrap();
    let filename = format!("{}/{}", write_dir, "blood_incremental.h5");
    spectrum::Hdf5::write_spectrum(filename, &spectrum).unwrap();
}

#[cfg(feature = "hdf5")]
#[test]
fn convert_spectra() {
    let read_path = "../data/bruker/blood";
    let spectra = spectrum::Bruker::read_spectra(read_path, 10, 10, (-2.2, 11.8)).unwrap();
    let tmp_path = env!("CARGO_TARGET_TMPDIR");
    let write_dir = format!("{}/test-hdf5", tmp_path);
    std::fs::create_dir_all(write_dir.as_str()).unwrap();
    let filename = format!("{}/{}", write_dir, "blood.h5");
    spectrum::Hdf5::write_spectra(filename, &spectra).unwrap();
}
