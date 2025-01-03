#[cfg(feature = "parallel")]
use rayon::prelude::*;

/// Data structure that represents a [Lorentzian function].
///
/// The [Lorentzian function] is typically defined as
///
/// ```text
/// f(x) = 1/pi * gamma / ((x - x0)^2 + gamma^2)
/// ```
///
/// `gamma` is the half width at half maximum (HWHM) and `x0` is the position of
/// the maximum. The scale factor 1/pi is chosen to make the integral of the
/// function equal to 1. In order to fit the function to data, an additional
/// scale factor `A` is introduced, which replaces the 1/pi factor, resulting in
/// the following expression:
///
/// ```text
/// f(x) = A * gamma / ((x - x0)^2 + gamma^2)
/// ```
///
/// However, this form is unwieldy for solving a system of equations, due to the
/// HWHM appearing in both the numerator and the denominator as a square. To
/// simplify the problem, the following transformation is introduced:
///
/// ```text
/// sfhw = A * gamma
/// hw2 = gamma^2
/// ```
///
/// The [Lorentzian function] can then be expressed as
///
/// ```text
/// f(x) = sfhw / (hw2 + (x - x0)^2)
/// ```
///
/// which is the form used internally in this implementation.
///
/// [Lorentzian function]: https://en.wikipedia.org/wiki/Cauchy_distribution
#[derive(Copy, Clone, Debug, Default)]
pub struct Lorentzian {
    /// Scale factor multiplied by the half width.
    scale_factor_half_width: f64,
    /// Half width squared.
    half_width_squared: f64,
    /// Position of the maximum.
    maximum_position: f64,
}

impl Lorentzian {
    /// Constructs a new `Lorentzian` from the given parameters.
    pub fn new(sfhw: f64, hw2: f64, maxp: f64) -> Self {
        Self {
            scale_factor_half_width: sfhw,
            half_width_squared: hw2,
            maximum_position: maxp,
        }
    }

    /// Returns the scale factor multiplied by the half width.
    pub fn sfhw(&self) -> f64 {
        self.scale_factor_half_width
    }

    /// Returns the half width squared.
    pub fn hw2(&self) -> f64 {
        self.half_width_squared
    }

    /// Returns the position of the maximum.
    pub fn maxp(&self) -> f64 {
        self.maximum_position
    }

    /// Returns the parameters of the `Lorentzian` as a tuple.
    pub fn parameters(&self) -> (f64, f64, f64) {
        (self.sfhw(), self.hw2(), self.maxp())
    }

    /// Sets the scale factor multiplied by the half width.
    pub fn set_sfhw(&mut self, scale_factor: f64) {
        self.scale_factor_half_width = scale_factor;
    }

    /// Sets the half width squared.
    pub fn set_hw2(&mut self, half_width: f64) {
        self.half_width_squared = half_width;
    }

    /// Sets the position of the maximum.
    pub fn set_maxp(&mut self, max_position: f64) {
        self.maximum_position = max_position;
    }

    /// Sets the parameters of the `Lorentzian`.
    pub fn set_parameters(&mut self, sfhw: f64, hw2: f64, maxp: f64) {
        self.scale_factor_half_width = sfhw;
        self.half_width_squared = hw2;
        self.maximum_position = maxp;
    }

    /// Returns the scale factor.
    pub fn sf(&self) -> f64 {
        self.scale_factor_half_width / self.hw()
    }

    /// Returns the half width.
    pub fn hw(&self) -> f64 {
        self.half_width_squared.sqrt()
    }

    /// Returns the retransformed parameters of the `Lorentzian` as a tuple.
    pub fn retransformed_parameters(&self) -> (f64, f64, f64) {
        (self.sf(), self.hw(), self.maxp())
    }

    /// Sets the scale factor
    pub fn set_sf(&mut self, sf: f64) {
        self.scale_factor_half_width = sf * self.hw();
    }

    /// Sets the half width
    pub fn set_hw(&mut self, hw: f64) {
        self.scale_factor_half_width = self.sf() * hw;
        self.half_width_squared = hw.powi(2);
    }

    /// Sets the retransformed parameters of the `Lorentzian`.
    pub fn set_retransformed_parameters(&mut self, sf: f64, hw: f64, maxp: f64) {
        self.scale_factor_half_width = sf * hw;
        self.half_width_squared = hw.powi(2);
        self.maximum_position = maxp;
    }

    /// Evaluates the `Lorentzian` at the given position `x`.
    pub fn evaluate(&self, x: f64) -> f64 {
        self.scale_factor_half_width
            / (self.half_width_squared + (x - self.maximum_position).powi(2))
    }

    /// Evaluates the `Lorentzian` at the given positions `x`.
    pub fn evaluate_vec(&self, x: &[f64]) -> Vec<f64> {
        x.iter().map(|&x| self.evaluate(x)).collect()
    }

    /// Returns the integral of the `Lorentzian`.
    pub fn integral(&self) -> f64 {
        std::f64::consts::PI * self.sf()
    }

    /// Evaluates the superposition of the given `Lorentzian`s at the given
    /// position `x`.
    pub fn superposition(x: f64, lorentzians: &[Self]) -> f64 {
        lorentzians.iter().map(|l| l.evaluate(x)).sum()
    }

    /// Evaluates the superposition of the given `Lorentzian`s at the given
    /// positions `x`.
    pub fn superposition_vec(x: &[f64], lorentzians: &[Self]) -> Vec<f64> {
        x.iter()
            .map(|&x| Self::superposition(x, lorentzians))
            .collect()
    }

    /// Evaluates the superposition of the given `Lorentzian`s at the given
    /// position `x` in parallel using Rayon.
    #[cfg(feature = "parallel")]
    pub fn par_superposition_vec(x: &[f64], lorentzians: &[Self]) -> Vec<f64> {
        x.par_iter()
            .map(|&x| Self::superposition(x, lorentzians))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accessors() {
        let lorentzian = Lorentzian::new(1.0, 1.0, 0.0);
        assert_eq!(lorentzian.sfhw(), 1.0);
        assert_eq!(lorentzian.hw2(), 1.0);
        assert_eq!(lorentzian.maxp(), 0.0);
        assert_eq!(lorentzian.parameters(), (1.0, 1.0, 0.0));
        assert_eq!(lorentzian.sf(), 1.0);
        assert_eq!(lorentzian.hw(), 1.0);
        assert_eq!(lorentzian.retransformed_parameters(), (1.0, 1.0, 0.0));
    }

    #[test]
    fn mutators() {
        let mut lorentzian = Lorentzian::new(1.0, 1.0, 0.0);
        lorentzian.set_sfhw(2.0);
        lorentzian.set_hw2(2.0);
        lorentzian.set_maxp(1.0);
        assert_eq!(lorentzian.sfhw(), 2.0);
        assert_eq!(lorentzian.hw2(), 2.0);
        assert_eq!(lorentzian.maxp(), 1.0);
        lorentzian.set_parameters(1.0, 1.0, 0.0);
        assert_eq!(lorentzian.parameters(), (1.0, 1.0, 0.0));
    }

    #[test]
    fn evaluate() {
        let lorentzian = Lorentzian::new(1.0, 1.0, 0.0);
        assert_eq!(lorentzian.evaluate(0.0), 1.0);
        assert_eq!(lorentzian.evaluate(1.0), 0.5);
        assert_eq!(lorentzian.evaluate(2.0), 0.2);
    }

    #[test]
    fn evaluate_vec() {
        let lorentzian = Lorentzian::new(1.0, 1.0, 0.0);
        assert_eq!(
            lorentzian.evaluate_vec(&[0.0, 1.0, 2.0]),
            vec![1.0, 0.5, 0.2]
        );
    }

    #[test]
    fn superposition() {
        let lorentzians = vec![
            Lorentzian::new(1.0, 1.0, 0.0),
            Lorentzian::new(1.0, 1.0, 2.0),
        ];
        assert_eq!(Lorentzian::superposition(0.0, &lorentzians), 1.2);
        assert_eq!(Lorentzian::superposition(1.0, &lorentzians), 1.0);
        assert_eq!(Lorentzian::superposition(2.0, &lorentzians), 1.2);
    }

    #[test]
    fn superposition_vec() {
        let lorentzians = vec![
            Lorentzian::new(1.0, 1.0, 0.0),
            Lorentzian::new(1.0, 1.0, 2.0),
        ];
        assert_eq!(
            Lorentzian::superposition_vec(&[0.0, 1.0, 2.0], &lorentzians),
            vec![1.2, 1.0, 1.2]
        );
    }
}
