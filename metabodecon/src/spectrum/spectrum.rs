use crate::Result;
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
    /// Helper function to determine the `Monotonicity` from 2 floating point
    /// numbers.
    ///
    /// Checks for the ordering of two floating point numbers and returns the
    /// corresponding `Some(Monotonicity)` variant. If the two numbers differ by
    /// less than 100 times the floating point precision, or are not finite
    /// numbers, or cannot be compared, `None` is returned.
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

    pub(crate) fn reverse(&mut self) {
        match self {
            Self::Increasing => *self = Self::Decreasing,
            Self::Decreasing => *self = Self::Increasing,
        }
    }
}

/// Data structure that represents a 1D NMR spectrum.
///
/// `Spectrum` is a fixed size container that holds the chemical shifts, signal
/// intensities and metadata of a 1D NMR spectrum.
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
///             // Right signal centered at 7 ppm.
///             + 1.0 * 0.25 / (0.25_f64.powi(2) + (x - 7.0).powi(2))
///     })
///     .collect::<Vec<f64>>();
///
/// // Define the signal region.
/// let signal_boundaries = (1.0, 9.0);
///
/// // Create a Spectrum object.
/// let spectrum =
///     Spectrum::new(chemical_shifts, intensities, signal_boundaries)?;
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Debug)]
pub struct Spectrum {
    /// The chemical shifts in ppm.
    chemical_shifts: Box<[f64]>,
    /// The intensities in arbitrary units.
    intensities: Box<[f64]>,
    /// The boundaries of the signal region.
    signal_boundaries: (f64, f64),
    /// The monotonicity of the data.
    monotonicity: Monotonicity,
}

impl AsRef<Spectrum> for Spectrum {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl Spectrum {
    /// Constructs a `Spectrum` from the given data.
    ///
    /// Note that this is generally not the recommended way to create `Spectrum`
    /// objects. See [`BrukerReader`] and [`JdxReader`] for parsing 1D NMR data
    /// from Bruker TopSpin and JCAMP-DX file formats.
    ///
    /// [`BrukerReader`]: crate::spectrum::Bruker
    /// [`JdxReader`]: crate::spectrum::JcampDx
    ///
    /// # Errors
    ///
    /// The input data is checked for validity to ensure that the `Spectrum` is
    /// well-formed and in a consistent state. The following conditions are
    /// checked:
    /// - The chemical shifts and intensities are not empty.
    /// - The lengths of the chemical shifts and intensities match.
    /// - All chemical shift values are finite and uniformly spaced.
    /// - All intensity values are finite.
    /// - The signal region boundaries are within the range of the chemical
    ///   shifts.
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
    ///             // Right signal centered at 7 ppm.
    ///             + 1.0 * 0.25 / (0.25_f64.powi(2) + (x - 7.0).powi(2))
    ///     })
    ///     .collect::<Vec<f64>>();
    ///
    /// // Define the signal region.
    /// let signal_boundaries = (1.0, 9.0);
    ///
    /// // Create a `Spectrum`.
    /// let spectrum =
    ///     Spectrum::new(chemical_shifts, intensities, signal_boundaries)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(
        chemical_shifts: Vec<f64>,
        intensities: Vec<f64>,
        signal_boundaries: (f64, f64),
    ) -> Result<Self> {
        Self::validate_lengths(&chemical_shifts, &intensities)?;
        Self::validate_spacing(&chemical_shifts)?;
        Self::validate_intensities(&intensities)?;
        let monotonicity = Monotonicity::from_f64s(chemical_shifts[0], chemical_shifts[1])
            .ok_or_else(|| Error::new(Kind::NonUniformSpacing { positions: (0, 1) }))?;
        let signal_boundaries =
            Self::validate_boundaries(monotonicity, &chemical_shifts, signal_boundaries)?;

        Ok(Self {
            chemical_shifts: chemical_shifts.into_boxed_slice(),
            intensities: intensities.into_boxed_slice(),
            signal_boundaries,
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
    ///     vec![1.0, 2.0, 3.0], // Intensities
    ///     (1.0, 3.0),
    /// )?;
    ///
    /// assert_eq!(spectrum.intensities().len(), 3);
    /// assert_approx_eq!(f64, spectrum.intensities()[0], 1.0);
    /// assert_approx_eq!(f64, spectrum.intensities()[1], 2.0);
    /// assert_approx_eq!(f64, spectrum.intensities()[2], 3.0);
    /// # Ok(())
    /// # }
    /// ```
    pub fn intensities(&self) -> &[f64] {
        &self.intensities
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

    /// Returns the monotonicity of the `Spectrum`.
    ///
    /// # Example
    ///
    /// ```
    /// use metabodecon::spectrum::{Monotonicity, Spectrum};
    ///
    /// # fn main() -> metabodecon::Result<()> {
    /// let spectrum_increasing =
    ///     Spectrum::new(vec![1.0, 2.0, 3.0], vec![1.0, 2.0, 3.0], (1.0, 3.0))?;
    /// let spectrum_decreasing =
    ///     Spectrum::new(vec![3.0, 2.0, 1.0], vec![3.0, 2.0, 1.0], (3.0, 1.0))?;
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
    /// Updates the order of the intensities and signal boundaries to match the
    /// new chemical shifts if that is necessary. Note that the signal
    /// boundaries must still be within the range of the new chemical shifts.
    ///
    /// # Errors
    ///
    /// The input data is checked for validity to ensure that the `Spectrum` is
    /// well-formed and in a consistent state. The following conditions are
    /// checked:
    /// - The chemical shifts are not empty.
    /// - The lengths of the chemical shifts and intensities match.
    /// - All chemical shift values are finite and uniformly spaced.
    /// - The signal region boundaries are within the range of the chemical
    ///   shifts.
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
        Self::validate_lengths(&chemical_shifts, self.intensities())?;
        Self::validate_spacing(&chemical_shifts)?;
        let monotonicity = Monotonicity::from_f64s(chemical_shifts[0], chemical_shifts[1])
            .ok_or_else(|| Error::new(Kind::NonUniformSpacing { positions: (0, 1) }))?;
        let signal_boundaries =
            Self::validate_boundaries(monotonicity, &chemical_shifts, self.signal_boundaries)?;
        if monotonicity != self.monotonicity {
            self.intensities.reverse()
        }
        self.chemical_shifts = chemical_shifts.into_boxed_slice();
        self.signal_boundaries = signal_boundaries;
        self.monotonicity = monotonicity;

        Ok(())
    }

    /// Sets the intensities of the `Spectrum`.
    ///
    /// # Errors
    ///
    /// The input data is checked for validity to ensure that the `Spectrum` is
    /// well-formed and in a consistent state. The following conditions are
    /// checked:
    /// - The intensities are not empty.
    /// - The lengths of the chemical shifts and intensities match.
    /// - All intensity values are finite.
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
    ///     vec![1.0, 2.0, 3.0], // Intensities
    ///     (1.0, 3.0),
    /// )?;
    /// spectrum.set_intensities(vec![10.0, 20.0, 30.0])?;
    ///
    /// assert_approx_eq!(f64, spectrum.intensities()[0], 10.0);
    /// assert_approx_eq!(f64, spectrum.intensities()[1], 20.0);
    /// assert_approx_eq!(f64, spectrum.intensities()[2], 30.0);
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_intensities(&mut self, intensities_raw: Vec<f64>) -> Result<()> {
        Self::validate_lengths(self.chemical_shifts(), &intensities_raw)?;
        Self::validate_intensities(&intensities_raw)?;
        self.intensities = intensities_raw.into_boxed_slice();

        Ok(())
    }

    /// Sets the signal region boundaries of the `Spectrum`.
    ///
    /// # Errors
    ///
    /// The input data is checked for validity to ensure that the `Spectrum` is
    /// well-formed and in a consistent state. Checks if the signal boundaries
    /// are within the range of the chemical shifts. Reorders the boundaries to
    /// match the chemical shifts if necessary.
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
    /// )?;
    /// spectrum.set_signal_boundaries((1.25, 2.75))?;
    ///
    /// assert_approx_eq!(f64, spectrum.signal_boundaries().0, 1.25);
    /// assert_approx_eq!(f64, spectrum.signal_boundaries().1, 2.75);
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_signal_boundaries(&mut self, signal_boundaries: (f64, f64)) -> Result<()> {
        self.signal_boundaries = Self::validate_boundaries(
            self.monotonicity,
            self.chemical_shifts(),
            signal_boundaries,
        )?;

        Ok(())
    }

    /// Reverses the order of the chemical shifts, intensities and signal
    /// boundaries.
    ///
    /// # Example
    ///
    /// ```
    /// use float_cmp::assert_approx_eq;
    /// use metabodecon::spectrum::{Monotonicity, Spectrum};
    ///
    /// # fn main() -> metabodecon::Result<()> {
    /// let mut spectrum =
    ///     Spectrum::new(vec![1.0, 2.0, 3.0], vec![1.0, 2.0, 3.0], (1.0, 3.0))?;
    /// spectrum.reverse();
    ///
    /// assert_approx_eq!(f64, spectrum.chemical_shifts()[0], 3.0);
    /// assert_approx_eq!(f64, spectrum.chemical_shifts()[1], 2.0);
    /// assert_approx_eq!(f64, spectrum.chemical_shifts()[2], 1.0);
    /// assert_approx_eq!(f64, spectrum.intensities()[0], 3.0);
    /// assert_approx_eq!(f64, spectrum.intensities()[1], 2.0);
    /// assert_approx_eq!(f64, spectrum.intensities()[2], 1.0);
    /// assert_approx_eq!(f64, spectrum.signal_boundaries().0, 3.0);
    /// assert_approx_eq!(f64, spectrum.signal_boundaries().1, 1.0);
    /// assert_eq!(spectrum.monotonicity(), Monotonicity::Decreasing);
    /// # Ok(())
    /// # }
    /// ```
    pub fn reverse(&mut self) {
        self.chemical_shifts.reverse();
        self.intensities.reverse();
        self.signal_boundaries = (self.signal_boundaries.1, self.signal_boundaries.0);
        self.monotonicity.reverse();
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
    /// )?;
    ///
    /// assert_approx_eq!(f64, spectrum.step(), 1.0);
    /// # Ok(())
    /// # }
    /// ```
    pub fn step(&self) -> f64 {
        self.chemical_shifts[1] - self.chemical_shifts[0]
    }

    /// Computes the range of the `Spectrum` in ppm.
    ///
    /// The range is sorted in the same order as the chemical shifts.
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
    /// )?;
    ///
    /// assert_approx_eq!(f64, spectrum.range().0, 1.0);
    /// assert_approx_eq!(f64, spectrum.range().1, 3.0);
    /// # Ok(())
    /// # }
    /// ```
    pub fn range(&self) -> (f64, f64) {
        (
            *self.chemical_shifts.first().unwrap(),
            *self.chemical_shifts.last().unwrap(),
        )
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
    /// )?;
    ///
    /// assert_approx_eq!(f64, spectrum.width(), 2.0);
    /// # Ok(())
    /// # }
    /// ```
    pub fn width(&self) -> f64 {
        (self.chemical_shifts.last().unwrap() - self.chemical_shifts.first().unwrap()).abs()
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
    /// )?;
    ///
    /// assert_approx_eq!(f64, spectrum.center(), 2.0);
    /// # Ok(())
    /// # }
    /// ```
    pub fn center(&self) -> f64 {
        self.chemical_shifts.first().unwrap() + 0.5 * self.width()
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

    /// Internal helper function to validate the lengths of the input data and
    /// return an error if the checks fail.
    ///
    /// # Errors
    ///
    /// The following errors are possible:
    /// - [`EmptyData`](Kind::EmptyData)
    /// - [`DataLengthMismatch`](Kind::DataLengthMismatch)
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
    /// The following errors are possible:
    /// - [`NonUniformSpacing`](Kind::NonUniformSpacing)
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
    /// The following errors are possible:
    /// - [`InvalidIntensities`](Kind::InvalidIntensities)
    fn validate_intensities(intensities: &[f64]) -> Result<()> {
        if let Some(position) = intensities
            .iter()
            .position(|intensity| !intensity.is_finite())
        {
            return Err(Error::new(Kind::InvalidIntensities { position }).into());
        }

        Ok(())
    }

    /// Internal helper function to validate the boundaries and return an error
    /// if the checks fail and return the validated boundaries. Reorders the
    /// boundaries to match `chemical_shifts` if necessary.
    ///
    /// # Errors
    ///
    /// The following errors are possible:
    /// - [`InvalidSignalBoundaries`](Kind::InvalidSignalBoundaries)
    fn validate_boundaries(
        monotonicity: Monotonicity,
        chemical_shifts: &[f64],
        signal_boundaries: (f64, f64),
    ) -> Result<(f64, f64)> {
        let chemical_shifts_range = (chemical_shifts[0], *chemical_shifts.last().unwrap());
        if f64::abs(signal_boundaries.0 - signal_boundaries.1) < 100.0 * f64::EPSILON
            || !(signal_boundaries.0 - signal_boundaries.1).is_finite()
        {
            return Err(Error::new(Kind::InvalidSignalBoundaries {
                signal_boundaries,
                chemical_shifts_range,
            })
            .into());
        }
        let signal_boundaries = match monotonicity {
            Monotonicity::Increasing => (
                f64::min(signal_boundaries.0, signal_boundaries.1),
                f64::max(signal_boundaries.0, signal_boundaries.1),
            ),
            Monotonicity::Decreasing => (
                f64::max(signal_boundaries.0, signal_boundaries.1),
                f64::min(signal_boundaries.0, signal_boundaries.1),
            ),
        };
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
            }
        }

        Ok(signal_boundaries)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Error;
    use float_cmp::assert_approx_eq;

    #[test]
    fn new() {
        let spectrum_increasing =
            Spectrum::new(vec![1.0, 2.0, 3.0], vec![1.0, 2.0, 3.0], (1.0, 3.0));
        let spectrum_decreasing =
            Spectrum::new(vec![3.0, 2.0, 1.0], vec![3.0, 2.0, 1.0], (3.0, 1.0));
        assert!(spectrum_increasing.is_ok());
        assert!(spectrum_decreasing.is_ok());
    }

    #[test]
    fn empty_data() {
        let s = (1.0, 3.0);
        let errors = [
            Spectrum::new(vec![], vec![1.0], s).unwrap_err(),
            Spectrum::new(vec![1.0], vec![], s).unwrap_err(),
            Spectrum::new(vec![], vec![], s).unwrap_err(),
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
        let errors = [
            Spectrum::new(vec![1.0, 2.0, 3.0], vec![1.0, 2.0], s).unwrap_err(),
            Spectrum::new(vec![1.0, 2.0], vec![1.0, 2.0, 3.0], s).unwrap_err(),
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
        let errors = [
            Spectrum::new(d.clone(), d.clone(), (0.0, 3.0)).unwrap_err(),
            Spectrum::new(d.clone(), d.clone(), (1.0, 4.0)).unwrap_err(),
            Spectrum::new(d.clone(), d.clone(), (2.0, 2.0)).unwrap_err(),
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
    fn accessors() {
        let spectrum = Spectrum::new(vec![1.0, 2.0, 3.0], vec![1.0, 2.0, 3.0], (1.0, 3.0)).unwrap();
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
        assert_eq!(spectrum.chemical_shifts().len(), 3);
        assert_eq!(spectrum.intensities().len(), 3);
        assert_approx_eq!(f64, spectrum.signal_boundaries().0, 1.0);
        assert_approx_eq!(f64, spectrum.signal_boundaries().1, 3.0);
        assert_eq!(spectrum.monotonicity(), Monotonicity::Increasing);
    }

    #[test]
    fn mutators() {
        let mut spectrum =
            Spectrum::new(vec![1.0, 2.0, 3.0], vec![1.0, 2.0, 3.0], (1.0, 3.0)).unwrap();
        spectrum
            .set_chemical_shifts(vec![0.0, 2.0, 4.0])
            .unwrap();
        spectrum
            .set_intensities(vec![0.0, 2.0, 4.0])
            .unwrap();
        spectrum
            .set_signal_boundaries((3.5, 0.5))
            .unwrap();
        spectrum
            .chemical_shifts()
            .iter()
            .zip([0.0, 2.0, 4.0])
            .for_each(|(&xc, xe)| assert_approx_eq!(f64, xc, xe));
        spectrum
            .intensities()
            .iter()
            .zip([0.0, 2.0, 4.0])
            .for_each(|(&ic, ie)| assert_approx_eq!(f64, ic, ie));
        assert_approx_eq!(f64, spectrum.signal_boundaries().0, 0.5);
        assert_approx_eq!(f64, spectrum.signal_boundaries().1, 3.5);
        spectrum.reverse();
        spectrum
            .chemical_shifts()
            .iter()
            .zip([4.0, 2.0, 0.0])
            .for_each(|(&xc, xe)| assert_approx_eq!(f64, xc, xe));
        spectrum
            .intensities()
            .iter()
            .zip([4.0, 2.0, 0.0])
            .for_each(|(&ic, ie)| assert_approx_eq!(f64, ic, ie));
        assert_approx_eq!(f64, spectrum.signal_boundaries().0, 3.5);
        assert_approx_eq!(f64, spectrum.signal_boundaries().1, 0.5);
        assert_eq!(spectrum.monotonicity(), Monotonicity::Decreasing);
    }

    #[test]
    fn properties() {
        let spectrum = Spectrum::new(
            vec![1.0, 2.0, 3.0, 4.0, 5.0],
            vec![1.0, 2.0, 3.0, 4.0, 5.0],
            (1.5, 4.5),
        )
        .unwrap();
        assert_approx_eq!(f64, spectrum.step(), 1.0);
        assert_approx_eq!(f64, spectrum.width(), 4.0);
        assert_approx_eq!(f64, spectrum.range().0, 1.0);
        assert_approx_eq!(f64, spectrum.range().1, 5.0);
        assert_approx_eq!(f64, spectrum.center(), 3.0);
        assert_eq!(spectrum.signal_boundaries_indices(), (0, 4));
    }
}
