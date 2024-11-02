use crate::data::{Lorentzian, Peak, Spectrum};
use crate::fitting::{Fitter, PeakStencilData, ReducedSpectrum};

pub struct FitterAnalytical {
    iterations: usize,
}

impl Fitter for FitterAnalytical {
    fn fit_lorentzian(&self, spectrum: &Spectrum, peaks: &[Peak]) -> Vec<Lorentzian> {
        let _reduced_spectrum = ReducedSpectrum::from_spectrum(spectrum, peaks);
        let peak_data = peaks
            .iter()
            .map(|peak| PeakStencilData::from_peak(spectrum, peak))
            .collect::<Vec<_>>();
        let mut lorentzians = peak_data
            .iter()
            .map(|peak| {
                let maxp = Self::maximum_position(peak);
                let hw2 = Self::half_width2(peak, maxp);
                let sfhw = Self::scale_factor_half_width(peak, maxp, hw2);
                Lorentzian::from_param(sfhw, hw2, maxp)
            })
            .collect::<Vec<_>>();

        lorentzians
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_approximations() {
        let peak = PeakStencilData::from_data(4., 8., 12., 5., 10., 5.);
        let maxp = FitterAnalytical::maximum_position(&peak);
        let hw2 = FitterAnalytical::half_width2(&peak, maxp);
        let sfhw = FitterAnalytical::scale_factor_half_width(&peak, maxp, hw2);
        assert_eq!(maxp, 8.);
        assert_eq!(hw2.sqrt(), 4.);
        assert_eq!(sfhw / hw2.sqrt(), 40.);
    }
}
