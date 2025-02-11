use criterion::{Criterion, criterion_group, criterion_main};
use metabodecon::{deconvolution::*, spectrum::*};

fn single_spectrum(c: &mut Criterion) {
    let sim_path = "../data/bruker/sim/sim_01";
    let blood_path = "../data/bruker/blood/blood_01";
    let sim_spectrum = Bruker::read_spectrum(sim_path, 10, 10, (3.339007, 3.553942)).unwrap();
    let blood_spectrum = Bruker::read_spectrum(blood_path, 10, 10, (-2.2, 11.8)).unwrap();
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
    let sim_path = "../data/bruker/sim";
    let blood_path = "../data/bruker/blood";
    let sim_spectra = Bruker::read_spectra(sim_path, 10, 10, (3.339007, 3.553942)).unwrap();
    let blood_spectra = Bruker::read_spectra(blood_path, 10, 10, (-2.2, 11.8)).unwrap();
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
