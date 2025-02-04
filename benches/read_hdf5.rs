use criterion::{Criterion, criterion_group, criterion_main};
use metabodecon::spectrum::Hdf5;

fn read_spectrum(c: &mut Criterion) {
    c.bench_function("read_hdf5_sim_spectrum", |b| {
        b.iter(|| Hdf5::read_spectrum("data/hdf5/sim.h5", "sim_01"))
    });
    c.bench_function("read_hdf5_blood_spectrum", |b| {
        b.iter(|| Hdf5::read_spectrum("data/hdf5/blood.h5", "blood_01"))
    });
}

fn read_spectra(c: &mut Criterion) {
    c.bench_function("read_hdf5_sim_spectra", |b| {
        b.iter(|| Hdf5::read_spectra("data/hdf5/sim.h5"))
    });
    c.bench_function("read_hdf5_blood_spectra", |b| {
        b.iter(|| Hdf5::read_spectra("data/hdf5/blood.h5"))
    });
}

criterion_group! {
    name = hdf5_reader;
    config = Criterion::default();
    targets = read_spectrum, read_spectra
}

criterion_main!(hdf5_reader);
