use metabodecon::*;
use std::io::Write;

#[cfg(test)]
pub fn store_deconvolution(deconvolution: Deconvolution, path: &str) {
    let mut file = std::fs::File::create(path).unwrap();
    writeln!(file, "sfhw,hw2,maxp").unwrap();
    for lorentzian in deconvolution.lorenztians() {
        let (sfhw, hw2, maxp) = lorentzian.parameters();
        writeln!(file, "{},{},{}", sfhw, hw2, maxp).unwrap();
    }
}
