#[cfg(feature = "parallel")]
use rayon::prelude::*;

/// Data structure that represents a [Lorentzian function].
///
/// # Definition
///
/// The [Lorentzian function] is typically defined as
///
/// ```text
/// f(x) = 1/pi * gamma / ((x - x0)^2 + gamma^2)
/// ```
///
/// `gamma` is the half width at half maximum (`hw` from here on) and `x0` is
/// the position of the maximum (`maxp` from here on). The scale factor 1/pi is
/// chosen to make the integral of the function equal to 1. In order to fit the
/// function to data, an additional scale factor `sf` is introduced, which
/// replaces the 1/pi factor, resulting in the following expression:
///
/// ```text
/// f(x) = sf * hw / (hw^2 + (x - maxp)^2)
/// ```
///
/// However, this form is unwieldy for solving a system of equations, due to the
/// `hw` appearing in both the numerator and the denominator as a square. To
/// simplify the problem, the following transformation is introduced:
///
/// ```text
/// sfhw = sf * hw
/// hw2 = hw^2
/// ```
///
/// The [Lorentzian function] can then be expressed as
///
/// ```text
/// f(x) = sfhw / (hw2 + (x - maxp)^2)
/// ```
///
/// which is the form used internally in this implementation as it avoids the
/// product and the square when evaluating the function, as well as the square
/// root and division when solving the system of equations.
///
/// [Lorentzian function]: https://en.wikipedia.org/wiki/Cauchy_distribution
///
/// # Example
///
/// The following serves as a basic introduction to this data structure. It is,
/// however, usually not very useful to create these manually. Though, this is a
/// simple method to generate some synthetic data.
///
/// ```
/// use float_cmp::assert_approx_eq;
/// use metabodecon::deconvolution::Lorentzian;
///
/// // Signal centered at 5 ppm with maximum intensity 2.0 and hw 0.15.
/// // sfhw = max_intensity * hw^2 = 2.0 * 0.15^2 = 0.045
/// // hw2 = hw^2 = 0.15^2 = 0.0225
/// // maxp = 5.0
/// let lorentzian = Lorentzian::new(0.045, 0.0225, 5.0);
///
/// // Evaluate the Lorentzian at the maximum position.
/// assert_approx_eq!(f64, lorentzian.evaluate(5.0), 2.0);
///
/// // Use the untransformed parameters.
/// let (sf, hw, maxp) = lorentzian.retransformed_parameters();
/// // sf = sfhw / sqrt(hw2) = 0.045 / 0.15 = 0.3
/// assert_approx_eq!(f64, sf, 0.3);
/// // hw = sqrt(hw2) = 0.15
/// assert_approx_eq!(f64, hw, 0.15);
/// // maxp unchanged
/// assert_approx_eq!(f64, maxp, 5.0);
///
/// // Generate 100 chemical shifts between 0.0 and 10.0 ppm.
/// let chemical_shifts = (0..100)
///     .map(|x| x as f64 * 10.0 / 99.0)
///     .collect::<Vec<f64>>();
///
/// // Evaluate the Lorentzian at the chemical shifts.
/// let intensities = lorentzian.evaluate_vec(&chemical_shifts);
///
/// // Get the integral of the Lorentzian.
/// let integral = lorentzian.integral();
/// assert_approx_eq!(f64, lorentzian.sf() * std::f64::consts::PI, integral);
///
/// // Create a peak triplet centered at 5 ppm with shorter side peaks.
/// let triplet = [
///     Lorentzian::new(0.03, 0.0009, 4.8),
///     Lorentzian::new(0.02, 0.0004, 5.0),
///     Lorentzian::new(0.03, 0.0009, 5.2),
/// ];
///
/// // Evaluate the superposition of the Lorentzians at the maximum.
/// assert_approx_eq!(
///     f64,
///     Lorentzian::superposition(5.0, &triplet),
///     51.466992,
///     epsilon = 1e-6
/// );
///
/// // Evaluate the superposition of the Lorentzians at the chemical shifts.
/// let sup1 = Lorentzian::superposition_vec(&chemical_shifts, &triplet);
/// // ...in parallel (not efficient for small numbers of points and peaks).
/// let sup2 = Lorentzian::par_superposition_vec(&chemical_shifts, &triplet);
/// ```
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
    ///
    /// Uses a transformation of the [Lorentzian function] to simplify the
    /// evaluation of the function. [Read more](Lorentzian)
    ///
    /// ```text
    /// f(x) = sfhw / (hw2 + (x - maxp)^2)
    /// ```
    ///
    /// * `sfhw` is the scale factor multiplied by the half width at half
    ///   maximum
    /// * `hw2` is the half width at half maximum squared
    /// * `maxp` is the position of the maximum (unchanged)
    ///
    /// [Lorentzian function]: https://en.wikipedia.org/wiki/Cauchy_distribution
    ///
    /// # Example
    ///
    /// ```
    /// use metabodecon::deconvolution::Lorentzian;
    ///
    /// // Signal centered at 5 ppm with maximum intensity 2.0 and hw 0.15.
    /// // sfhw = max_intensity * hw^2 = 2.0 * 0.15^2 = 0.045
    /// // hw2 = hw^2 = 0.15^2 = 0.0225
    /// // maxp = 5.0
    /// let lorentzian = Lorentzian::new(0.045, 0.0225, 5.0);
    ///
    /// // Signal centered at 0 ppm with maximum intensity 30.0 and hw 0.5.
    /// // sfhw = max_intensity * hw^2 = 30.0 * 0.5^2 = 7.5
    /// // hw2 = hw^2 = 0.5^2 = 0.25
    /// // maxp = 0.0
    /// let lorentzian = Lorentzian::new(7.5, 0.25, 0.0);
    /// ```
    pub fn new(sfhw: f64, hw2: f64, maxp: f64) -> Self {
        Self {
            scale_factor_half_width: sfhw,
            half_width_squared: hw2,
            maximum_position: maxp,
        }
    }

    /// Returns the scale factor multiplied by the half width.
    ///
    /// This is part of the interface for the transformed parameters. By itself,
    /// `sfhw` does not provide much information, as there are infinitely many
    /// combinations of scale factors and half widths that result in the same
    /// `sfhw`. See [`sf`] and [`hw`] for the untransformed parameters.
    /// [Read more](Lorentzian)
    ///
    /// [`sf`]: Lorentzian::sf
    /// [`hw`]: Lorentzian::hw
    ///
    /// # Example
    ///
    /// ```
    /// use float_cmp::assert_approx_eq;
    /// use metabodecon::deconvolution::Lorentzian;
    ///
    /// // Signal centered at 5 ppm with maximum intensity 2.0 and hw 0.15.
    /// // sfhw = max_intensity * hw^2 = 2.0 * 0.15^2 = 0.045
    /// // hw2 = hw^2 = 0.15^2 = 0.0225
    /// // maxp = 5.0
    /// let lorentzian = Lorentzian::new(0.045, 0.0225, 5.0);
    ///
    /// assert_approx_eq!(f64, lorentzian.sfhw(), 0.045);
    /// ```
    pub fn sfhw(&self) -> f64 {
        self.scale_factor_half_width
    }

    /// Returns the half width squared.
    ///
    /// This is part of the interface for the transformed parameters. See [`hw`]
    /// for the untransformed parameter. [Read more](Lorentzian)
    ///
    /// [`hw`]: Lorentzian::hw
    ///
    /// # Example
    ///
    /// ```
    /// use float_cmp::assert_approx_eq;
    /// use metabodecon::deconvolution::Lorentzian;
    ///
    /// // Signal centered at 5 ppm with maximum intensity 2.0 and hw 0.15.
    /// // sfhw = max_intensity * hw^2 = 2.0 * 0.15^2 = 0.045
    /// // hw2 = hw^2 = 0.15^2 = 0.0225
    /// // maxp = 5.0
    /// let lorentzian = Lorentzian::new(0.045, 0.0225, 5.0);
    ///
    /// assert_approx_eq!(f64, lorentzian.hw2(), 0.0225);
    /// ```
    pub fn hw2(&self) -> f64 {
        self.half_width_squared
    }

    /// Returns the position of the maximum.
    ///
    /// # Example
    ///
    /// ```
    /// use float_cmp::assert_approx_eq;
    /// use metabodecon::deconvolution::Lorentzian;
    ///
    /// // Signal centered at 5 ppm with maximum intensity 2.0 and hw 0.15.
    /// // sfhw = max_intensity * hw^2 = 2.0 * 0.15^2 = 0.045
    /// // hw2 = hw^2 = 0.15^2 = 0.0225
    /// // maxp = 5.0
    /// let lorentzian = Lorentzian::new(0.045, 0.0225, 5.0);
    ///
    /// assert_approx_eq!(f64, lorentzian.maxp(), 5.0);
    /// ```
    pub fn maxp(&self) -> f64 {
        self.maximum_position
    }

    /// Returns the parameters of the `Lorentzian` as a tuple.
    ///
    /// This is part of the interface for the transformed parameters. See
    /// [`retransformed_parameters`] for the untransformed parameters.
    /// [Read more](Lorentzian)
    ///
    /// [`retransformed_parameters`]: Lorentzian::retransformed_parameters
    ///
    /// # Example
    ///
    /// ```
    /// use float_cmp::assert_approx_eq;
    /// use metabodecon::deconvolution::Lorentzian;
    ///
    /// // Signal centered at 5 ppm with maximum intensity 2.0 and hw 0.15.
    /// // sfhw = max_intensity * hw^2 = 2.0 * 0.15^2 = 0.045
    /// // hw2 = hw^2 = 0.15^2 = 0.0225
    /// // maxp = 5.0
    /// let lorentzian = Lorentzian::new(0.045, 0.0225, 5.0);
    ///
    /// let (sfhw, hw2, maxp) = lorentzian.parameters();
    /// assert_approx_eq!(f64, sfhw, 0.045);
    /// assert_approx_eq!(f64, hw2, 0.0225);
    /// assert_approx_eq!(f64, maxp, 5.0);
    /// ```
    pub fn parameters(&self) -> (f64, f64, f64) {
        (self.sfhw(), self.hw2(), self.maxp())
    }

    /// Sets the scale factor multiplied by the half width.
    ///
    /// This is part of the interface for the transformed parameters. Modifies
    /// the scale factor in an indirect way. See [`set_sf`] to modify the scale
    /// factor directly. [Read more](Lorentzian)
    ///
    /// [`set_sf`]: Lorentzian::set_sf
    /// [`set_hw`]: Lorentzian::set_hw
    ///
    /// # Example
    ///
    /// ```
    /// use float_cmp::assert_approx_eq;
    /// use metabodecon::deconvolution::Lorentzian;
    ///
    /// // Signal centered at 5 ppm with maximum intensity 2.0 and hw 0.15.
    /// // sfhw = max_intensity * hw^2 = 2.0 * 0.15^2 = 0.045
    /// // hw2 = hw^2 = 0.15^2 = 0.0225
    /// // maxp = 5.0
    /// let mut lorentzian = Lorentzian::new(0.045, 0.0225, 5.0);
    ///
    /// // Set sfhw to 0.0675, maximum intensity is now 3.0.
    /// // max_intensity = sf / hw = sfhw / hw2 = 0.0675 / 0.0225 = 3.0
    /// lorentzian.set_sfhw(0.0675);
    /// assert_approx_eq!(f64, lorentzian.sfhw(), 0.0675);
    /// assert_approx_eq!(f64, lorentzian.sfhw() / lorentzian.hw2(), 3.0);
    /// ```
    pub fn set_sfhw(&mut self, sfhw: f64) {
        self.scale_factor_half_width = sfhw;
    }

    /// Sets the half width squared.
    ///
    /// This is part of the interface for the transformed parameters. Modifies
    /// the half width in an indirect way, which also modifies the scale factor
    /// as a side effect due to the nature of the transformation. See [`set_hw`]
    /// to modify the half width directly without side effects.
    /// [Read more](Lorentzian)
    ///
    /// [`set_hw`]: Lorentzian::set_hw
    ///
    /// # Example
    ///
    /// ```
    /// use float_cmp::assert_approx_eq;
    /// use metabodecon::deconvolution::Lorentzian;
    ///
    /// // Signal centered at 5 ppm with maximum intensity 2.0 and hw 0.15.
    /// // sfhw = max_intensity * hw^2 = 2.0 * 0.15^2 = 0.045
    /// // hw2 = hw^2 = 0.15^2 = 0.0225
    /// // maxp = 5.0
    /// let mut lorentzian = Lorentzian::new(0.045, 0.0225, 5.0);
    ///
    /// // Set hw2 to 0.03, maximum intensity is now 1.5.
    /// // max_intensity = sf / hw = sfhw / hw2 = 0.045 / 0.03 = 1.5
    /// lorentzian.set_hw2(0.03);
    /// assert_approx_eq!(f64, lorentzian.hw2(), 0.03);
    /// assert_approx_eq!(f64, lorentzian.sfhw() / lorentzian.hw2(), 1.5);
    /// ```
    pub fn set_hw2(&mut self, hw2: f64) {
        self.half_width_squared = hw2;
    }

    /// Sets the position of the maximum.
    ///
    /// # Example
    ///
    /// ```
    /// use float_cmp::assert_approx_eq;
    /// use metabodecon::deconvolution::Lorentzian;
    ///
    /// // Signal centered at 5 ppm with maximum intensity 2.0 and hw 0.15.
    /// // sfhw = max_intensity * hw^2 = 2.0 * 0.15^2 = 0.045
    /// // hw2 = hw^2 = 0.15^2 = 0.0225
    /// // maxp = 5.0
    /// let mut lorentzian = Lorentzian::new(0.045, 0.0225, 5.0);
    ///
    /// // Set maxp to -5.0.
    /// lorentzian.set_maxp(-5.0);
    /// assert_approx_eq!(f64, lorentzian.maxp(), -5.0);
    /// ```
    pub fn set_maxp(&mut self, max_position: f64) {
        self.maximum_position = max_position;
    }

    /// Sets the parameters of the `Lorentzian`.
    ///
    /// This is part of the interface for the transformed parameters. See
    /// [`set_retransformed_parameters`] to set the parameters using their
    /// untransformed representation. [Read more](Lorentzian)
    ///
    /// [`set_retransformed_parameters`]: Lorentzian::set_retransformed_parameters
    ///
    /// # Example
    ///
    /// ```
    /// use float_cmp::assert_approx_eq;
    /// use metabodecon::deconvolution::Lorentzian;
    ///
    /// // Signal centered at 5 ppm with maximum intensity 2.0 and hw 0.15.
    /// // sfhw = max_intensity * hw^2 = 2.0 * 0.15^2 = 0.045
    /// // hw2 = hw^2 = 0.15^2 = 0.0225
    /// // maxp = 5.0
    /// let mut lorentzian = Lorentzian::new(0.045, 0.0225, 5.0);
    ///
    /// // Set maximum intensity to 3.0, hw to 0.2, and shift maximum to -5.0.
    /// // sfhw = max_intensity * hw^2 = 3.0 * 0.2^2 = 0.12
    /// // hw2 = hw^2 = 0.2^2 = 0.04
    /// // maxp = -5.0
    /// lorentzian.set_parameters(0.12, 0.04, -5.0);
    /// assert_approx_eq!(f64, lorentzian.sfhw(), 0.12);
    /// assert_approx_eq!(f64, lorentzian.hw2(), 0.04);
    /// assert_approx_eq!(f64, lorentzian.maxp(), -5.0);
    /// assert_approx_eq!(f64, lorentzian.sfhw() / lorentzian.hw2(), 3.0);
    /// ```
    pub fn set_parameters(&mut self, sfhw: f64, hw2: f64, maxp: f64) {
        self.scale_factor_half_width = sfhw;
        self.half_width_squared = hw2;
        self.maximum_position = maxp;
    }

    /// Undoes the transformation and returns the scale factor.
    ///
    /// Computing the scale factor has some overhead, as it requires a division
    /// and a square root to undo the transformation. [Read more](Lorentzian)
    ///
    /// # Example
    ///
    /// ```
    /// use float_cmp::assert_approx_eq;
    /// use metabodecon::deconvolution::Lorentzian;
    ///
    /// // Signal centered at 5 ppm with maximum intensity 2.0 and hw 0.15.
    /// // sfhw = max_intensity * hw^2 = 2.0 * 0.15^2 = 0.045
    /// // hw2 = hw^2 = 0.15^2 = 0.0225
    /// // maxp = 5.0
    /// let lorentzian = Lorentzian::new(0.045, 0.0225, 5.0);
    ///
    /// // sf = sfhw / sqrt(hw2)
    /// assert_approx_eq!(f64, lorentzian.sf(), 0.3);
    /// ```
    pub fn sf(&self) -> f64 {
        self.scale_factor_half_width / self.hw()
    }

    /// Undoes the transformation and returns the half width.
    ///
    /// Computing the half width has some overhead, as it requires a square root
    /// to undo the transformation. [Read more](Lorentzian)
    ///
    /// # Example
    ///
    /// ```
    /// use float_cmp::assert_approx_eq;
    /// use metabodecon::deconvolution::Lorentzian;
    ///
    /// // Signal centered at 5 ppm with maximum intensity 2.0 and hw 0.15.
    /// // sfhw = max_intensity * hw^2 = 2.0 * 0.15^2 = 0.045
    /// // hw2 = hw^2 = 0.15^2 = 0.0225
    /// // maxp = 5.0
    /// let lorentzian = Lorentzian::new(0.045, 0.0225, 5.0);
    ///
    /// // hw = sqrt(hw2)
    /// assert_approx_eq!(f64, lorentzian.hw(), 0.15);
    pub fn hw(&self) -> f64 {
        self.half_width_squared.sqrt()
    }

    /// Undoes the transformation and returns the parameters as a tuple.
    ///
    /// Computing the untransformed parameters has some overhead, as it requires
    /// a division and a square root to undo the transformation.
    /// [Read more](Lorentzian)
    ///
    /// # Example
    ///
    /// ```
    /// use float_cmp::assert_approx_eq;
    /// use metabodecon::deconvolution::Lorentzian;
    ///
    /// // Signal centered at 5 ppm with maximum intensity 2.0 and hw 0.15.
    /// // sfhw = max_intensity * hw^2 = 2.0 * 0.15^2 = 0.045
    /// // hw2 = hw^2 = 0.15^2 = 0.0225
    /// // maxp = 5.0
    /// let lorentzian = Lorentzian::new(0.045, 0.0225, 5.0);
    ///
    /// let (sf, hw, maxp) = lorentzian.retransformed_parameters();
    /// // sf = sfhw / sqrt(hw2)
    /// assert_approx_eq!(f64, sf, 0.3);
    /// // hw = sqrt(hw2)
    /// assert_approx_eq!(f64, hw, 0.15);
    /// // unchanged.
    /// assert_approx_eq!(f64, maxp, 5.0);
    /// ```
    pub fn retransformed_parameters(&self) -> (f64, f64, f64) {
        (self.sf(), self.hw(), self.maxp())
    }

    /// Sets the scale factor.
    ///
    /// This has some overhead, as it requires a square root to undo the
    /// transformation of the half width, which is needed to update the
    /// internal representation of the parameters. [Read more](Lorentzian)
    ///
    /// # Example
    ///
    /// ```
    /// use float_cmp::assert_approx_eq;
    /// use metabodecon::deconvolution::Lorentzian;
    ///
    /// // Signal centered at 5 ppm with maximum intensity 2.0 and hw 0.15.
    /// // sfhw = max_intensity * hw^2 = 2.0 * 0.15^2 = 0.045
    /// // hw2 = hw^2 = 0.15^2 = 0.0225
    /// // maxp = 5.0
    /// let mut lorentzian = Lorentzian::new(0.045, 0.0225, 5.0);
    ///
    /// // Set sf to 0.45, which increases the maximum intensity to 3.0.
    /// lorentzian.set_sf(0.45);
    /// assert_approx_eq!(f64, lorentzian.sf(), 0.45);
    /// assert_approx_eq!(f64, lorentzian.sf() / lorentzian.hw(), 3.0);
    /// ```
    pub fn set_sf(&mut self, sf: f64) {
        self.scale_factor_half_width = sf * self.hw();
    }

    /// Sets the half width.
    ///
    /// This has some overhead, as it requires a division and a square root to
    /// update the internal representation of the parameters.
    /// [Read more](Lorentzian)
    ///
    /// # Example
    ///
    /// ```
    /// use float_cmp::assert_approx_eq;
    /// use metabodecon::deconvolution::Lorentzian;
    ///
    /// // Signal centered at 5 ppm with maximum intensity 2.0 and hw 0.15.
    /// // sfhw = max_intensity * hw^2 = 2.0 * 0.15^2 = 0.045
    /// // hw2 = hw^2 = 0.15^2 = 0.0225
    /// // maxp = 5.0
    /// let mut lorentzian = Lorentzian::new(0.045, 0.0225, 5.0);
    ///
    /// // Set hw to 0.30, which halves the maximum intensity.
    /// lorentzian.set_hw(0.30);
    /// assert_approx_eq!(f64, lorentzian.hw(), 0.30);
    /// assert_approx_eq!(f64, lorentzian.sf() / lorentzian.hw(), 1.0);
    /// ```
    pub fn set_hw(&mut self, hw: f64) {
        self.scale_factor_half_width = self.sf() * hw;
        self.half_width_squared = hw.powi(2);
    }

    /// Sets the parameters of the `Lorentzian`.
    ///
    /// This has some overhead, as it requires a division and a square root to
    /// update the internal representation of the parameters.
    /// [Read more](Lorentzian)
    ///
    /// # Example
    ///
    /// ```
    /// use float_cmp::assert_approx_eq;
    /// use metabodecon::deconvolution::Lorentzian;
    ///
    /// // Signal centered at 5 ppm with maximum intensity 2.0 and hw 0.15.
    /// // sfhw = max_intensity * hw^2 = 2.0 * 0.15^2 = 0.045
    /// // hw2 = hw^2 = 0.15^2 = 0.0225
    /// // maxp = 5.0
    /// let mut lorentzian = Lorentzian::new(0.045, 0.0225, 5.0);
    ///
    /// // Set maximum intensity to 3.0, hw to 0.2, and shift maximum to -5.0.
    /// // sf = max_intensity * hw = 3.0 * 0.2 = 0.6
    /// lorentzian.set_retransformed_parameters(0.6, 0.2, -5.0);
    /// assert_approx_eq!(f64, lorentzian.sf(), 0.6);
    /// assert_approx_eq!(f64, lorentzian.hw(), 0.2);
    /// assert_approx_eq!(f64, lorentzian.maxp(), -5.0);
    /// assert_approx_eq!(f64, lorentzian.sf() / lorentzian.hw(), 3.0);
    /// ```
    pub fn set_retransformed_parameters(&mut self, sf: f64, hw: f64, maxp: f64) {
        self.scale_factor_half_width = sf * hw;
        self.half_width_squared = hw.powi(2);
        self.maximum_position = maxp;
    }

    /// Evaluates the `Lorentzian` at the given position `x`.
    ///
    /// # Example
    ///
    /// ```
    /// use float_cmp::assert_approx_eq;
    /// use metabodecon::deconvolution::Lorentzian;
    ///
    /// // Signal centered at 5 ppm with maximum intensity 2.0 and hw 0.15.
    /// // sfhw = max_intensity * hw^2 = 2.0 * 0.15^2 = 0.045
    /// // hw2 = hw^2 = 0.15^2 = 0.0225
    /// // maxp = 5.0
    /// let lorentzian = Lorentzian::new(0.045, 0.0225, 5.0);
    ///
    /// // Evaluate the Lorentzian at the maximum position.
    /// assert_approx_eq!(f64, lorentzian.evaluate(5.0), 2.0);
    /// ```
    pub fn evaluate(&self, x: f64) -> f64 {
        self.scale_factor_half_width
            / (self.half_width_squared + (x - self.maximum_position).powi(2))
    }

    /// Evaluates the `Lorentzian` at the given positions `x`.
    ///
    /// # Example
    ///
    /// ```
    /// use metabodecon::deconvolution::Lorentzian;
    ///
    /// // Signal centered at 5 ppm with maximum intensity 2.0 and hw 0.15.
    /// // sfhw = max_intensity * hw^2 = 2.0 * 0.15^2 = 0.045
    /// // hw2 = hw^2 = 0.15^2 = 0.0225
    /// // maxp = 5.0
    /// let lorentzian = Lorentzian::new(0.045, 0.0225, 5.0);
    ///
    /// // Generate 100 chemical shifts between 0.0 and 10.0 ppm.
    /// let chemical_shifts = (0..100)
    ///     .map(|x| x as f64 * 10.0 / 99.0)
    ///     .collect::<Vec<f64>>();
    ///
    /// // Evaluate the Lorentzian at the chemical shifts.
    /// let intensities = lorentzian.evaluate_vec(&chemical_shifts);
    /// ```
    pub fn evaluate_vec(&self, x: &[f64]) -> Vec<f64> {
        x.iter().map(|&x| self.evaluate(x)).collect()
    }

    /// Returns the integral of the `Lorentzian` from negative infinity to
    /// positive infinity.
    ///
    /// # Example
    ///
    /// ```
    /// use float_cmp::assert_approx_eq;
    /// use metabodecon::deconvolution::Lorentzian;
    ///
    /// // Signal centered at 5 ppm with maximum intensity 2.0 and hw 0.15.
    /// // sfhw = max_intensity * hw^2 = 2.0 * 0.15^2 = 0.045
    /// // hw2 = hw^2 = 0.15^2 = 0.0225
    /// // maxp = 5.0
    /// let lorentzian = Lorentzian::new(0.045, 0.0225, 5.0);
    ///
    /// // Get the integral of the Lorentzian.
    /// let integral = lorentzian.integral();
    /// assert_approx_eq!(f64, lorentzian.sf() * std::f64::consts::PI, integral);
    /// ```
    pub fn integral(&self) -> f64 {
        std::f64::consts::PI * self.sf()
    }

    /// Evaluates the superposition of the given `Lorentzian`s at the given
    /// position `x`.
    ///
    /// # Example
    ///
    /// ```
    /// use float_cmp::assert_approx_eq;
    /// use metabodecon::deconvolution::Lorentzian;
    ///
    /// // Create a peak triplet centered at 5 ppm with shorter side peaks.
    /// let triplet = [
    ///     Lorentzian::new(0.03, 0.0009, 4.8),
    ///     Lorentzian::new(0.02, 0.0004, 5.0),
    ///     Lorentzian::new(0.03, 0.0009, 5.2),
    /// ];
    ///
    /// // Evaluate the superposition of the Lorentzians at the maximum.
    /// assert_approx_eq!(
    ///     f64,
    ///     Lorentzian::superposition(5.0, &triplet),
    ///     51.466992,
    ///     epsilon = 1e-6
    /// );
    /// ```
    pub fn superposition(x: f64, lorentzians: &[Self]) -> f64 {
        lorentzians.iter().map(|l| l.evaluate(x)).sum()
    }

    /// Evaluates the superposition of the given `Lorentzian`s at the given
    /// positions `x`.
    ///
    /// # Example
    ///
    /// ```
    /// use metabodecon::deconvolution::Lorentzian;
    ///
    /// // Create a peak triplet centered at 5 ppm with shorter side peaks.
    /// let triplet = [
    ///     Lorentzian::new(0.03, 0.0009, 4.8),
    ///     Lorentzian::new(0.02, 0.0004, 5.0),
    ///     Lorentzian::new(0.03, 0.0009, 5.2),
    /// ];
    ///
    /// // Generate 100 chemical shifts between 0.0 and 10.0 ppm.
    /// let chemical_shifts = (0..100)
    ///     .map(|x| x as f64 * 10.0 / 99.0)
    ///     .collect::<Vec<f64>>();
    ///
    /// // Evaluate the superposition of the Lorentzians at the chemical shifts.
    /// let sup = Lorentzian::superposition_vec(&chemical_shifts, &triplet);
    /// ```
    pub fn superposition_vec(x: &[f64], lorentzians: &[Self]) -> Vec<f64> {
        x.iter()
            .map(|&x| Self::superposition(x, lorentzians))
            .collect()
    }

    /// Evaluates the superposition of the given `Lorentzian`s at the given
    /// position `x` in parallel using Rayon.
    ///
    /// # Example
    ///
    /// ```
    /// use metabodecon::deconvolution::Lorentzian;
    ///
    /// // Create a peak triplet centered at 5 ppm with shorter side peaks.
    /// let triplet = [
    ///     Lorentzian::new(0.03, 0.0009, 4.8),
    ///     Lorentzian::new(0.02, 0.0004, 5.0),
    ///     Lorentzian::new(0.03, 0.0009, 5.2),
    /// ];
    ///
    /// // Generate 100 chemical shifts between 0.0 and 10.0 ppm.
    /// let chemical_shifts = (0..100)
    ///     .map(|x| x as f64 * 10.0 / 99.0)
    ///     .collect::<Vec<f64>>();
    ///
    /// // Evaluate the superposition of the Lorentzians at the chemical shifts.
    /// let sup = Lorentzian::par_superposition_vec(&chemical_shifts, &triplet);
    /// ```
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
    use float_cmp::assert_approx_eq;

    #[test]
    fn accessors() {
        let lorentzian = Lorentzian::new(1.0, 0.25, 0.0);
        assert_approx_eq!(f64, lorentzian.sfhw(), 1.0);
        assert_approx_eq!(f64, lorentzian.hw2(), 0.25);
        assert_approx_eq!(f64, lorentzian.maxp(), 0.0);
        assert_approx_eq!(f64, lorentzian.sf(), 2.0);
        assert_approx_eq!(f64, lorentzian.hw(), 0.5);
    }

    #[test]
    fn mutators() {
        let mut lorentzian = Lorentzian::new(1.0, 1.0, 0.0);
        lorentzian.set_sfhw(1.5);
        lorentzian.set_hw2(2.25);
        lorentzian.set_maxp(1.0);
        assert_approx_eq!(f64, lorentzian.sfhw(), 1.5);
        assert_approx_eq!(f64, lorentzian.hw2(), 2.25);
        assert_approx_eq!(f64, lorentzian.maxp(), 1.0);
        assert_approx_eq!(f64, lorentzian.sf(), 1.0);
        assert_approx_eq!(f64, lorentzian.hw(), 1.5);
        lorentzian.set_parameters(1.0, 1.0, 0.0);
        assert_approx_eq!(f64, lorentzian.sfhw(), 1.0);
        assert_approx_eq!(f64, lorentzian.hw2(), 1.0);
        assert_approx_eq!(f64, lorentzian.maxp(), 0.0);
        assert_approx_eq!(f64, lorentzian.sf(), 1.0);
        assert_approx_eq!(f64, lorentzian.hw(), 1.0);
    }

    #[test]
    fn evaluate() {
        let lorentzian = Lorentzian::new(1.0, 1.0, 0.0);
        let chemical_shifts = (0..11)
            .map(|x| -5.0 + x as f64)
            .collect::<Vec<f64>>();
        let expected_intensities = [
            1.0 / 26.0,
            1.0 / 17.0,
            1.0 / 10.0,
            1.0 / 5.0,
            1.0 / 2.0,
            1.0 / 1.0,
            1.0 / 2.0,
            1.0 / 5.0,
            1.0 / 10.0,
            1.0 / 17.0,
            1.0 / 26.0,
        ];
        chemical_shifts
            .iter()
            .zip(expected_intensities.iter())
            .for_each(|(&x, &y)| {
                assert_approx_eq!(f64, lorentzian.evaluate(x), y);
            });
        let computed_intensities = lorentzian.evaluate_vec(&chemical_shifts);
        computed_intensities
            .iter()
            .zip(expected_intensities.iter())
            .for_each(|(&yc, &ye)| {
                assert_approx_eq!(f64, yc, ye);
            });
    }

    #[test]
    fn superposition() {
        let lorentzians = vec![
            Lorentzian::new(1.0, 0.5, -2.0),
            Lorentzian::new(2.0, 0.75, 0.0),
            Lorentzian::new(1.0, 0.5, 2.0),
        ];
        let chemical_shifts = (0..11)
            .map(|x| -5.0 + x as f64)
            .collect::<Vec<f64>>();
        let expected_intensities = [
            1.0 / 9.5 + 2.0 / 25.75 + 1.0 / 49.5,
            1.0 / 4.5 + 2.0 / 16.75 + 1.0 / 36.5,
            1.0 / 1.5 + 2.0 / 9.75 + 1.0 / 25.5,
            1.0 / 0.5 + 2.0 / 4.75 + 1.0 / 16.5,
            1.0 / 1.5 + 2.0 / 1.75 + 1.0 / 9.5,
            1.0 / 4.5 + 2.0 / 0.75 + 1.0 / 4.5,
            1.0 / 9.5 + 2.0 / 1.75 + 1.0 / 1.5,
            1.0 / 16.5 + 2.0 / 4.75 + 1.0 / 0.5,
            1.0 / 25.5 + 2.0 / 9.75 + 1.0 / 1.5,
            1.0 / 36.5 + 2.0 / 16.75 + 1.0 / 4.5,
            1.0 / 49.5 + 2.0 / 25.75 + 1.0 / 9.5,
        ];
        chemical_shifts
            .iter()
            .zip(expected_intensities.iter())
            .for_each(|(&x, &y)| {
                assert_approx_eq!(f64, Lorentzian::superposition(x, &lorentzians), y);
            });
        let computed_intensities = Lorentzian::superposition_vec(&chemical_shifts, &lorentzians);
        computed_intensities
            .iter()
            .zip(expected_intensities.iter())
            .for_each(|(&yc, &ye)| {
                assert_approx_eq!(f64, yc, ye);
            });
    }
}
