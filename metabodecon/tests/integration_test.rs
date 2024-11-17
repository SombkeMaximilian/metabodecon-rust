use std::fs::File;
use std::io::Write;
use metabodecon::*;

#[test]
fn test_deconvoluter() -> Result<(), std::io::Error> {
    let mut spectrum = Spectrum::from_hdf5("data/sim.h5", "sim_01").unwrap();
    let deconvoluter = Deconvoluter::new(
        SmoothingAlgo::MovingAverage {
            algo: MovingAverageAlgo::Simple,
            iterations: 2,
            window_size: 5,
        },
        6.4,
        FittingAlgo::Analytical { iterations: 10 },
    );
    let deconvolution = deconvoluter.deconvolute_spectrum(&mut spectrum);

    let mut file = File::create("deconvolution_results.csv")?;
    writeln!(file, "sf,hw,maxp")?;

    for lorentzian in deconvolution.lorenztians() {
        let (sf, hw, maxp) = lorentzian.retransformed_parameters();
        writeln!(file, "{},{},{}", sf, hw, maxp)?;
    }

    Ok(())
}
