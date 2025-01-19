use metabodecon::deconvolution::Deconvolution;
use std::io::Write;

#[cfg(test)]
#[allow(unreachable_pub)]
pub fn store_deconvolution(deconvolution: Deconvolution, filename: &str) {
    let tmp_path = env!("CARGO_TARGET_TMPDIR");
    let deconvolutions = format!("{}/test-deconvolutions", tmp_path);
    std::fs::create_dir_all(deconvolutions).unwrap();
    let filename = format!("{}/test-deconvolutions/{}", tmp_path, filename);
    let mut file = std::fs::File::create(filename).unwrap();
    writeln!(file, "sfhw,hw2,maxp").unwrap();
    for lorentzian in deconvolution.lorentzians() {
        let (sfhw, hw2, maxp) = lorentzian.parameters();
        writeln!(file, "{},{},{}", sfhw, hw2, maxp).unwrap();
    }
}
