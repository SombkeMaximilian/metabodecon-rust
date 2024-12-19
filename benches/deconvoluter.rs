use criterion::{criterion_group, criterion_main, Criterion};
use metabodecon::*;

fn d() -> Deconvoluter {
    Deconvoluter::new(
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
    )
}

fn read_spectra() -> (Spectrum, Spectrum, Spectrum) {
    let sim_spectrum = Spectrum::from_hdf5("../data/hdf5/sim.h5", "sim_01").unwrap();
    let blood_spectrum = Spectrum::from_hdf5("../data/hdf5/blood.h5", "blood_01").unwrap();
    let urine_spectrum = Spectrum::from_hdf5("../data/hdf5/urine.h5", "urine_1").unwrap();
    (sim_spectrum, blood_spectrum, urine_spectrum)
}

fn deconvolute_spectrum(c: &mut Criterion) {
    let (mut sim_spectrum, mut blood_spectrum, mut urine_spectrum) = read_spectra();
    let deconvoluter = d();

    c.bench_function("deconvolute_sim_spectrum", |b| {
        b.iter(|| deconvoluter.deconvolute_spectrum(&mut sim_spectrum))
    });
    c.bench_function("deconvolute_blood_spectrum", |b| {
        b.iter(|| deconvoluter.deconvolute_spectrum(&mut blood_spectrum))
    });
    c.bench_function("deconvolute_urine_spectrum", |b| {
        b.iter(|| deconvoluter.deconvolute_spectrum(&mut urine_spectrum))
    });
}

fn par_deconvolute_spectrum(c: &mut Criterion) {
    let (mut sim_spectrum, mut blood_spectrum, mut urine_spectrum) = read_spectra();
    let deconvoluter = d();

    c.bench_function("parallel_deconvolute_sim_spectrum", |b| {
        b.iter(|| deconvoluter.par_deconvolute_spectrum(&mut sim_spectrum))
    });
    c.bench_function("parallel_deconvolute_blood_spectrum", |b| {
        b.iter(|| deconvoluter.par_deconvolute_spectrum(&mut blood_spectrum))
    });
    c.bench_function("parallel_deconvolute_urine_spectrum", |b| {
        b.iter(|| deconvoluter.par_deconvolute_spectrum(&mut urine_spectrum))
    });
}

fn criterion_config() -> Criterion {
    Criterion::default().measurement_time(std::time::Duration::from_secs(20))
}

criterion_group! {
    name = deconvoluter;
    config = criterion_config();
    targets = deconvolute_spectrum, par_deconvolute_spectrum
}

criterion_main!(deconvoluter);
