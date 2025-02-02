use metabodecon::{deconvolution::*, spectrum::*};
mod common;
use common::store_deconvolution;

#[cfg(feature = "parallel")]
#[cfg(test)]
pub fn run_par_deconvolution(path: &str, data: &str) -> metabodecon::Result<Deconvolution> {
    let spectrum = Hdf5::read_spectrum(path, data)?;
    let deconvoluter = Deconvoluter::default();

    deconvoluter.par_deconvolute_spectrum(&spectrum)
}

#[cfg(feature = "parallel")]
#[test]
fn sim() {
    let path = "../data/hdf5/sim.h5";
    let dataset = "sim_01";
    let deconvolution = run_par_deconvolution(path, dataset).unwrap();
    let filename = format!("{}_par_deconvolution.csv", dataset);
    store_deconvolution(deconvolution, filename.as_str());
}

#[cfg(feature = "parallel")]
#[test]
fn blood() {
    let path = "../data/hdf5/blood.h5";
    let dataset = "blood_01";
    let deconvolution = run_par_deconvolution(path, dataset).unwrap();
    let filename = format!("{}_par_deconvolution.csv", dataset);
    store_deconvolution(deconvolution, filename.as_str());
}

#[cfg(feature = "parallel")]
#[test]
fn urine() {
    let path = "../data/hdf5/urine.h5";
    let dataset = "urine_1";
    let deconvolution = run_par_deconvolution(path, dataset).unwrap();
    let filename = format!("{}_par_deconvolution.csv", dataset);
    store_deconvolution(deconvolution, filename.as_str());
}
