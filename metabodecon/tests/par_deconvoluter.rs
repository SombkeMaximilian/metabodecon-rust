use metabodecon::*;
mod common;
use common::store_deconvolution;

#[cfg(feature = "parallel")]
#[cfg(test)]
pub fn run_par_deconvolution(path: &str, data: &str) -> Deconvolution {
    let mut spectrum = Spectrum::from_hdf5(path, data).unwrap();
    let deconvoluter = Deconvoluter::new(
        SmoothingAlgo::MovingAverage {
            algo: MovingAverageAlgo::SumCache,
            iterations: 2,
            window_size: 5,
        },
        SelectionAlgo::Default {
            scoring_algo: ScoringAlgo::MinimumSum,
            threshold: 6.4,
        },
        FittingAlgo::Analytical { iterations: 10 },
    );
    deconvoluter.par_deconvolute_spectrum(&mut spectrum)
}

#[cfg(feature = "parallel")]
#[test]
fn sim() {
    let path = "../data/hdf5/sim.h5";
    let dataset = "sim_01";
    let deconvolution = run_par_deconvolution(path, dataset);
    store_deconvolution(deconvolution, "../target/sim_par_deconvolution.csv");
}

#[cfg(feature = "parallel")]
#[test]
fn blood() {
    let path = "../data/hdf5/blood.h5";
    let dataset = "blood_01";
    let deconvolution = run_par_deconvolution(path, dataset);
    store_deconvolution(deconvolution, "../target/blood_par_deconvolution.csv");
}

#[cfg(feature = "parallel")]
#[test]
fn urine() {
    let path = "../data/hdf5/urine.h5";
    let dataset = "urine_1";
    let deconvolution = run_par_deconvolution(path, dataset);
    store_deconvolution(deconvolution, "../target/urine_par_deconvolution.csv");
}
