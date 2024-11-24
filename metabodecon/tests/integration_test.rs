use metabodecon::*;
use std::fs::File;
use std::io::Write;

#[test]
fn test_deconvoluter() -> Result<(), std::io::Error> {
    let mut spectrum = Spectrum::from_hdf5("data/blood.h5", "blood_01").unwrap();
    let deconvoluter = Deconvoluter::new(
        SmoothingAlgo::MovingAverage {
            algo: MovingAverageAlgo::Simple,
            iterations: 2,
            window_size: 5,
        },
        SelectionAlgo::Default {
            scoring_algo: ScoringAlgo::MinimumSum,
            threshold: 6.4,
        },
        FittingAlgo::Analytical { iterations: 10 },
    );
    let deconvolution = deconvoluter.deconvolute_spectrum(&mut spectrum);

    let mut file = File::create("deconvolution_results.csv")?;
    writeln!(file, "sfhw,hw2,maxp")?;

    for lorentzian in deconvolution.lorenztians() {
        let (sfhw, hw2, maxp) = lorentzian.parameters();
        writeln!(file, "{},{},{}", sfhw, hw2, maxp)?;
    }

    Ok(())
}
