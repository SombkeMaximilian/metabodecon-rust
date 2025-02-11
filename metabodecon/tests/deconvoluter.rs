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

#[test]
fn sim() {
    let path = "../data/bruker/sim/sim_01";
    let spectrum = Bruker::read_spectrum(path, 10, 10, (3.339007, 3.553942)).unwrap();
    let deconvoluter = Deconvoluter::default();
    let deconvolution = deconvoluter.deconvolute_spectrum(&spectrum).unwrap();
    let filename = format!("{}_deconvolution.csv", "sim_01");
    store_deconvolution(deconvolution, filename.as_str());
}

#[test]
fn blood() {
    let path = "../data/bruker/blood/blood_01";
    let spectrum = Bruker::read_spectrum(path, 10, 10, (-2.2, 11.8)).unwrap();
    let deconvoluter = Deconvoluter::default();
    let deconvolution = deconvoluter.deconvolute_spectrum(&spectrum).unwrap();
    let filename = format!("{}_deconvolution.csv", "blood_01");
    store_deconvolution(deconvolution, filename.as_str());
}

#[cfg(feature = "parallel")]
#[test]
fn par_sim() {
    let path = "../data/bruker/sim/sim_01";
    let spectrum = Bruker::read_spectrum(path, 10, 10, (3.339007, 3.553942)).unwrap();
    let deconvoluter = Deconvoluter::default();
    let deconvolution = deconvoluter.par_deconvolute_spectrum(&spectrum).unwrap();
    let filename = format!("{}_par_deconvolution.csv", "sim_01");
    store_deconvolution(deconvolution, filename.as_str());
}

#[cfg(feature = "parallel")]
#[test]
fn par_blood() {
    let path = "../data/bruker/blood/blood_01";
    let spectrum = Bruker::read_spectrum(path, 10, 10, (-2.2, 11.8)).unwrap();
    let deconvoluter = Deconvoluter::default();
    let deconvolution = deconvoluter.par_deconvolute_spectrum(&spectrum).unwrap();
    let filename = format!("{}_par_deconvolution.csv", "blood_01");
    store_deconvolution(deconvolution, filename.as_str());
}
