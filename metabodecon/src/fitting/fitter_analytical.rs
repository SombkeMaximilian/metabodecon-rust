use crate::data::{Lorentzian, Peak, Spectrum};
use crate::fitting::{Fitter, PeakStencilData, ReducedSpectrum};

pub struct FitterAnalytical {
    iterations: usize,
}

impl Fitter for FitterAnalytical {
    fn fit_lorentzian(&self, spectrum: &Spectrum, peaks: &[Peak]) -> Vec<Lorentzian> {
        let _reduced_spectrum = ReducedSpectrum::from_spectrum(spectrum, peaks);
        let _peak_data = peaks
            .iter()
            .map(|peak| PeakStencilData::from_peak(spectrum, peak))
            .collect::<Vec<_>>();

        Vec::<Lorentzian>::new()
    }
}

impl FitterAnalytical {
    pub fn new(iterations: usize) -> Self {
        Self { iterations }
    }

    pub fn iterations(&self) -> usize {
        self.iterations
    }

    pub fn set_iterations(&mut self, iterations: usize) {
        self.iterations = iterations;
    }

}
