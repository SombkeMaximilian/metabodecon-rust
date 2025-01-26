use criterion::{Criterion, criterion_group, criterion_main};
use metabodecon::{deconvolution::*, spectrum::*};

fn single_spectrum(c: &mut Criterion) {
    let reader = Hdf5Reader::new();
    let sim_spectrum = reader
        .read_spectrum("../data/hdf5/sim.h5", "sim_01")
        .unwrap();
    let blood_spectrum = reader
        .read_spectrum("../data/hdf5/blood.h5", "blood_01")
        .unwrap();
    let deconvoluter = Deconvoluter::default();

    // Sequential
    c.bench_function("deconvolute_sim_spectrum", |b| {
        b.iter(|| deconvoluter.deconvolute_spectrum(&sim_spectrum))
    });
    c.bench_function("deconvolute_blood_spectrum", |b| {
        b.iter(|| deconvoluter.deconvolute_spectrum(&blood_spectrum))
    });

    // Parallel
    c.bench_function("parallel_deconvolute_sim_spectrum", |b| {
        b.iter(|| deconvoluter.par_deconvolute_spectrum(&sim_spectrum))
    });
    c.bench_function("parallel_deconvolute_blood_spectrum", |b| {
        b.iter(|| deconvoluter.par_deconvolute_spectrum(&blood_spectrum))
    });
}

fn multiple_spectra(c: &mut Criterion) {
    let reader = Hdf5Reader::new();
    let sim_spectra = reader
        .read_spectra("../data/hdf5/sim.h5")
        .unwrap();
    let blood_spectra = reader
        .read_spectra("../data/hdf5/blood.h5")
        .unwrap();
    let deconvoluter = Deconvoluter::default();

    // Only the parallel case is interesting here
    c.bench_function("parallel_deconvolute_sim_spectra", |b| {
        b.iter(|| deconvoluter.par_deconvolute_spectra(&sim_spectra))
    });
    c.bench_function("parallel_deconvolute_blood_spectra", |b| {
        b.iter(|| deconvoluter.par_deconvolute_spectra(&blood_spectra))
    });
}

criterion_group! {
    name = deconvolute_spectrum;
    config = Criterion::default();
    targets = single_spectrum
}

criterion_group! {
    name = deconvolute_spectra;
    config = Criterion::default();
    targets = multiple_spectra
}

criterion_main!(deconvolute_spectrum, deconvolute_spectra);
