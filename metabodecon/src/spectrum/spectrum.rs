use crate::error::Result;
use crate::smoothing::{MovingAverage, Smoother, SmoothingAlgo};
use crate::spectrum::error::{Error, Kind};

/// Represents the ordering of 1D NMR spectrum data.
///
/// Typically, 1D NMR data is ordered in `Decreasing` order of chemical shifts,
/// but this is not always the case. Additionally, it is often simpler to work
/// with the data if it is ordered in `Increasing` order, and only reorder it
/// for display purposes.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub enum Monotonicity {
    /// The data is ordered in increasing order of chemical shifts.
    #[default]
    Increasing,
    /// The data is ordered in decreasing order of chemical shifts.
    Decreasing,
}

impl Monotonicity {
    /// Helper function to determine the `Monotonicity` of 2 floating point
    /// numbers.
    ///
    /// Checks for the ordering of two floating point numbers and returns the
    /// corresponding `Some(Monotonicity)` variant. If the two numbers differ by
    /// less than 100 times the floating point precision, or are not finite
    /// numbers, or cannot be compared, `None` is returned.
    ///
    /// [`NonUniformSpacing`]: Kind::NonUniformSpacing
    pub(crate) fn from_f64s(first: f64, second: f64) -> Option<Self> {
        if f64::abs(first - second) < 100.0 * f64::EPSILON || !(first - second).is_finite() {
            return None;
        }
        match first.partial_cmp(&second) {
            Some(std::cmp::Ordering::Less) => Some(Self::Increasing),
            Some(std::cmp::Ordering::Greater) => Some(Self::Decreasing),
            _ => None,
        }
    }
}

/// Data structure that represents a 1D NMR spectrum.
///
/// `Spectrum` is a container that holds the chemical shifts, raw intensities,
/// preprocessed intensities and metadata of a 1D NMR spectrum. Preprocessed
/// intensities are empty when the `Spectrum` is created. 1D NMR spectra
/// typically contain signal free regions on both ends of the frequency range,
/// and a water artifact within the signal region.
///
/// # Example: Constructing a `Spectrum` manually
///
/// ```
/// use metabodecon::spectrum::Spectrum;
///
/// # fn main() -> metabodecon::Result<()> {
/// // Generate 2^15 chemical shifts between 0 and 10 ppm.
/// let chemical_shifts = (0..2_u32.pow(15))
///     .into_iter()
///     .map(|i| i as f64 * 10.0 / (2_f64.powi(15) - 1.0))
///     .collect::<Vec<f64>>();
///
/// // Generate intensities using 3 Lorentzian peaks.
/// let intensities = chemical_shifts
///     .iter()
///     .map(|x| {
///         // Left signal centered at 3 ppm.
///         1.0 * 0.25 / (0.25_f64.powi(2) + (x - 3.0).powi(2))
///             // Mock water artifact centered at 5 ppm (not a realistic shape).
///             + 0.1 * 0.05 / (0.05_f64.powi(2) + (x - 5.0).powi(2))
///             // Right signal centered at 7 ppm.
///             + 1.0 * 0.25 / (0.25_f64.powi(2) + (x - 7.0).powi(2))
///     })
///     .collect::<Vec<f64>>();
///
/// // Define the signal region and water artifact boundaries.
/// let signal_boundaries = (1.0, 9.0);
/// let water_boundaries = (4.5, 5.5);
///
/// // Create a Spectrum object.
/// let spectrum = Spectrum::new(
///     chemical_shifts,
///     intensities,
///     signal_boundaries,
///     water_boundaries,
/// )?;
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Debug, Default)]
pub struct Spectrum {
    /// The chemical shifts in ppm.
    chemical_shifts: Box<[f64]>,
    /// The preprocessed intensities in arbitrary units.
    intensities: Box<[f64]>,
    /// The raw intensities in arbitrary units.
    intensities_raw: Box<[f64]>,
    /// The boundaries of the signal region.
    signal_boundaries: (f64, f64),
    /// The boundaries of the water artifact.
    water_boundaries: (f64, f64),
    /// The monotonicity of the data. Used internally for validation.
    monotonicity: Monotonicity,
}

impl Spectrum {
    /// Constructs a `Spectrum` from the given data.
    ///
    /// Note that this is generally not the recommended way to create `Spectrum`
    /// objects. See [`BrukerReader`] and [`JdxReader`] for parsing 1D NMR data
    /// from Bruker TopSpin and JCAMP-DX file formats.
    ///
    /// [`BrukerReader`]: crate::spectrum::BrukerReader
    /// [`JdxReader`]: crate::spectrum::JdxReader
    ///
    /// # Errors
    ///
    /// Input data is checked for validity. Returns an [`Error::Spectrum`] if
    /// any of the checks fail. This error type contains a
    /// [`spectrum::error::Error`], which can be matched against the
    /// [`spectrum::error::Kind`] enum to handle the specific error.
    ///
    /// [`Error::Spectrum`]: crate::Error::Spectrum
    /// [`spectrum::error::Error`]: Error
    /// [`spectrum::error::Kind`]: Kind
    ///
    /// The following errors are possible if the respective check fails:
    /// - [`EmptyData`]: Either the chemical shifts or intensities are empty.
    /// - [`DataLengthMismatch`]: The lengths of the chemical shifts and
    ///   intensities do not match.
    /// - [`NonUniformSpacing`]: Some chemical shift value is not finite, or the
    ///   difference between two consecutive values is different from the prior
    ///   step size or less than 100 times the floating-point precision.
    /// - [`InvalidIntensities`]: Some intensity value is not finite.
    /// - [`MonotonicityMismatch`]: The chemical shifts, signal boundaries, and
    ///   water boundaries are not sorted in the same order or if the
    ///   monotonicity cannot be determined due to non-uniform spacing or
    ///   non-comparable values.
    /// - [`InvalidSignalBoundaries`]: The signal region boundaries are not
    ///   within the range of the chemical shifts.
    /// - [`InvalidWaterBoundaries`]: The water artifact boundaries are not
    ///   within the signal region boundaries.
    ///
    /// [`EmptyData`]: Kind::EmptyData
    /// [`DataLengthMismatch`]: Kind::DataLengthMismatch
    /// [`NonUniformSpacing`]: Kind::NonUniformSpacing
    /// [`InvalidIntensities`]: Kind::InvalidIntensities
    /// [`MonotonicityMismatch`]: Kind::MonotonicityMismatch
    /// [`InvalidSignalBoundaries`]: Kind::InvalidSignalBoundaries
    /// [`InvalidWaterBoundaries`]: Kind::InvalidWaterBoundaries
    ///
    /// # Example
    ///
    /// ```
    /// use metabodecon::spectrum::Spectrum;
    ///
    /// # fn main() -> metabodecon::Result<()> {
    /// // Generate 2^15 chemical shifts between 0 and 10 ppm.
    /// let chemical_shifts = (0..2_u32.pow(15))
    ///     .into_iter()
    ///     .map(|i| i as f64 * 10.0 / (2_f64.powi(15) - 1.0))
    ///     .collect::<Vec<f64>>();
    ///
    /// // Generate intensities using 3 Lorentzian peaks.
    /// let intensities = chemical_shifts
    ///     .iter()
    ///     .map(|x| {
    ///         // Left signal centered at 3 ppm.
    ///         1.0 * 0.25 / (0.25_f64.powi(2) + (x - 3.0).powi(2))
    ///             // Mock water artifact centered at 5 ppm (not a realistic shape).
    ///             + 0.1 * 0.05 / (0.05_f64.powi(2) + (x - 5.0).powi(2))
    ///             // Right signal centered at 7 ppm.
    ///             + 1.0 * 0.25 / (0.25_f64.powi(2) + (x - 7.0).powi(2))
    ///     })
    ///     .collect::<Vec<f64>>();
    ///
    /// // Define the signal region and water artifact boundaries.
    /// let signal_boundaries = (1.0, 9.0);
    /// let water_boundaries = (4.5, 5.5);
    ///
    /// // Create a `Spectrum`.
    /// let spectrum = Spectrum::new(
    ///     chemical_shifts,
    ///     intensities,
    ///     signal_boundaries,
    ///     water_boundaries,
    /// )?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(
        chemical_shifts: Vec<f64>,
        intensities: Vec<f64>,
        signal_boundaries: (f64, f64),
        water_boundaries: (f64, f64),
    ) -> Result<Self> {
        Self::validate_lengths(&chemical_shifts, &intensities)?;
        Self::validate_spacing(&chemical_shifts)?;
        Self::validate_intensities(&intensities)?;
        let monotonicity =
            Self::validate_monotonicity(&chemical_shifts, signal_boundaries, water_boundaries)?;
        Self::validate_boundaries(
            monotonicity,
            &chemical_shifts,
            signal_boundaries,
            water_boundaries,
        )?;

        Ok(Self {
            chemical_shifts: chemical_shifts.into_boxed_slice(),
            intensities: Box::new([]),
            intensities_raw: intensities.into_boxed_slice(),
            signal_boundaries,
            water_boundaries,
            monotonicity,
        })
    }

    /// Returns the chemical shifts of the `Spectrum` as a slice.
    ///
    /// # Example
    ///
    /// ```
    /// use float_cmp::assert_approx_eq;
    /// use metabodecon::spectrum::Spectrum;
    ///
    /// # fn main() -> metabodecon::Result<()> {
    /// let spectrum = Spectrum::new(
    ///     vec![1.0, 2.0, 3.0], // Chemical shifts
    ///     vec![1.0, 2.0, 3.0],
    ///     (1.0, 3.0),
    ///     (2.0, 2.5),
    /// )?;
    ///
    /// assert_eq!(spectrum.chemical_shifts().len(), 3);
    /// assert_approx_eq!(f64, spectrum.chemical_shifts()[0], 1.0);
    /// assert_approx_eq!(f64, spectrum.chemical_shifts()[1], 2.0);
    /// assert_approx_eq!(f64, spectrum.chemical_shifts()[2], 3.0);
    /// # Ok(())
    /// # }
    /// ```
    pub fn chemical_shifts(&self) -> &[f64] {
        &self.chemical_shifts
    }

    /// Returns the preprocessed intensities of the `Spectrum` as a slice.
    ///
    /// This field is empty when the `Spectrum` is created and gets populated
    /// after applying the [`deconvolution`] algorithm.
    ///
    /// [`deconvolution`]: crate::deconvolution
    ///
    /// # Example
    ///
    /// ```
    /// use metabodecon::spectrum::Spectrum;
    ///
    /// # fn main() -> metabodecon::Result<()> {
    /// let spectrum = Spectrum::new(
    ///     vec![1.0, 2.0, 3.0],
    ///     vec![1.0, 2.0, 3.0],
    ///     (1.0, 3.0),
    ///     (2.0, 2.5),
    /// )?;
    ///
    /// assert!(spectrum.intensities().is_empty());
    /// # Ok(())
    /// # }
    /// ```
    pub fn intensities(&self) -> &[f64] {
        &self.intensities
    }

    /// Returns the raw intensities of the `Spectrum` as a slice.
    ///
    /// # Example
    ///
    /// ```
    /// use float_cmp::assert_approx_eq;
    /// use metabodecon::spectrum::Spectrum;
    ///
    /// # fn main() -> metabodecon::Result<()> {
    /// let spectrum = Spectrum::new(
    ///     vec![1.0, 2.0, 3.0],
    ///     vec![1.0, 2.0, 3.0], // Raw intensities
    ///     (1.0, 3.0),
    ///     (2.0, 2.5),
    /// )?;
    ///
    /// assert_eq!(spectrum.intensities_raw().len(), 3);
    /// assert_approx_eq!(f64, spectrum.intensities_raw()[0], 1.0);
    /// assert_approx_eq!(f64, spectrum.intensities_raw()[1], 2.0);
    /// assert_approx_eq!(f64, spectrum.intensities_raw()[2], 3.0);
    /// # Ok(())
    /// # }
    /// ```
    pub fn intensities_raw(&self) -> &[f64] {
        &self.intensities_raw
    }

    /// Returns the signal region boundaries of the `Spectrum` as a tuple.
    ///
    /// # Example
    ///
    /// ```
    /// use float_cmp::assert_approx_eq;
    /// use metabodecon::spectrum::Spectrum;
    ///
    /// # fn main() -> metabodecon::Result<()> {
    /// let spectrum = Spectrum::new(
    ///     vec![1.0, 2.0, 3.0],
    ///     vec![1.0, 2.0, 3.0],
    ///     (1.0, 3.0), // Signal boundaries
    ///     (2.0, 2.5),
    /// )?;
    ///
    /// assert_approx_eq!(f64, spectrum.signal_boundaries().0, 1.0);
    /// assert_approx_eq!(f64, spectrum.signal_boundaries().1, 3.0);
    /// # Ok(())
    /// # }
    /// ```
    pub fn signal_boundaries(&self) -> (f64, f64) {
        self.signal_boundaries
    }

    /// Returns the water artifact boundaries as a tuple.
    ///
    /// # Example
    ///
    /// ```
    /// use float_cmp::assert_approx_eq;
    /// use metabodecon::spectrum::Spectrum;
    ///
    /// # fn main() -> metabodecon::Result<()> {
    /// let spectrum = Spectrum::new(
    ///     vec![1.0, 2.0, 3.0],
    ///     vec![1.0, 2.0, 3.0],
    ///     (1.0, 3.0),
    ///     (2.0, 2.5), // Water boundaries
    /// )?;
    ///
    /// assert_approx_eq!(f64, spectrum.water_boundaries().0, 2.0);
    /// assert_approx_eq!(f64, spectrum.water_boundaries().1, 2.5);
    /// # Ok(())
    /// # }
    /// ```
    pub fn water_boundaries(&self) -> (f64, f64) {
        self.water_boundaries
    }

    /// Returns the monotonicity of the `Spectrum`.
    ///
    /// # Example
    ///
    /// ```
    /// use metabodecon::spectrum::{Monotonicity, Spectrum};
    ///
    /// # fn main() -> metabodecon::Result<()> {
    /// let spectrum_increasing = Spectrum::new(
    ///     vec![1.0, 2.0, 3.0],
    ///     vec![1.0, 2.0, 3.0],
    ///     (1.0, 3.0),
    ///     (2.0, 2.5),
    /// )?;
    /// let spectrum_decreasing = Spectrum::new(
    ///     vec![3.0, 2.0, 1.0],
    ///     vec![3.0, 2.0, 1.0],
    ///     (3.0, 1.0),
    ///     (2.5, 2.0),
    /// )?;
    ///
    /// assert_eq!(spectrum_increasing.monotonicity(), Monotonicity::Increasing);
    /// assert_eq!(spectrum_decreasing.monotonicity(), Monotonicity::Decreasing);
    /// # Ok(())
    /// # }
    /// ```
    pub fn monotonicity(&self) -> Monotonicity {
        self.monotonicity
    }

    /// Sets the chemical shifts of the `Spectrum`.
    ///
    /// # Errors
    ///
    /// Performs the same checks as [`Spectrum::new`] on the chemical shifts,
    /// using the intensities and boundaries of the `Spectrum`.
    ///
    /// # Example
    ///
    /// ```
    /// use float_cmp::assert_approx_eq;
    /// use metabodecon::spectrum::Spectrum;
    ///
    /// # fn main() -> metabodecon::Result<()> {
    /// let mut spectrum = Spectrum::new(
    ///     vec![1.0, 2.0, 3.0], // Chemical shifts
    ///     vec![1.0, 2.0, 3.0],
    ///     (1.0, 3.0),
    ///     (2.0, 2.5),
    /// )?;
    /// spectrum.set_chemical_shifts(vec![0.0, 2.0, 4.0])?;
    ///
    /// assert_approx_eq!(f64, spectrum.chemical_shifts()[0], 0.0);
    /// assert_approx_eq!(f64, spectrum.chemical_shifts()[1], 2.0);
    /// assert_approx_eq!(f64, spectrum.chemical_shifts()[2], 4.0);
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_chemical_shifts(&mut self, chemical_shifts: Vec<f64>) -> Result<()> {
        Self::validate_lengths(&chemical_shifts, self.intensities_raw())?;
        Self::validate_spacing(&chemical_shifts)?;
        Self::validate_monotonicity(
            &chemical_shifts,
            self.signal_boundaries,
            self.water_boundaries,
        )?;
        Self::validate_boundaries(
            self.monotonicity,
            &chemical_shifts,
            self.signal_boundaries,
            self.water_boundaries,
        )?;
        self.chemical_shifts = chemical_shifts.into_boxed_slice();

        Ok(())
    }

    /// Sets the preprocessed intensities of the `Spectrum`.
    ///
    /// This can be used to set the intensities after applying preprocessing
    /// steps yourself. Generally not recommended.
    ///
    /// # Errors
    ///
    /// Performs the same checks as [`Spectrum::new`] on the intensities, using
    /// the chemical shifts of the `Spectrum`.
    ///
    /// # Example
    ///
    /// ```
    /// use float_cmp::assert_approx_eq;
    /// use metabodecon::spectrum::Spectrum;
    ///
    /// # fn main() -> metabodecon::Result<()> {
    /// let mut spectrum = Spectrum::new(
    ///     vec![1.0, 2.0, 3.0],
    ///     vec![1.0, 2.0, 3.0],
    ///     (1.0, 3.0),
    ///     (2.0, 2.5),
    /// )?;
    /// spectrum.set_intensities(vec![1.5, 2.0, 2.5])?;
    ///
    /// assert_approx_eq!(f64, spectrum.intensities()[0], 1.5);
    /// assert_approx_eq!(f64, spectrum.intensities()[1], 2.0);
    /// assert_approx_eq!(f64, spectrum.intensities()[2], 2.5);
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_intensities(&mut self, intensities: Vec<f64>) -> Result<()> {
        Self::validate_lengths(self.chemical_shifts(), &intensities)?;
        Self::validate_intensities(&intensities)?;
        self.intensities = intensities.into_boxed_slice();

        Ok(())
    }

    /// Sets the raw intensities of the `Spectrum`.
    ///
    /// # Errors
    ///
    /// Performs the same checks as [`Spectrum::new`] on the raw intensities,
    /// using the chemical shifts of the `Spectrum`.
    ///
    /// # Example
    ///
    /// ```
    /// use float_cmp::assert_approx_eq;
    /// use metabodecon::spectrum::Spectrum;
    ///
    /// # fn main() -> metabodecon::Result<()> {
    /// let mut spectrum = Spectrum::new(
    ///     vec![1.0, 2.0, 3.0],
    ///     vec![1.0, 2.0, 3.0], // Raw intensities
    ///     (1.0, 3.0),
    ///     (2.0, 2.5),
    /// )?;
    /// spectrum.set_intensities_raw(vec![10.0, 20.0, 30.0])?;
    ///
    /// assert_approx_eq!(f64, spectrum.intensities_raw()[0], 10.0);
    /// assert_approx_eq!(f64, spectrum.intensities_raw()[1], 20.0);
    /// assert_approx_eq!(f64, spectrum.intensities_raw()[2], 30.0);
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_intensities_raw(&mut self, intensities_raw: Vec<f64>) -> Result<()> {
        Self::validate_lengths(self.chemical_shifts(), &intensities_raw)?;
        Self::validate_intensities(&intensities_raw)?;
        self.intensities_raw = intensities_raw.into_boxed_slice();

        Ok(())
    }

    /// Sets the signal region boundaries of the `Spectrum`.
    ///
    /// # Errors
    ///
    /// Performs the same checks as [`Spectrum::new`] on the signal boundaries,
    /// using the chemical shifts and water boundaries of the `Spectrum`.
    ///
    /// # Example
    ///
    /// ```
    /// use float_cmp::assert_approx_eq;
    /// use metabodecon::spectrum::Spectrum;
    ///
    /// # fn main() -> metabodecon::Result<()> {
    /// let mut spectrum = Spectrum::new(
    ///     vec![1.0, 2.0, 3.0],
    ///     vec![1.0, 2.0, 3.0],
    ///     (1.0, 3.0), // Signal boundaries
    ///     (2.0, 2.5),
    /// )?;
    /// spectrum.set_signal_boundaries((1.25, 2.75))?;
    ///
    /// assert_approx_eq!(f64, spectrum.signal_boundaries().0, 1.25);
    /// assert_approx_eq!(f64, spectrum.signal_boundaries().1, 2.75);
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_signal_boundaries(&mut self, signal_boundaries: (f64, f64)) -> Result<()> {
        Self::validate_monotonicity(
            self.chemical_shifts(),
            signal_boundaries,
            self.water_boundaries,
        )?;
        Self::validate_boundaries(
            self.monotonicity,
            self.chemical_shifts(),
            signal_boundaries,
            self.water_boundaries,
        )?;
        self.signal_boundaries = signal_boundaries;

        Ok(())
    }

    /// Sets the water artifact boundaries of the `Spectrum`.
    ///
    /// # Errors
    ///
    /// Performs the same checks as [`Spectrum::new`] on the water boundaries,
    /// using the chemical shifts and signal boundaries of the `Spectrum`.
    ///
    /// # Example
    ///
    /// ```
    /// use float_cmp::assert_approx_eq;
    /// use metabodecon::spectrum::Spectrum;
    ///
    /// # fn main() -> metabodecon::Result<()> {
    /// let mut spectrum = Spectrum::new(
    ///     vec![1.0, 2.0, 3.0],
    ///     vec![1.0, 2.0, 3.0],
    ///     (1.0, 3.0),
    ///     (2.0, 2.5), // Water boundaries
    /// )?;
    /// spectrum.set_water_boundaries((2.05, 2.10))?;
    ///
    /// assert_approx_eq!(f64, spectrum.water_boundaries().0, 2.05);
    /// assert_approx_eq!(f64, spectrum.water_boundaries().1, 2.10);
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_water_boundaries(&mut self, water_boundaries: (f64, f64)) -> Result<()> {
        Self::validate_monotonicity(
            self.chemical_shifts(),
            self.signal_boundaries,
            water_boundaries,
        )?;
        Self::validate_boundaries(
            self.monotonicity,
            self.chemical_shifts(),
            self.signal_boundaries,
            water_boundaries,
        )?;
        self.water_boundaries = water_boundaries;

        Ok(())
    }

    /// Computes the step size between two consecutive chemical shifts in ppm.
    ///
    /// # Example
    ///
    /// ```
    /// use float_cmp::assert_approx_eq;
    /// use metabodecon::spectrum::Spectrum;
    ///
    /// # fn main() -> metabodecon::Result<()> {
    /// let spectrum = Spectrum::new(
    ///     vec![1.0, 2.0, 3.0], // Chemical shifts
    ///     vec![1.0, 2.0, 3.0],
    ///     (1.0, 3.0),
    ///     (2.0, 2.5),
    /// )?;
    ///
    /// assert_approx_eq!(f64, spectrum.step(), 1.0);
    /// # Ok(())
    /// # }
    /// ```
    pub fn step(&self) -> f64 {
        self.chemical_shifts[1] - self.chemical_shifts[0]
    }

    /// Computes the width of the `Spectrum` in ppm.
    ///
    /// # Example
    ///
    /// ```
    /// use float_cmp::assert_approx_eq;
    /// use metabodecon::spectrum::Spectrum;
    ///
    /// # fn main() -> metabodecon::Result<()> {
    /// let spectrum = Spectrum::new(
    ///     vec![1.0, 2.0, 3.0], // Chemical shifts
    ///     vec![1.0, 2.0, 3.0],
    ///     (1.0, 3.0),
    ///     (2.0, 2.5),
    /// )?;
    ///
    /// assert_approx_eq!(f64, spectrum.width(), 2.0);
    /// # Ok(())
    /// # }
    /// ```
    pub fn width(&self) -> f64 {
        self.chemical_shifts.last().unwrap() - self.chemical_shifts.first().unwrap()
    }

    /// Computes the center of the `Spectrum` in ppm.
    ///
    /// # Example
    ///
    /// ```
    /// use float_cmp::assert_approx_eq;
    /// use metabodecon::spectrum::Spectrum;
    ///
    /// # fn main() -> metabodecon::Result<()> {
    /// let spectrum = Spectrum::new(
    ///     vec![1.0, 2.0, 3.0], // Chemical shifts
    ///     vec![1.0, 2.0, 3.0],
    ///     (1.0, 3.0),
    ///     (2.0, 2.5),
    /// )?;
    ///
    /// assert_approx_eq!(f64, spectrum.center(), 2.0);
    /// # Ok(())
    /// # }
    /// ```
    pub fn center(&self) -> f64 {
        self.chemical_shifts.first().unwrap() + self.width() / 2.0
    }

    /// Computes the indices of the chemical shifts that are closest to the
    /// signal region boundaries.
    ///
    /// # Example
    ///
    /// ```
    /// use float_cmp::assert_approx_eq;
    /// use metabodecon::spectrum::Spectrum;
    ///
    /// # fn main() -> metabodecon::Result<()> {
    /// let spectrum = Spectrum::new(
    ///     vec![1.0, 2.0, 3.0, 4.0, 5.0], // Chemical shifts
    ///     vec![1.0, 2.0, 3.0, 4.0, 5.0],
    ///     (2.25, 3.75), // Signal boundaries
    ///     (2.45, 2.55),
    /// )?;
    ///
    /// assert_eq!(spectrum.signal_boundaries_indices(), (1, 3));
    /// # Ok(())
    /// # }
    /// ```
    pub fn signal_boundaries_indices(&self) -> (usize, usize) {
        (
            ((self.signal_boundaries.0 - self.chemical_shifts[0]) / self.step()).floor() as usize,
            ((self.signal_boundaries.1 - self.chemical_shifts[0]) / self.step()).ceil() as usize,
        )
    }

    /// Computes the indices of the chemical shifts that are closest to the
    /// water artifact boundaries.
    ///
    /// # Example
    ///
    /// ```
    /// use float_cmp::assert_approx_eq;
    /// use metabodecon::spectrum::Spectrum;
    ///
    /// # fn main() -> metabodecon::Result<()> {
    /// let spectrum = Spectrum::new(
    ///     vec![1.0, 2.0, 3.0, 4.0, 5.0], // Chemical shifts
    ///     vec![1.0, 2.0, 3.0, 4.0, 5.0],
    ///     (2.25, 3.75),
    ///     (2.45, 3.55), // Water boundaries
    /// )?;
    ///
    /// assert_eq!(spectrum.water_boundaries_indices(), (1, 3));
    /// # Ok(())
    /// # }
    /// ```
    pub fn water_boundaries_indices(&self) -> (usize, usize) {
        (
            ((self.water_boundaries.0 - self.chemical_shifts[0]) / self.step()).floor() as usize,
            ((self.water_boundaries.1 - self.chemical_shifts[0]) / self.step()).ceil() as usize,
        )
    }

    /// Applies preprocessing to the raw intensities of the `Spectrum` and
    /// stores the result in the intensities.
    ///
    /// The preprocessing steps are:
    /// 1. Removing the water signal from the intensities.
    /// 2. Removing negative values from the intensities.
    /// 3. Smoothing the intensities using the specified [`SmoothingAlgo`].
    ///
    /// # Errors
    ///
    /// Performs the same checks as [`Spectrum::new`] on the computed
    /// intensities, using the chemical shifts of the `Spectrum`.
    ///
    /// # Example
    ///
    /// ```
    /// use float_cmp::assert_approx_eq;
    /// use metabodecon::{deconvolution::SmoothingAlgo, spectrum::Spectrum};
    ///
    /// # fn main() -> metabodecon::Result<()> {
    /// let mut spectrum = Spectrum::new(
    ///     vec![1.0, 2.0, 3.0, 4.0, 5.0],
    ///     vec![1.25, 1.75, -1.5, -2.0, 1.75], // Raw intensities
    ///     (2.0, 4.0),
    ///     (2.95, 3.05), // Water boundaries
    /// )?;
    /// spectrum.apply_preprocessing(SmoothingAlgo::default())?;
    ///
    /// assert_eq!(spectrum.intensities().len(), 5);
    /// assert_approx_eq!(f64, spectrum.intensities()[0], 1.232638, epsilon = 1e-6);
    /// assert_approx_eq!(f64, spectrum.intensities()[1], 1.276041, epsilon = 1e-6);
    /// assert_approx_eq!(f64, spectrum.intensities()[2], 1.279166, epsilon = 1e-6);
    /// assert_approx_eq!(f64, spectrum.intensities()[3], 1.338541, epsilon = 1e-6);
    /// assert_approx_eq!(f64, spectrum.intensities()[4], 1.357638, epsilon = 1e-6);
    /// # Ok(())
    /// # }
    /// ```
    pub fn apply_preprocessing(&mut self, smoothing_algo: SmoothingAlgo) -> Result<()> {
        let water_boundaries_indices = self.water_boundaries_indices();
        let mut intensities = self.intensities_raw().to_vec();
        Self::remove_water_signal(&mut intensities, water_boundaries_indices);
        Self::remove_negative_values(&mut intensities);
        Self::smooth_intensities(&mut intensities, smoothing_algo);
        self.set_intensities(intensities)?;

        Ok(())
    }

    /// Internal helper function to remove the water signal from the provided
    /// intensities by fitting a line through the boundaries.
    fn remove_water_signal(intensities: &mut [f64], boundary_indices: (usize, usize)) {
        let slope = (intensities[boundary_indices.1] - intensities[boundary_indices.0])
            / f64::abs(boundary_indices.1 as f64 - boundary_indices.0 as f64);
        let start = intensities[boundary_indices.0];
        intensities[boundary_indices.0..boundary_indices.1]
            .iter_mut()
            .enumerate()
            .for_each(|(index, intensity)| {
                *intensity = slope * index as f64 + start;
            });
    }

    /// Internal helper function to remove negative values from the provided
    /// intensities.
    fn remove_negative_values(intensities: &mut [f64]) {
        intensities
            .iter_mut()
            .filter(|intensity| **intensity < 0.0)
            .for_each(|intensity| *intensity = -*intensity);
    }

    /// Internal helper function to smooth the intensities using the specified
    /// algorithm.
    fn smooth_intensities(intensities: &mut [f64], algorithm: SmoothingAlgo) {
        match algorithm {
            SmoothingAlgo::MovingAverage {
                iterations,
                window_size,
            } => {
                let mut smoother = MovingAverage::<f64>::new(iterations, window_size);
                smoother.smooth_values(intensities);
            }
        }
    }

    /// Internal helper function to validate the lengths of the input data and
    /// return an error if the checks fail.
    ///
    /// # Errors
    ///
    /// The following errors are possible if the respective check fails:
    /// - [`EmptyData`]: Either the chemical shifts or intensities are empty.
    /// - [`DataLengthMismatch`]: The lengths of the chemical shifts and
    ///   intensities do not match.
    ///
    /// [`EmptyData`]: Kind::EmptyData
    /// [`DataLengthMismatch`]: Kind::DataLengthMismatch
    fn validate_lengths(chemical_shifts: &[f64], intensities: &[f64]) -> Result<()> {
        if chemical_shifts.is_empty() || intensities.is_empty() {
            return Err(Error::new(Kind::EmptyData {
                chemical_shifts: chemical_shifts.len(),
                intensities: intensities.len(),
            })
            .into());
        }

        if chemical_shifts.len() != intensities.len() {
            return Err(Error::new(Kind::DataLengthMismatch {
                chemical_shifts: chemical_shifts.len(),
                intensities: intensities.len(),
            })
            .into());
        }

        Ok(())
    }

    /// Internal helper function to validate the spacing of the chemical shifts
    /// and return an error if the checks fail.
    ///
    /// # Errors
    ///
    /// The following errors are possible if the respective check fails:
    /// - [`NonUniformSpacing`]: Some chemical shift value is not finite, or the
    ///   difference between two consecutive values is different from the prior
    ///   step size or less than 100 times the floating-point precision.
    ///
    /// [`NonUniformSpacing`]: Kind::NonUniformSpacing
    fn validate_spacing(chemical_shifts: &[f64]) -> Result<()> {
        let step_size = chemical_shifts[1] - chemical_shifts[0];
        if step_size.abs() < 100.0 * f64::EPSILON {
            return Err(Error::new(Kind::NonUniformSpacing { positions: (0, 1) }).into());
        }

        if let Some(position) = chemical_shifts.windows(2).position(|w| {
            (w[1] - w[0] - step_size).abs() > 100.0 * f64::EPSILON || !(w[1] - w[0]).is_finite()
        }) {
            return Err(Error::new(Kind::NonUniformSpacing {
                positions: (position, position + 1),
            })
            .into());
        }

        Ok(())
    }

    /// Internal helper function to validate the intensities and return an error
    /// if the checks fail.
    ///
    /// # Errors
    ///
    /// The following errors are possible if the respective check fails:
    /// - [`InvalidIntensities`]: Some intensity value is not finite.
    ///
    /// [`InvalidIntensities`]: Kind::InvalidIntensities
    fn validate_intensities(intensities: &[f64]) -> Result<()> {
        if let Some(position) = intensities
            .iter()
            .position(|intensity| !intensity.is_finite())
        {
            return Err(Error::new(Kind::InvalidIntensities { position }).into());
        }

        Ok(())
    }

    /// Internal helper function to validate the monotonicity. Returns an error
    /// if the chemical shifts, signal boundaries, and water boundaries do not
    /// have the same monotonicity, or if the monotonicity cannot be determined
    /// due to non-uniform spacing or non-comparable values.
    ///
    /// # Errors
    ///
    /// The following errors are possible if the respective check fails:
    /// - [`MonotonicityMismatch`]: The chemical shifts, signal boundaries, and
    ///   water boundaries are not sorted in the same order.
    ///
    /// [`MonotonicityMismatch`]: Kind::MonotonicityMismatch
    fn validate_monotonicity(
        chemical_shifts: &[f64],
        signal_boundaries: (f64, f64),
        water_boundaries: (f64, f64),
    ) -> Result<Monotonicity> {
        let chemical_shifts_monotonicity =
            Monotonicity::from_f64s(chemical_shifts[0], chemical_shifts[1])
                .ok_or_else(|| Error::new(Kind::NonUniformSpacing { positions: (0, 1) }))?;
        let signal_boundaries_monotonicity =
            Monotonicity::from_f64s(signal_boundaries.0, signal_boundaries.1).ok_or_else(|| {
                Error::new(Kind::InvalidSignalBoundaries {
                    signal_boundaries,
                    chemical_shifts_range: (chemical_shifts[0], *chemical_shifts.last().unwrap()),
                })
            })?;
        let water_boundaries_monotonicity =
            Monotonicity::from_f64s(water_boundaries.0, water_boundaries.1).ok_or_else(|| {
                Error::new(Kind::InvalidWaterBoundaries {
                    water_boundaries,
                    signal_boundaries,
                    chemical_shifts_range: (chemical_shifts[0], *chemical_shifts.last().unwrap()),
                })
            })?;

        if chemical_shifts_monotonicity != signal_boundaries_monotonicity
            || chemical_shifts_monotonicity != water_boundaries_monotonicity
        {
            return Err(Error::new(Kind::MonotonicityMismatch {
                chemical_shifts: chemical_shifts_monotonicity,
                signal_boundaries: signal_boundaries_monotonicity,
                water_boundaries: water_boundaries_monotonicity,
            })
            .into());
        }

        Ok(chemical_shifts_monotonicity)
    }

    /// Internal helper function to validate the boundaries and return an error
    /// if the checks fail.
    ///
    /// # Errors
    ///
    /// The following errors are possible if the respective check fails:
    /// - [`InvalidSignalBoundaries`]: The signal region boundaries are not
    ///   within the range of the chemical shifts.
    /// - [`InvalidWaterBoundaries`]: The water artifact boundaries are not
    ///   within the signal region boundaries.
    ///
    /// [`InvalidSignalBoundaries`]: Kind::InvalidSignalBoundaries
    /// [`InvalidWaterBoundaries`]: Kind::InvalidWaterBoundaries
    fn validate_boundaries(
        monotonicity: Monotonicity,
        chemical_shifts: &[f64],
        signal_boundaries: (f64, f64),
        water_boundaries: (f64, f64),
    ) -> Result<()> {
        let chemical_shifts_range = (chemical_shifts[0], *chemical_shifts.last().unwrap());
        match monotonicity {
            Monotonicity::Increasing => {
                if signal_boundaries.0 < chemical_shifts_range.0
                    || signal_boundaries.1 > chemical_shifts_range.1
                {
                    return Err(Error::new(Kind::InvalidSignalBoundaries {
                        signal_boundaries,
                        chemical_shifts_range,
                    })
                    .into());
                }
                if water_boundaries.0 < signal_boundaries.0
                    || water_boundaries.1 > signal_boundaries.1
                {
                    return Err(Error::new(Kind::InvalidWaterBoundaries {
                        water_boundaries,
                        signal_boundaries,
                        chemical_shifts_range,
                    })
                    .into());
                }
            }
            Monotonicity::Decreasing => {
                if signal_boundaries.0 > chemical_shifts_range.0
                    || signal_boundaries.1 < chemical_shifts_range.1
                {
                    return Err(Error::new(Kind::InvalidSignalBoundaries {
                        signal_boundaries,
                        chemical_shifts_range,
                    })
                    .into());
                }
                if water_boundaries.0 > signal_boundaries.0
                    || water_boundaries.1 < signal_boundaries.1
                {
                    return Err(Error::new(Kind::InvalidWaterBoundaries {
                        water_boundaries,
                        signal_boundaries,
                        chemical_shifts_range,
                    })
                    .into());
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Error;
    use float_cmp::assert_approx_eq;

    #[test]
    fn new() {
        let spectrum_increasing = Spectrum::new(
            vec![1.0, 2.0, 3.0],
            vec![1.0, 2.0, 3.0],
            (1.0, 3.0),
            (2.0, 2.5),
        );
        let spectrum_decreasing = Spectrum::new(
            vec![3.0, 2.0, 1.0],
            vec![3.0, 2.0, 1.0],
            (3.0, 1.0),
            (2.5, 2.0),
        );
        assert!(spectrum_increasing.is_ok());
        assert!(spectrum_decreasing.is_ok());
    }

    #[test]
    fn empty_data() {
        let s = (1.0, 3.0);
        let w = (2.0, 2.5);
        let errors = [
            Spectrum::new(vec![], vec![1.0], s, w).unwrap_err(),
            Spectrum::new(vec![1.0], vec![], s, w).unwrap_err(),
            Spectrum::new(vec![], vec![], s, w).unwrap_err(),
        ];
        let expected_context = [(0, 1), (1, 0), (0, 0)];
        errors
            .into_iter()
            .zip(expected_context)
            .for_each(|(error, context)| {
                match error {
                    Error::Spectrum(inner) => match inner.kind() {
                        Kind::EmptyData {
                            chemical_shifts,
                            intensities,
                        } => {
                            assert_eq!(*chemical_shifts, context.0);
                            assert_eq!(*intensities, context.1);
                        }
                        _ => panic!("Unexpected kind: {:?}", inner),
                    },
                    _ => panic!("Unexpected error: {:?}", error),
                };
            });
    }

    #[test]
    fn data_length_mismatch() {
        let s = (1.0, 3.0);
        let w = (2.0, 2.5);
        let errors = [
            Spectrum::new(vec![1.0, 2.0, 3.0], vec![1.0, 2.0], s, w).unwrap_err(),
            Spectrum::new(vec![1.0, 2.0], vec![1.0, 2.0, 3.0], s, w).unwrap_err(),
        ];
        let expected_context = [(3, 2), (2, 3)];
        errors
            .into_iter()
            .zip(expected_context)
            .for_each(|(error, context)| {
                match error {
                    Error::Spectrum(inner) => match inner.kind() {
                        Kind::DataLengthMismatch {
                            chemical_shifts,
                            intensities,
                        } => {
                            assert_eq!(*chemical_shifts, context.0);
                            assert_eq!(*intensities, context.1);
                        }
                        _ => panic!("Unexpected kind: {:?}", inner),
                    },
                    _ => panic!("Unexpected error: {:?}", error),
                };
            })
    }

    #[test]
    fn invalid_signal_boundaries() {
        let d = vec![1.0, 2.0, 3.0];
        let r = (1.0, 3.0);
        let w = (2.0, 2.5);
        let errors = [
            Spectrum::new(d.clone(), d.clone(), (0.0, 3.0), w).unwrap_err(),
            Spectrum::new(d.clone(), d.clone(), (1.0, 4.0), w).unwrap_err(),
            Spectrum::new(d.clone(), d.clone(), (2.0, 2.0), w).unwrap_err(),
        ];
        let expected_contest = [((0.0, 3.0), r), ((1.0, 4.0), r), ((2.0, 2.0), r)];
        errors
            .into_iter()
            .zip(expected_contest)
            .for_each(|(error, context)| {
                match error {
                    Error::Spectrum(inner) => match inner.kind() {
                        Kind::InvalidSignalBoundaries {
                            signal_boundaries,
                            chemical_shifts_range,
                        } => {
                            assert_approx_eq!(f64, signal_boundaries.0, context.0.0);
                            assert_approx_eq!(f64, signal_boundaries.1, context.0.1);
                            assert_approx_eq!(f64, chemical_shifts_range.0, context.1.0);
                            assert_approx_eq!(f64, chemical_shifts_range.1, context.1.1);
                        }
                        _ => panic!("Unexpected kind: {:?}", inner),
                    },
                    _ => panic!("Unexpected error: {:?}", error),
                };
            });
    }

    #[test]
    fn invalid_water_boundaries() {
        let d = vec![1.0, 2.0, 3.0];
        let r = (1.0, 3.0);
        let s = (1.0, 3.0);
        let errors = [
            Spectrum::new(d.clone(), d.clone(), s, (0.0, 2.5)).unwrap_err(),
            Spectrum::new(d.clone(), d.clone(), s, (2.0, 4.0)).unwrap_err(),
            Spectrum::new(d.clone(), d.clone(), s, (2.0, 2.0)).unwrap_err(),
        ];
        let expected_contest = [((0.0, 2.5), s, r), ((2.0, 4.0), s, r), ((2.0, 2.0), s, r)];
        errors
            .into_iter()
            .zip(expected_contest)
            .for_each(|(error, context)| {
                match error {
                    Error::Spectrum(inner) => match inner.kind() {
                        Kind::InvalidWaterBoundaries {
                            water_boundaries,
                            signal_boundaries,
                            chemical_shifts_range,
                        } => {
                            assert_approx_eq!(f64, water_boundaries.0, context.0.0);
                            assert_approx_eq!(f64, water_boundaries.1, context.0.1);
                            assert_approx_eq!(f64, signal_boundaries.0, context.1.0);
                            assert_approx_eq!(f64, signal_boundaries.1, context.1.1);
                            assert_approx_eq!(f64, chemical_shifts_range.0, context.2.0);
                            assert_approx_eq!(f64, chemical_shifts_range.1, context.2.1);
                        }
                        _ => panic!("Unexpected kind: {:?}", inner),
                    },
                    _ => panic!("Unexpected error: {:?}", error),
                };
            });
    }

    #[test]
    fn monotonicity_mismatch() {
        let d = vec![1.0, 2.0, 3.0];
        // i = increasing, d = decreasing
        let di = vec![1.0, 2.0, 3.0];
        let dd = vec![3.0, 2.0, 1.0];
        let si = (1.0, 3.0);
        let sd = (3.0, 1.0);
        let wi = (2.0, 2.5);
        let wd = (2.5, 2.0);
        let errors = [
            Spectrum::new(di.clone(), d.clone(), si, wd).unwrap_err(),
            Spectrum::new(di.clone(), d.clone(), sd, wi).unwrap_err(),
            Spectrum::new(di.clone(), d.clone(), sd, wd).unwrap_err(),
            Spectrum::new(dd.clone(), d.clone(), si, wd).unwrap_err(),
            Spectrum::new(dd.clone(), d.clone(), sd, wi).unwrap_err(),
            Spectrum::new(dd.clone(), d.clone(), si, wi).unwrap_err(),
        ];
        let expected_context = [
            (
                Monotonicity::Increasing,
                Monotonicity::Increasing,
                Monotonicity::Decreasing,
            ),
            (
                Monotonicity::Increasing,
                Monotonicity::Decreasing,
                Monotonicity::Increasing,
            ),
            (
                Monotonicity::Increasing,
                Monotonicity::Decreasing,
                Monotonicity::Decreasing,
            ),
            (
                Monotonicity::Decreasing,
                Monotonicity::Increasing,
                Monotonicity::Decreasing,
            ),
            (
                Monotonicity::Decreasing,
                Monotonicity::Decreasing,
                Monotonicity::Increasing,
            ),
            (
                Monotonicity::Decreasing,
                Monotonicity::Increasing,
                Monotonicity::Increasing,
            ),
        ];
        errors
            .into_iter()
            .zip(expected_context)
            .for_each(|(error, context)| {
                match error {
                    Error::Spectrum(inner) => match inner.kind() {
                        Kind::MonotonicityMismatch {
                            chemical_shifts,
                            signal_boundaries,
                            water_boundaries,
                        } => {
                            assert_eq!(*chemical_shifts, context.0);
                            assert_eq!(*signal_boundaries, context.1);
                            assert_eq!(*water_boundaries, context.2);
                        }
                        _ => panic!("Unexpected kind: {:?}", inner),
                    },
                    _ => panic!("Unexpected error: {:?}", error),
                };
            });
    }

    #[test]
    fn accessors() {
        let spectrum = Spectrum::new(
            vec![1.0, 2.0, 3.0],
            vec![1.0, 2.0, 3.0],
            (1.0, 3.0),
            (2.0, 2.5),
        )
        .unwrap();
        spectrum
            .chemical_shifts()
            .iter()
            .zip([1.0, 2.0, 3.0])
            .for_each(|(&xc, xe)| assert_approx_eq!(f64, xc, xe));
        spectrum
            .intensities()
            .iter()
            .zip([1.0, 2.0, 3.0])
            .for_each(|(&ic, ie)| assert_approx_eq!(f64, ic, ie));
        assert_eq!(spectrum.intensities().len(), 0);
        assert_approx_eq!(f64, spectrum.signal_boundaries().0, 1.0);
        assert_approx_eq!(f64, spectrum.signal_boundaries().1, 3.0);
        assert_approx_eq!(f64, spectrum.water_boundaries().0, 2.0);
        assert_approx_eq!(f64, spectrum.water_boundaries().1, 2.5);
    }

    #[test]
    fn mutators() {
        let mut spectrum = Spectrum::new(
            vec![1.0, 2.0, 3.0],
            vec![1.0, 2.0, 3.0],
            (1.0, 3.0),
            (2.0, 2.5),
        )
        .unwrap();
        spectrum
            .set_chemical_shifts(vec![0.0, 2.0, 4.0])
            .unwrap();
        spectrum
            .set_intensities_raw(vec![0.0, 2.0, 4.0])
            .unwrap();
        spectrum
            .set_intensities(vec![1.0, 2.0, 3.0])
            .unwrap();
        spectrum
            .set_signal_boundaries((0.5, 3.5))
            .unwrap();
        spectrum.set_water_boundaries((1.5, 2.5)).unwrap();
        spectrum
            .set_intensities(
                spectrum
                    .intensities()
                    .iter()
                    .map(|intensity| -intensity)
                    .collect(),
            )
            .unwrap();
        spectrum
            .chemical_shifts()
            .iter()
            .zip([0.0, 2.0, 4.0])
            .for_each(|(&xc, xe)| assert_approx_eq!(f64, xc, xe));
        spectrum
            .intensities_raw()
            .iter()
            .zip([0.0, 2.0, 4.0])
            .for_each(|(&ic, ie)| assert_approx_eq!(f64, ic, ie));
        spectrum
            .intensities()
            .iter()
            .zip([-1.0, -2.0, -3.0])
            .for_each(|(&ic, ie)| assert_approx_eq!(f64, ic, ie));
        assert_approx_eq!(f64, spectrum.signal_boundaries().0, 0.5);
        assert_approx_eq!(f64, spectrum.signal_boundaries().1, 3.5);
        assert_approx_eq!(f64, spectrum.water_boundaries().0, 1.5);
        assert_approx_eq!(f64, spectrum.water_boundaries().1, 2.5);
    }

    #[test]
    fn properties() {
        let mut spectrum = Spectrum::new(
            vec![1.0, 2.0, 3.0, 4.0, 5.0],
            vec![1.0, 2.0, 3.0, 4.0, 5.0],
            (1.5, 4.5),
            (2.5, 3.5),
        )
        .unwrap();
        spectrum
            .set_intensities(vec![1.0, 2.0, 3.0, 4.0, 5.0])
            .unwrap();
        assert_approx_eq!(f64, spectrum.step(), 1.0);
        assert_approx_eq!(f64, spectrum.width(), 4.0);
        assert_approx_eq!(f64, spectrum.center(), 3.0);
        assert_eq!(spectrum.signal_boundaries_indices(), (0, 4));
        assert_eq!(spectrum.water_boundaries_indices(), (1, 3));
    }

    #[test]
    fn remove_water_signal() {
        let mut intensities = vec![1.0, 15.0, 16.0, 15.0, 5.0];
        let water_boundaries_indices = (0, 4);
        Spectrum::remove_water_signal(&mut intensities, water_boundaries_indices);
        intensities
            .iter()
            .zip([1.0, 2.0, 3.0, 4.0, 5.0])
            .for_each(|(&yc, ye)| {
                assert_approx_eq!(f64, yc, ye);
            });
    }

    #[test]
    fn remove_negative_values() {
        let mut intensities = vec![1.0, -2.0, 3.0, -4.0, 5.0];
        Spectrum::remove_negative_values(&mut intensities);
        intensities
            .iter()
            .zip([1.0, 2.0, 3.0, 4.0, 5.0])
            .for_each(|(&yc, ye)| {
                assert_approx_eq!(f64, yc, ye);
            });
    }

    #[test]
    fn smooth_intensities() {
        let mut intensities = vec![1.25, 1.75, 1.5, 2.0, 1.75];
        let algorithm = SmoothingAlgo::MovingAverage {
            iterations: 1,
            window_size: 3,
        };
        Spectrum::smooth_intensities(&mut intensities, algorithm);
        intensities
            .iter()
            .zip([1.5, 1.5, 1.75, 1.75, 1.875])
            .for_each(|(&yc, ye)| {
                assert_approx_eq!(f64, yc, ye);
            });
    }
}
