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

    pub fn maximum_position(p: &PeakStencilData) -> f64 {
        let numerator = p.x_1().powi(2) * p.y_1() * (p.y_2() - p.y_3())
            + p.x_2().powi(2) * p.y_2() * (p.y_3() - p.y_1())
            + p.x_3().powi(2) * p.y_3() * (p.y_1() - p.y_2());
        let divisor = 2. * (p.x_1() - p.x_2()) * p.y_1() * p.y_2()
            + 2. * (p.x_2() - p.x_3()) * p.y_2() * p.y_3()
            + 2. * (p.x_3() - p.x_1()) * p.y_3() * p.y_1();
        numerator / divisor
    }

    pub fn half_width2(p: &PeakStencilData, maxp: f64) -> f64 {
        let left = (p.y_1() * (p.x_1() - maxp).powi(2) - p.y_2() * (p.x_2() - maxp).powi(2))
            / (p.y_2() - p.y_1());
        let right = (p.y_2() * (p.x_2() - maxp).powi(2) - p.y_3() * (p.x_3() - maxp).powi(2))
            / (p.y_3() - p.y_2());
        (left + right) / 2.
    }

    pub fn scale_factor_half_width(p: &PeakStencilData, maxp: f64, hw2: f64) -> f64 {
        p.y_2() * (hw2 + (p.x_2() - maxp).powi(2))
    }
}
