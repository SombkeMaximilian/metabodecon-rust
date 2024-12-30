use crate::fitting::{Fitter, Lorentzian, PeakStencilData, ReducedSpectrum};
use crate::peak_selection::Peak;
use crate::spectrum::Spectrum;

#[cfg(feature = "parallel")]
use rayon::prelude::*;

/// Fitting algorithm based on the analytical solution of a system of equations
/// using a 3-point peak stencil.
#[derive(Debug)]
pub struct FitterAnalytical {
    /// The number of iterations to refine the Lorentzian parameters.
    iterations: usize,
}

impl Fitter for FitterAnalytical {
    /// Fits a set of Lorentzians to the spectrum using the given peaks.
    fn fit_lorentzian(&self, spectrum: &Spectrum, peaks: &[Peak]) -> Vec<Lorentzian> {
        let reduced_spectrum = ReducedSpectrum::new(spectrum, peaks);
        let mut peak_data = peaks
            .iter()
            .map(|peak| {
                let mut p = PeakStencilData::new(spectrum, peak);
                p.mirror_shoulder();
                p
            })
            .collect::<Vec<_>>();
        let mut lorentzians = peak_data
            .iter()
            .map(|peak| {
                let maxp = Self::maximum_position(peak);
                let hw2 = Self::half_width2(peak, maxp);
                let sfhw = Self::scale_factor_half_width(peak, maxp, hw2);
                Lorentzian::new(sfhw, hw2, maxp)
            })
            .collect::<Vec<_>>();

        for _ in 0..self.iterations {
            let superposition =
                Lorentzian::superposition_vec(reduced_spectrum.chemical_shifts(), &lorentzians);
            let ratios = reduced_spectrum
                .intensities()
                .iter()
                .zip(superposition.iter())
                .map(|(a, b)| a / b)
                .collect::<Vec<_>>();
            peak_data
                .iter_mut()
                .zip(ratios.chunks(3))
                .for_each(|(p, r)| {
                    p.set_y_1(p.y_1() * r[0]);
                    p.set_y_2(p.y_2() * r[1]);
                    p.set_y_3(p.y_3() * r[2]);
                    p.mirror_shoulder();
                });
            lorentzians
                .iter_mut()
                .zip(peak_data.iter())
                .for_each(|(l, p)| {
                    let maxp = Self::maximum_position(p);
                    let hw2 = Self::half_width2(p, maxp);
                    let sfhw = Self::scale_factor_half_width(p, maxp, hw2);
                    l.set_parameters(sfhw, hw2, maxp);
                });
        }

        lorentzians
    }

    /// Fits a set of Lorentzians to the spectrum using the given peaks in
    /// parallel.
    #[cfg(feature = "parallel")]
    fn par_fit_lorentzian(&self, spectrum: &Spectrum, peaks: &[Peak]) -> Vec<Lorentzian> {
        let reduced_spectrum = ReducedSpectrum::new(spectrum, peaks);
        let mut peak_data = peaks
            .iter()
            .map(|peak| {
                let mut p = PeakStencilData::new(spectrum, peak);
                p.mirror_shoulder();
                p
            })
            .collect::<Vec<_>>();
        let mut lorentzians = peak_data
            .iter()
            .map(|peak| {
                let maxp = Self::maximum_position(peak);
                let hw2 = Self::half_width2(peak, maxp);
                let sfhw = Self::scale_factor_half_width(peak, maxp, hw2);
                Lorentzian::new(sfhw, hw2, maxp)
            })
            .collect::<Vec<_>>();

        for _ in 0..self.iterations {
            let superposition =
                Lorentzian::par_superposition_vec(reduced_spectrum.chemical_shifts(), &lorentzians);
            let ratios = reduced_spectrum
                .intensities()
                .iter()
                .zip(superposition.iter())
                .map(|(a, b)| a / b)
                .collect::<Vec<_>>();
            peak_data
                .iter_mut()
                .zip(ratios.chunks(3))
                .for_each(|(p, r)| {
                    p.set_y_1(p.y_1() * r[0]);
                    p.set_y_2(p.y_2() * r[1]);
                    p.set_y_3(p.y_3() * r[2]);
                    p.mirror_shoulder();
                });
            lorentzians
                .par_iter_mut()
                .zip(peak_data.par_iter())
                .for_each(|(l, p)| {
                    let maxp = Self::maximum_position(p);
                    let hw2 = Self::half_width2(p, maxp);
                    let sfhw = Self::scale_factor_half_width(p, maxp, hw2);
                    l.set_parameters(sfhw, hw2, maxp);
                });
        }

        lorentzians
    }
}

impl FitterAnalytical {
    /// Constructs a new `FitterAnalytical` with the given number of iterations.
    pub fn new(iterations: usize) -> Self {
        Self { iterations }
    }

    /// Internal helper function to analytically compute the maximum position of
    /// the peak in ppm by solving the system of 3 equations.
    fn maximum_position(p: &PeakStencilData) -> f64 {
        let numerator = p.x_1().powi(2) * p.y_1() * (p.y_2() - p.y_3())
            + p.x_2().powi(2) * p.y_2() * (p.y_3() - p.y_1())
            + p.x_3().powi(2) * p.y_3() * (p.y_1() - p.y_2());
        let divisor = 2. * (p.x_1() - p.x_2()) * p.y_1() * p.y_2()
            + 2. * (p.x_2() - p.x_3()) * p.y_2() * p.y_3()
            + 2. * (p.x_3() - p.x_1()) * p.y_3() * p.y_1();
        numerator / divisor
    }

    /// Internal helper function to analytically compute the half width at half
    /// maximum of the peak in ppm^2 by solving the system of 3 equations.
    fn half_width2(p: &PeakStencilData, maxp: f64) -> f64 {
        let left = (p.y_1() * (p.x_1() - maxp).powi(2) - p.y_2() * (p.x_2() - maxp).powi(2))
            / (p.y_2() - p.y_1());
        let right = (p.y_2() * (p.x_2() - maxp).powi(2) - p.y_3() * (p.x_3() - maxp).powi(2))
            / (p.y_3() - p.y_2());
        (left + right) / 2.
    }

    /// Internal helper function to analytically compute the scale factor times
    /// the half width at half maximum of the peak in ppm^2 by solving the
    /// system of 3 equations.
    fn scale_factor_half_width(p: &PeakStencilData, maxp: f64, hw2: f64) -> f64 {
        p.y_2() * (hw2 + (p.x_2() - maxp).powi(2))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maxp() {
        let peak = PeakStencilData::from_data(4., 8., 12., 5., 10., 5.);
        let maxp = FitterAnalytical::maximum_position(&peak);
        assert_eq!(maxp, 8.);
    }

    #[test]
    fn hw2() {
        let peak = PeakStencilData::from_data(4., 8., 12., 5., 10., 5.);
        let maxp = 8.;
        let hw2 = FitterAnalytical::half_width2(&peak, maxp);
        assert_eq!(hw2.sqrt(), 4.);
    }

    #[test]
    fn sfhw() {
        let peak = PeakStencilData::from_data(4., 8., 12., 5., 10., 5.);
        let maxp = 8.;
        let hw2 = 16.;
        let sfhw = FitterAnalytical::scale_factor_half_width(&peak, maxp, hw2);
        assert_eq!(sfhw / hw2.sqrt(), 40.);
    }

    #[test]
    fn approximations() {
        let peak = PeakStencilData::from_data(4., 8., 12., 5., 10., 5.);
        let maxp = FitterAnalytical::maximum_position(&peak);
        let hw2 = FitterAnalytical::half_width2(&peak, maxp);
        let sfhw = FitterAnalytical::scale_factor_half_width(&peak, maxp, hw2);
        assert_eq!(maxp, 8.);
        assert_eq!(hw2.sqrt(), 4.);
        assert_eq!(sfhw / hw2.sqrt(), 40.);
    }
}
