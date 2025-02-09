use crate::Result;
use crate::spectrum::error::{Error, Kind};
use crate::spectrum::meta::Monotonicity;
use std::sync::Arc;

#[cfg(feature = "serde")]
use crate::spectrum::SerializedSpectrum;
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

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
    chemical_shifts: Arc<[f64]>,
    /// The intensities in arbitrary units.
    intensities: Arc<[f64]>,
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

#[cfg(feature = "serde")]
impl Serialize for Spectrum {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        SerializedSpectrum::from(self).serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Spectrum {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        SerializedSpectrum::deserialize(deserializer)?
            .try_into()
            .map_err(serde::de::Error::custom)
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
        let monotonicity = Monotonicity::from_f64s(chemical_shifts[0], chemical_shifts[1]).unwrap();
        let signal_boundaries =
            Self::validate_boundaries(monotonicity, &chemical_shifts, signal_boundaries)?;

        Ok(Self {
            chemical_shifts: chemical_shifts.into(),
            intensities: intensities.into(),
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
    /// use metabodecon::spectrum::Spectrum;
    /// use metabodecon::spectrum::meta::Monotonicity;
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

    /// Returns the number of chemical shift, intensity pairs in the `Spectrum`.
    ///
    /// # Example
    ///
    /// ```
    /// use metabodecon::spectrum::Spectrum;
    ///
    /// # fn main() -> metabodecon::Result<()> {
    /// let spectrum = Spectrum::new(
    ///     vec![1.0, 2.0, 3.0], // Chemical shifts
    ///     vec![1.0, 2.0, 3.0], // Intensities
    ///     (1.0, 3.0),
    /// )?;
    ///
    /// assert_eq!(spectrum.len(), 3);
    /// # Ok(())
    /// # }
    /// ```
    #[allow(clippy::len_without_is_empty)] // Can never be empty.
    pub fn len(&self) -> usize {
        self.chemical_shifts.len()
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
            return Err(Error::new(Kind::NonUniformSpacing {
                step_size,
                positions: (0, 1),
            })
            .into());
        }

        if let Some(position) = chemical_shifts.windows(2).position(|w| {
            (w[1] - w[0] - step_size).abs() > 100.0 * f64::EPSILON || !(w[1] - w[0]).is_finite()
        }) {
            Err(Error::new(Kind::NonUniformSpacing {
                step_size,
                positions: (position, position + 1),
            })
            .into())
        } else {
            Ok(())
        }
    }

    /// Internal helper function to validate the intensities and return an error
    /// if the checks fail.
    ///
    /// # Errors
    ///
    /// The following errors are possible:
    /// - [`InvalidIntensities`](Kind::InvalidIntensities)
    fn validate_intensities(intensities: &[f64]) -> Result<()> {
        let positions = intensities
            .iter()
            .enumerate()
            .filter_map(|(i, intensity)| {
                if !intensity.is_finite() {
                    Some(i)
                } else {
                    None
                }
            })
            .collect::<Vec<usize>>();

        match positions.is_empty() {
            true => Ok(()),
            false => Err(Error::new(Kind::InvalidIntensities { positions }).into()),
        }
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
        let width = signal_boundaries.0 - signal_boundaries.1;
        if f64::abs(width) < 100.0 * f64::EPSILON || !width.is_finite() {
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
    use crate::{Error, assert_send, assert_sync};
    use float_cmp::assert_approx_eq;

    #[test]
    fn thread_safety() {
        assert_send!(Spectrum);
        assert_sync!(Spectrum);
    }

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
                        _ => panic!("unexpected kind: {:?}", inner),
                    },
                    _ => panic!("unexpected error: {:?}", error),
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
                        _ => panic!("unexpected kind: {:?}", inner),
                    },
                    _ => panic!("unexpected error: {:?}", error),
                };
            });
    }

    #[test]
    fn non_uniform_spacing() {
        let d = (1..=10).map(|i| i as f64).collect::<Vec<f64>>();
        let s = (2.0, 9.0);
        let nan_step = (1..=10)
            .map(|i| if i > 1 { i as f64 } else { f64::NAN })
            .collect();
        let inf_step = (1..=10)
            .map(|i| if i > 1 { i as f64 } else { f64::INFINITY })
            .collect();
        let neg_inf_step = (1..=10)
            .map(|i| if i > 1 { i as f64 } else { f64::NEG_INFINITY })
            .collect();
        let zero_step = (1..=10)
            .map(|i| if i > 1 { i as f64 } else { 2.0 })
            .collect();
        let one_nan = (1..=10)
            .map(|i| if i != 5 { i as f64 } else { f64::NAN })
            .collect();
        let one_inf = (1..=10)
            .map(|i| if i != 5 { i as f64 } else { f64::INFINITY })
            .collect();
        let one_neg_inf = (1..=10)
            .map(|i| if i != 5 { i as f64 } else { f64::NEG_INFINITY })
            .collect();
        let one_zero = (1..=10)
            .map(|i| if i < 5 { i as f64 } else { (i - 1) as f64 })
            .collect();
        let errors = [
            Spectrum::new(nan_step, d.clone(), s).unwrap_err(),
            Spectrum::new(inf_step, d.clone(), s).unwrap_err(),
            Spectrum::new(neg_inf_step, d.clone(), s).unwrap_err(),
            Spectrum::new(zero_step, d.clone(), s).unwrap_err(),
            Spectrum::new(one_nan, d.clone(), s).unwrap_err(),
            Spectrum::new(one_inf, d.clone(), s).unwrap_err(),
            Spectrum::new(one_neg_inf, d.clone(), s).unwrap_err(),
            Spectrum::new(one_zero, d.clone(), s).unwrap_err(),
        ];
        let expected_context = [
            (f64::NAN, (0, 1)),
            (f64::NEG_INFINITY, (0, 1)),
            (f64::INFINITY, (0, 1)),
            (0.0, (0, 1)),
            (1.0, (3, 4)),
            (1.0, (3, 4)),
            (1.0, (3, 4)),
            (1.0, (3, 4)),
        ];
        errors
            .into_iter()
            .zip(expected_context)
            .for_each(|(error, context)| {
                match error {
                    Error::Spectrum(inner) => match inner.kind() {
                        Kind::NonUniformSpacing {
                            step_size,
                            positions,
                        } => {
                            assert_eq!(step_size.is_nan(), context.0.is_nan());
                            assert_eq!(step_size.is_infinite(), context.0.is_infinite());
                            assert_eq!(step_size.is_sign_positive(), context.0.is_sign_positive());
                            if step_size.is_finite() && context.0.is_finite() {
                                assert_approx_eq!(f64, *step_size, context.0);
                            };
                            assert_eq!(*positions, context.1);
                        }
                        _ => panic!("unexpected kind: {:?}", inner),
                    },
                    _ => panic!("unexpected error: {:?}", error),
                };
            });
    }

    #[test]
    fn invalid_intensities() {
        let d = (1..=10).map(|i| i as f64).collect::<Vec<f64>>();
        let s = (2.0, 9.0);
        let one_nan = (0..10)
            .map(|i| if i < 9 { 0.0 } else { f64::NAN })
            .collect();
        let one_inf = (0..10)
            .map(|i| if i < 9 { 0.0 } else { f64::INFINITY })
            .collect();
        let one_neg_inf = (0..10)
            .map(|i| if i < 9 { 0.0 } else { f64::NEG_INFINITY })
            .collect();
        let five_nan = (0..10)
            .map(|i| if i < 5 { 0.0 } else { f64::NAN })
            .collect();
        let five_inf = (0..10)
            .map(|i| if i < 5 { 0.0 } else { f64::INFINITY })
            .collect();
        let five_neg_inf = (0..10)
            .map(|i| if i < 5 { 0.0 } else { f64::NEG_INFINITY })
            .collect();
        let all_nan = vec![f64::NAN; 10];
        let all_inf = vec![f64::INFINITY; 10];
        let all_neg_inf = vec![f64::NEG_INFINITY; 10];
        let errors = [
            Spectrum::new(d.clone(), one_nan, s).unwrap_err(),
            Spectrum::new(d.clone(), one_inf, s).unwrap_err(),
            Spectrum::new(d.clone(), one_neg_inf, s).unwrap_err(),
            Spectrum::new(d.clone(), five_nan, s).unwrap_err(),
            Spectrum::new(d.clone(), five_inf, s).unwrap_err(),
            Spectrum::new(d.clone(), five_neg_inf, s).unwrap_err(),
            Spectrum::new(d.clone(), all_nan, s).unwrap_err(),
            Spectrum::new(d.clone(), all_inf, s).unwrap_err(),
            Spectrum::new(d.clone(), all_neg_inf, s).unwrap_err(),
        ];
        let expected_context = [
            (vec![9], 1),
            (vec![9], 1),
            (vec![9], 1),
            (vec![5, 6, 7, 8, 9], 5),
            (vec![5, 6, 7, 8, 9], 5),
            (vec![5, 6, 7, 8, 9], 5),
            ((0..10).collect(), 10),
            ((0..10).collect(), 10),
            ((0..10).collect(), 10),
        ];
        errors
            .into_iter()
            .zip(expected_context)
            .for_each(|(error, context)| {
                match error {
                    Error::Spectrum(inner) => match inner.kind() {
                        Kind::InvalidIntensities { positions } => {
                            assert_eq!(*positions, context.0);
                            assert_eq!(positions.len(), context.1);
                        }
                        _ => panic!("unexpected kind: {:?}", inner),
                    },
                    _ => panic!("unexpected error: {:?}", error),
                };
            });
    }

    #[test]
    fn invalid_signal_boundaries() {
        let d = vec![1.0, 2.0, 3.0];
        let r = (1.0, 3.0);
        let errors = [
            Spectrum::new(d.clone(), d.clone(), (f64::NAN, 3.0)).unwrap_err(),
            Spectrum::new(d.clone(), d.clone(), (0.0, 3.0)).unwrap_err(),
            Spectrum::new(d.clone(), d.clone(), (1.0, 4.0)).unwrap_err(),
            Spectrum::new(d.clone(), d.clone(), (2.0, 2.0)).unwrap_err(),
            Spectrum::new(d.clone(), d.clone(), (4.0, 4.0)).unwrap_err(),
        ];
        let expected_context = [
            ((f64::NAN, 3.0), r),
            ((0.0, 3.0), r),
            ((1.0, 4.0), r),
            ((2.0, 2.0), r),
            ((4.0, 4.0), r),
        ];
        errors
            .into_iter()
            .zip(expected_context)
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
                        _ => panic!("unexpected kind: {:?}", inner),
                    },
                    _ => panic!("unexpected error: {:?}", error),
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
            .set_signal_boundaries((2.5, 1.5))
            .unwrap();
        assert_approx_eq!(f64, spectrum.signal_boundaries().0, 1.5);
        assert_approx_eq!(f64, spectrum.signal_boundaries().1, 2.5);
    }

    #[test]
    fn properties() {
        let spectrum = Spectrum::new(
            vec![1.0, 2.0, 3.0, 4.0, 5.0],
            vec![1.0, 2.0, 3.0, 4.0, 5.0],
            (1.5, 4.5),
        )
        .unwrap();
        assert_eq!(spectrum.len(), 5);
        assert_approx_eq!(f64, spectrum.step(), 1.0);
        assert_approx_eq!(f64, spectrum.width(), 4.0);
        assert_approx_eq!(f64, spectrum.range().0, 1.0);
        assert_approx_eq!(f64, spectrum.range().1, 5.0);
        assert_approx_eq!(f64, spectrum.center(), 3.0);
        assert_eq!(spectrum.signal_boundaries_indices(), (0, 4));
    }
}
