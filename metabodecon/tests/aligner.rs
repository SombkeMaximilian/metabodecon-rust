use metabodecon::alignment::Aligner;
use metabodecon::deconvolution::Deconvoluter;
use metabodecon::spectrum::Bruker;

#[test]
fn blood() {
    let spectra = Bruker::read_spectra("../data/bruker/blood", 10, 10, (-2.2, 11.8)).unwrap();
    let deconvoluter = Deconvoluter::default();
    let deconvolutions = deconvoluter
        .deconvolute_spectra(&spectra)
        .unwrap();
    let aligner = Aligner::new(0.02, 0.5);
    let _ = aligner.align_deconvolutions(&deconvolutions);
}
