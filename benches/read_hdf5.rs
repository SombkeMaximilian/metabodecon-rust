use criterion::{criterion_group, criterion_main, Criterion};
use metabodecon::spectrum::Hdf5Reader;

fn read_spectrum(c: &mut Criterion) {
    let reader = Hdf5Reader::new();

    c.bench_function("read_hdf5_sim_spectrum", |b| {
        b.iter(|| reader.read_spectrum("data/hdf5/sim.h5", "sim_01"))
    });
    c.bench_function("read_hdf5_blood_spectrum", |b| {
        b.iter(|| reader.read_spectrum("data/hdf5/blood.h5", "blood_01"))
    });
}

fn read_spectra(c: &mut Criterion) {
    let reader = Hdf5Reader::new();

    c.bench_function("read_hdf5_sim_spectra", |b| {
        b.iter(|| reader.read_spectra("data/hdf5/sim.h5"))
    });
    c.bench_function("read_hdf5_blood_spectra", |b| {
        b.iter(|| reader.read_spectra("data/hdf5/blood.h5"))
    });
}

criterion_group! {
    name = hdf5_reader;
    config = Criterion::default();
    targets = read_spectrum, read_spectra
}

criterion_main!(hdf5_reader);
