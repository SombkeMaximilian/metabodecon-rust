use criterion::{black_box, criterion_group, criterion_main, Criterion};
use metabodecon::*;

fn deconvolute_spectrum(c: &mut Criterion) {
    let mut sim_spectrum = Spectrum::from_hdf5("data/sim.h5", "sim_01").unwrap();
    let mut blood_spectrum = Spectrum::from_hdf5("data/blood.h5", "blood_01").unwrap();
    let mut urine_spectrum = Spectrum::from_hdf5("data/urine.h5", "urine_1").unwrap();
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

    #[cfg(feature = "sequential")]
    {
        c.bench_function("deconvolute_sim_spectrum", |b| {
            b.iter(|| {
                deconvoluter.deconvolute_spectrum(black_box(&mut sim_spectrum));
            })
        });
        c.bench_function("deconvolute_blood_spectrum", |b| {
            b.iter(|| {
                deconvoluter.deconvolute_spectrum(black_box(&mut blood_spectrum));
            })
        });
        c.bench_function("deconvolute_urine_spectrum", |b| {
            b.iter(|| {
                deconvoluter.deconvolute_spectrum(black_box(&mut urine_spectrum));
            })
        });
    }

    #[cfg(feature = "parallel")]
    {
        c.bench_function("parallel_deconvolute_sim_spectrum", |b| {
            b.iter(|| {
                deconvoluter.deconvolute_spectrum(black_box(&mut sim_spectrum));
            })
        });
        c.bench_function("parallel_deconvolute_blood_spectrum", |b| {
            b.iter(|| {
                deconvoluter.deconvolute_spectrum(black_box(&mut blood_spectrum));
            })
        });
        c.bench_function("parallel_deconvolute_urine_spectrum", |b| {
            b.iter(|| {
                deconvoluter.deconvolute_spectrum(black_box(&mut urine_spectrum));
            })
        });
    }
}

fn criterion_config() -> Criterion {
    Criterion::default().measurement_time(std::time::Duration::from_secs(20))
}

#[cfg(feature = "sequential")]
criterion_group! {
    name = sequential_benches;
    config = criterion_config();
    targets = deconvolute_spectrum
}

#[cfg(feature = "sequential")]
criterion_main!(sequential_benches);

#[cfg(feature = "parallel")]
criterion_group! {
    name = parallel_benches;
    config = criterion_config();
    targets = deconvolute_spectrum
}

#[cfg(feature = "parallel")]
criterion_main!(parallel_benches);
