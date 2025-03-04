use criterion::{Criterion, criterion_group, criterion_main};
use metabodecon::spectrum::Bruker;

fn read_spectrum(c: &mut Criterion) {
    let sim_path = "data/bruker/sim/sim_01";
    let blood_path = "data/bruker/blood/blood_01";

    c.bench_function("read_bruker_sim_spectrum", |b| {
        b.iter(|| Bruker::read_spectrum(sim_path, 10, 10, (0.0, 0.1)))
    });
    c.bench_function("read_bruker_blood_spectrum", |b| {
        b.iter(|| Bruker::read_spectrum(blood_path, 10, 10, (0.0, 0.1)))
    });
}

fn read_spectra(c: &mut Criterion) {
    let sim_path = "data/bruker/sim";
    let blood_path = "data/bruker/blood";

    c.bench_function("read_bruker_sim_spectra", |b| {
        b.iter(|| Bruker::read_spectra(sim_path, 10, 10, (0.0, 0.1)))
    });
    c.bench_function("read_bruker_blood_spectra", |b| {
        b.iter(|| Bruker::read_spectra(blood_path, 10, 10, (0.0, 0.1)))
    });
}

criterion_group! {
    name = bruker_reader;
    config = Criterion::default();
    targets = read_spectrum, read_spectra
}

criterion_main!(bruker_reader);
