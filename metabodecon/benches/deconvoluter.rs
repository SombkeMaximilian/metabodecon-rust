use criterion::{black_box, criterion_group, criterion_main, Criterion};
use metabodecon::*;

fn deconvolute_spectrum(c: &mut Criterion) {
    let sim_spectrum = Spectrum::from_hdf5("data/sim.h5", "sim_01").unwrap();
    let blood_spectrum = Spectrum::from_hdf5("data/blood.h5", "blood_01").unwrap();
    let urine_spectrum = Spectrum::from_hdf5("data/urine.h5", "urine_1").unwrap();
    let deconvoluter = Deconvoluter::new(
        SmoothingAlgo::MovingAverage {
            algo: MovingAverageAlgo::Simple,
            iterations: 2,
            window_size: 5,
        },
        6.4,
        FittingAlgo::Analytical { iterations: 10 },
    );

    c.bench_function("deconvolute_sim_spectrum", |b| {
        b.iter(|| {
            let mut spectrum_clone = sim_spectrum.clone();
            deconvoluter.deconvolute_spectrum(black_box(&mut spectrum_clone));
        })
    });
    c.bench_function("deconvolute_blood_spectrum", |b| {
        b.iter(|| {
            let mut spectrum_clone = blood_spectrum.clone();
            deconvoluter.deconvolute_spectrum(black_box(&mut spectrum_clone));
        })
    });
    c.bench_function("deconvolute_urine_spectrum", |b| {
        b.iter(|| {
            let mut spectrum_clone = urine_spectrum.clone();
            deconvoluter.deconvolute_spectrum(black_box(&mut spectrum_clone));
        })
    });
}

fn criterion_config() -> Criterion {
    Criterion::default().measurement_time(std::time::Duration::from_secs(20))
}

criterion_group! {
    name = benches;
    config = criterion_config();
    targets = deconvolute_spectrum
}
criterion_main!(benches);
