use criterion::{Criterion, criterion_group, criterion_main};
use metabodecon::spectrum::JcampDx;

fn read_spectrum(c: &mut Criterion) {
    let affn_path = "data/jcampdx/BRUKAFFN.dx";
    let pac_path = "data/jcampdx/BRUKPAC.dx";
    let sqz_path = "data/jcampdx/BRUKSQZ.dx";
    let dif_dup_path = "data/jcampdx/BRUKDIF.dx";
    let n_tuples_path = "data/jcamp-dx/BRUKNTUP.dx";

    c.bench_function("read_jdx_affn_spectrum", |b| {
        b.iter(|| JcampDx::read_spectrum(affn_path, (0.0, 0.1)))
    });
    c.bench_function("read_jdx_pac_spectrum", |b| {
        b.iter(|| JcampDx::read_spectrum(pac_path, (0.0, 0.1)))
    });
    c.bench_function("read_jdx_sqz_spectrum", |b| {
        b.iter(|| JcampDx::read_spectrum(sqz_path, (0.0, 0.1)))
    });
    c.bench_function("read_jdx_dif_dup_spectrum", |b| {
        b.iter(|| JcampDx::read_spectrum(dif_dup_path, (0.0, 0.1)))
    });
    c.bench_function("read_jdx_n_tuples_spectrum", |b| {
        b.iter(|| JcampDx::read_spectrum(n_tuples_path, (0.0, 0.1)))
    });
}

criterion_group! {
    name = jcampdx_reader;
    config = Criterion::default();
    targets = read_spectrum
}

criterion_main!(jcampdx_reader);
