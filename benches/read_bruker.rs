use criterion::{criterion_group, criterion_main, Criterion};
use metabodecon::BrukerReader;

fn read_spectrum(c: &mut Criterion) {
    let reader = BrukerReader::new();

    c.bench_function("read_bruker_sim_spectrum", |b| {
        b.iter(|| reader.read_spectrum("data/bruker/sim/sim_01", 10, 10))
    });
    c.bench_function("read_bruker_blood_spectrum", |b| {
        b.iter(|| reader.read_spectrum("data/bruker/blood/blood_01", 10, 10))
    });
}

fn read_spectra(c: &mut Criterion) {
    let reader = BrukerReader::new();

    c.bench_function("read_bruker_sim_spectra", |b| {
        b.iter(|| reader.read_spectra("data/bruker/sim", 10, 10))
    });
    c.bench_function("read_bruker_blood_spectra", |b| {
        b.iter(|| reader.read_spectra("data/bruker/blood", 10, 10))
    });
}

criterion_group! {
    name = bruker_reader;
    config = Criterion::default();
    targets = read_spectrum, read_spectra
}

criterion_main!(bruker_reader);
