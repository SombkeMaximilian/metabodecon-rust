use metabodecon::{deconvolution::*, spectrum::*};
use std::io::Write;

#[cfg(test)]
fn store_deconvolution(deconvolution: Deconvolution, filename: &str) {
    let tmp_path = env!("CARGO_TARGET_TMPDIR");
    let deconvolutions = format!("{}/test-deconvolutions", tmp_path);
    std::fs::create_dir_all(deconvolutions).unwrap();
    let filename = format!("{}/test-deconvolutions/{}", tmp_path, filename);
    let mut file = std::fs::File::create(filename).unwrap();
    writeln!(file, "sfhw,hw2,maxp").unwrap();
    for lorentzian in deconvolution.lorentzians() {
        let (sfhw, hw2, maxp) = lorentzian.parameters();
        writeln!(file, "{},{},{}", sfhw, hw2, maxp).unwrap();
    }
}

#[cfg(test)]
fn run_deconvolution(path: &str, dataset: &str) -> metabodecon::Result<Deconvolution> {
    let spectrum = Hdf5::read_spectrum(path, dataset)?;
    let deconvoluter = Deconvoluter::default();

    deconvoluter.deconvolute_spectrum(&spectrum)
}

#[cfg(feature = "parallel")]
#[cfg(test)]
fn par_run_deconvolution(path: &str, data: &str) -> metabodecon::Result<Deconvolution> {
    let spectrum = Hdf5::read_spectrum(path, data)?;
    let deconvoluter = Deconvoluter::default();

    deconvoluter.par_deconvolute_spectrum(&spectrum)
}

#[test]
fn sim() {
    let path = "../data/hdf5/sim.h5";
    let dataset = "sim_01";
    let deconvolution = run_deconvolution(path, dataset).unwrap();
    let filename = format!("{}_deconvolution.csv", dataset);
    store_deconvolution(deconvolution, filename.as_str());
}

#[test]
fn blood() {
    let path = "../data/hdf5/blood.h5";
    let dataset = "blood_01";
    let deconvolution = run_deconvolution(path, dataset).unwrap();
    let filename = format!("{}_deconvolution.csv", dataset);
    store_deconvolution(deconvolution, filename.as_str());
}

#[test]
fn urine() {
    let path = "../data/hdf5/urine.h5";
    let dataset = "urine_1";
    let deconvolution = run_deconvolution(path, dataset).unwrap();
    let filename = format!("{}_deconvolution.csv", dataset);
    store_deconvolution(deconvolution, filename.as_str());
}

#[cfg(feature = "parallel")]
#[test]
fn par_sim() {
    let path = "../data/hdf5/sim.h5";
    let dataset = "sim_01";
    let deconvolution = par_run_deconvolution(path, dataset).unwrap();
    let filename = format!("{}_par_deconvolution.csv", dataset);
    store_deconvolution(deconvolution, filename.as_str());
}

#[cfg(feature = "parallel")]
#[test]
fn par_blood() {
    let path = "../data/hdf5/blood.h5";
    let dataset = "blood_01";
    let deconvolution = par_run_deconvolution(path, dataset).unwrap();
    let filename = format!("{}_par_deconvolution.csv", dataset);
    store_deconvolution(deconvolution, filename.as_str());
}

#[cfg(feature = "parallel")]
#[test]
fn par_urine() {
    let path = "../data/hdf5/urine.h5";
    let dataset = "urine_1";
    let deconvolution = par_run_deconvolution(path, dataset).unwrap();
    let filename = format!("{}_par_deconvolution.csv", dataset);
    store_deconvolution(deconvolution, filename.as_str());
}
