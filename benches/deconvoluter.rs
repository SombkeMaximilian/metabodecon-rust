use criterion::{Criterion, criterion_group, criterion_main};
use metabodecon::{deconvolution::*, spectrum::*};

fn read_spectra() -> (Spectrum, Spectrum, Spectrum) {
    let reader = Hdf5Reader::new();
    let sim_spectrum = reader
        .read_spectrum("../data/hdf5/sim.h5", "sim_01")
        .unwrap();
    let blood_spectrum = reader
        .read_spectrum("../data/hdf5/blood.h5", "blood_01")
        .unwrap();
    let urine_spectrum = reader
        .read_spectrum("../data/hdf5/urine.h5", "urine_1")
        .unwrap();
    (sim_spectrum, blood_spectrum, urine_spectrum)
}

fn deconvolute_spectrum(c: &mut Criterion) {
    let (mut sim_spectrum, mut blood_spectrum, mut urine_spectrum) = read_spectra();
    let deconvoluter = Deconvoluter::default();

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
    let deconvoluter = Deconvoluter::default();

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
