use metabodecon::{deconvolution::*, spectrum::*};
mod common;
use common::store_deconvolution;

#[cfg(test)]
pub fn run_deconvolution(path: &str, dataset: &str) -> metabodecon::Result<Deconvolution> {
    let reader = Hdf5Reader::new();
    let mut spectrum = reader.read_spectrum(path, dataset)?;
    let deconvoluter = Deconvoluter::new(
        SmoothingAlgo::MovingAverage {
            iterations: 2,
            window_size: 5,
        },
        SelectionAlgo::NoiseScoreFilter {
            scoring_algo: ScoringAlgo::MinimumSum,
            threshold: 6.4,
        },
        FittingAlgo::Analytical { iterations: 10 },
    );
    deconvoluter.deconvolute_spectrum(&mut spectrum)
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
