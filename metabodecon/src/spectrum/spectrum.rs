use crate::error::Result;
use crate::smoothing::{MovingAverageSmoother, Smoother, SmoothingAlgo};
use crate::spectrum::error::{Error, Kind};

/// Represents the ordering of 1D NMR spectrum data.
///
/// Typically, 1D NMR data is ordered in [`Decreasing`] order of chemical
/// shifts, but this is not always the case. Additionally, it is often simpler
/// to work with the data if it is ordered in [`Increasing`] order, and only
/// reorder it for display purposes.
///
/// [`Increasing`]: Monotonicity::Increasing
/// [`Decreasing`]: Monotonicity::Decreasing
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub enum Monotonicity {
    /// The data is ordered in increasing order of chemical shifts.
    #[default]
    Increasing,
    /// The data is ordered in decreasing order of chemical shifts.
    Decreasing,
}

impl Monotonicity {
    /// Helper function to determine the [`Monotonicity`] of a [`Spectrum`].
    /// Checks for the ordering of two floating point numbers and returns the
    /// corresponding [`Monotonicity`] variant. If the two numbers are equal
    /// or cannot be compared, a [`NonUniformSpacing`] error is returned.
    ///
    /// [`NonUniformSpacing`]: Kind::NonUniformSpacing
    pub(crate) fn from_f64s(first: f64, second: f64) -> Result<Self> {
        match first.partial_cmp(&second) {
            Some(std::cmp::Ordering::Less) => Ok(Self::Increasing),
            Some(std::cmp::Ordering::Greater) => Ok(Self::Decreasing),
            _ => Err(Error::new(Kind::NonUniformSpacing { positions: (0, 1) }).into()),
        }
    }
}

/// Data structure that represents a 1D NMR spectrum.
///
/// `Spectrum` is a container that holds the chemical shifts, raw intensities,
/// preprocessed intensities and metadata of a 1D NMR spectrum. Preprocessed
/// intensities are empty when the Spectrum is created. 1D NMR spectra typically
/// contain signal free regions on both ends of the frequency range, and a water
/// artifact within the signal region. The boundaries of the signal region and
/// water artifact are stored in the spectrum object as tuples.
#[derive(Clone, Debug, Default)]
pub struct Spectrum {
    /// The chemical shifts of the spectrum.
    chemical_shifts: Box<[f64]>,
    /// The preprocessed intensities of the spectrum.
    intensities: Box<[f64]>,
    /// The raw intensities of the spectrum.
    intensities_raw: Box<[f64]>,
    /// The boundaries of the signal region.
    signal_boundaries: (f64, f64),
    /// The boundaries of the water artifact.
    water_boundaries: (f64, f64),
    /// The monotonicity of the spectrum. Used internally to validate the data.
    monotonicity: Monotonicity,
}

impl Spectrum {
    /// Constructs a `Spectrum` from the given data. Performs some basic checks
    /// on the input data to validate it.
    pub fn new(
        chemical_shifts: Vec<f64>,
        intensities: Vec<f64>,
        signal_boundaries: (f64, f64),
        water_boundaries: (f64, f64),
    ) -> Result<Self> {
        if chemical_shifts.is_empty() && intensities.is_empty() {
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

        let step_size = chemical_shifts[1] - chemical_shifts[0];
        if step_size.abs() < f64::EPSILON {
            return Err(Error::new(Kind::NonUniformSpacing { positions: (0, 1) }).into());
        }

        if let Some(position) = chemical_shifts
            .windows(2)
            .position(|w| (w[1] - w[0] - step_size).abs() > 100.0 * f64::EPSILON)
        {
            let _values = (chemical_shifts[position], chemical_shifts[position + 1]);
            let _diff = (chemical_shifts[position + 1] - chemical_shifts[position]).abs();
            return Err(Error::new(Kind::NonUniformSpacing {
                positions: (position, position + 1),
            })
            .into());
        }

        let monotonicity = {
            let chemical_shifts_monotonicity =
                Monotonicity::from_f64s(chemical_shifts[0], chemical_shifts[1])?;
            let signal_boundaries_monotonicity =
                Monotonicity::from_f64s(signal_boundaries.0, signal_boundaries.1)?;
            let water_boundaries_monotonicity =
                Monotonicity::from_f64s(water_boundaries.0, water_boundaries.1)?;

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
            chemical_shifts_monotonicity
        };

        Ok(Self {
            chemical_shifts: chemical_shifts.into_boxed_slice(),
            intensities: Box::new([]),
            intensities_raw: intensities.into_boxed_slice(),
            signal_boundaries,
            water_boundaries,
            monotonicity,
        })
    }

    /// Returns the chemical shifts as a slice.
    pub fn chemical_shifts(&self) -> &[f64] {
        &self.chemical_shifts
    }

    /// Returns the preprocessed intensities as a slice.
    pub fn intensities(&self) -> &[f64] {
        &self.intensities
    }

    /// Returns the raw intensities as a slice.
    pub fn intensities_raw(&self) -> &[f64] {
        &self.intensities_raw
    }

    /// Returns the signal boundaries as a tuple.
    pub fn signal_boundaries(&self) -> (f64, f64) {
        self.signal_boundaries
    }

    /// Returns the water boundaries as a tuple.
    pub fn water_boundaries(&self) -> (f64, f64) {
        self.water_boundaries
    }

    /// Returns the monotonicity of the spectrum.
    pub fn monotonicity(&self) -> Monotonicity {
        self.monotonicity
    }

    /// Sets the chemical shifts of the spectrum. This currently does not
    /// perform any monotonicity validation on the input data.
    pub fn set_chemical_shifts(&mut self, chemical_shifts: Vec<f64>) {
        self.chemical_shifts = chemical_shifts.into_boxed_slice();
    }

    /// Sets the preprocessed intensities of the spectrum.
    pub fn set_intensities(&mut self, intensities: Vec<f64>) {
        self.intensities = intensities.into_boxed_slice();
    }

    /// Sets the raw intensities of the spectrum.
    pub fn set_intensities_raw(&mut self, intensities_raw: Vec<f64>) {
        self.intensities_raw = intensities_raw.into_boxed_slice();
    }

    /// Sets the signal boundaries of the spectrum. This currently does not
    /// perform any monotonicity validation on the input data.
    pub fn set_signal_boundaries(&mut self, signal_boundaries: (f64, f64)) {
        self.signal_boundaries = signal_boundaries;
    }

    /// Sets the water boundaries of the spectrum. This currently does not
    /// perform any monotonicity validation on the input data.
    pub fn set_water_boundaries(&mut self, water_boundaries: (f64, f64)) {
        self.water_boundaries = water_boundaries;
    }

    /// Computes the step size between two consecutive chemical shifts in ppm.
    pub fn step(&self) -> f64 {
        self.chemical_shifts[1] - self.chemical_shifts[0]
    }

    /// Computes the width of the spectrum in ppm.
    pub fn width(&self) -> f64 {
        self.chemical_shifts.last().unwrap() - self.chemical_shifts.first().unwrap()
    }

    /// Computes the center of the spectrum in ppm.
    pub fn center(&self) -> f64 {
        self.chemical_shifts.first().unwrap() + self.width() / 2.
    }

    /// Computes the indices of the chemical shifts that are closest to the
    /// signal region boundaries.
    pub fn signal_boundaries_indices(&self) -> (usize, usize) {
        (
            ((self.signal_boundaries.0 - self.chemical_shifts[0]) / self.step()).floor() as usize,
            ((self.signal_boundaries.1 - self.chemical_shifts[0]) / self.step()).ceil() as usize,
        )
    }

    /// Computes the indices of the chemical shifts that are closest to the
    /// water artifact boundaries.
    pub fn water_boundaries_indices(&self) -> (usize, usize) {
        (
            ((self.water_boundaries.0 - self.chemical_shifts[0]) / self.step()).floor() as usize,
            ((self.water_boundaries.1 - self.chemical_shifts[0]) / self.step()).ceil() as usize,
        )
    }

    /// Applies preprocessing to the raw intensities of the spectrum and stores
    /// the result in the intensities. The preprocessing steps include:
    /// 1. Removing the water signal from the intensities.
    /// 2. Removing negative values from the intensities.
    /// 3. Smoothing the intensities using the specified [`SmoothingAlgo`].
    pub fn apply_preprocessing(&mut self, smoothing_algo: SmoothingAlgo) {
        let water_boundaries_indices = self.water_boundaries_indices();
        let mut intensities = self.intensities_raw().to_vec();
        Self::remove_water_signal(&mut intensities, water_boundaries_indices);
        Self::remove_negative_values(&mut intensities);
        Self::smooth_intensities(&mut intensities, smoothing_algo);
        self.set_intensities(intensities);
    }

    /// Removes the water signal from the provided intensities.
    fn remove_water_signal(intensities: &mut [f64], boundary_indices: (usize, usize)) {
        let min_intensity = *intensities
            .iter()
            .min_by(|a, b| a.total_cmp(b))
            .unwrap_or(&0.);
        let water_region = &mut intensities[boundary_indices.0..boundary_indices.1];
        water_region.fill(min_intensity);
    }

    /// Removes negative values from the provided intensities.
    fn remove_negative_values(intensities: &mut [f64]) {
        intensities
            .iter_mut()
            .filter(|intensity| **intensity < 0.0)
            .for_each(|intensity| *intensity = -*intensity);
    }

    /// Smooths the intensities using the specified algorithm.
    fn smooth_intensities(intensities: &mut [f64], algorithm: SmoothingAlgo) {
        match algorithm {
            SmoothingAlgo::MovingAverage {
                algo,
                iterations,
                window_size,
            } => {
                let mut smoother = MovingAverageSmoother::<f64>::new(algo, iterations, window_size);
                smoother.smooth_values(intensities);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::smoothing::MovingAverageAlgo;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn accessors() {
        let spectrum = Spectrum::new(
            vec![1.0, 2.0, 3.0],
            vec![1.0, 2.0, 3.0],
            (1.0, 3.0),
            (2.0, 2.5),
        )
        .unwrap();
        assert_eq!(spectrum.chemical_shifts(), &[1.0, 2.0, 3.0]);
        assert_eq!(spectrum.intensities(), &[]);
        assert_eq!(spectrum.intensities_raw(), &[1.0, 2.0, 3.0]);
        assert_eq!(spectrum.signal_boundaries(), (1.0, 3.0));
        assert_eq!(spectrum.water_boundaries(), (2.0, 2.5));
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
        spectrum.set_chemical_shifts(vec![1.0, 2.0, 3.0, 4.0]);
        spectrum.set_intensities_raw(vec![1.0, 2.0, 3.0, 4.0]);
        spectrum.set_intensities(vec![1.0, 2.0, 3.0, 4.0]);
        spectrum.set_signal_boundaries((1.0, 4.0));
        spectrum.set_water_boundaries((2.5, 3.0));
        spectrum.set_intensities(
            spectrum
                .intensities()
                .iter()
                .map(|intensity| -intensity)
                .collect(),
        );
        assert_eq!(spectrum.intensities(), &[-1.0, -2.0, -3.0, -4.0]);
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
        spectrum.set_intensities(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        assert_approx_eq!(spectrum.step(), 1.0);
        assert_approx_eq!(spectrum.width(), 4.0);
        assert_approx_eq!(spectrum.center(), 3.0);
        assert_eq!(spectrum.signal_boundaries_indices(), (0, 4));
        assert_eq!(spectrum.water_boundaries_indices(), (1, 3));
    }

    #[test]
    fn remove_water_signal() {
        let mut intensities = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let water_boundaries_indices = (1, 4);
        Spectrum::remove_water_signal(&mut intensities, water_boundaries_indices);
        assert_eq!(intensities, vec![1.0, 1.0, 1.0, 1.0, 5.0]);
    }

    #[test]
    fn remove_negative_values() {
        let mut intensities = vec![1.0, -2.0, 3.0, -4.0, 5.0];
        Spectrum::remove_negative_values(&mut intensities);
        assert_eq!(intensities, vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    }

    #[test]
    fn smooth_intensities() {
        let mut intensities = vec![1.25, 1.75, 1.5, 2.0, 1.75];
        let algorithm = SmoothingAlgo::MovingAverage {
            algo: MovingAverageAlgo::Simple,
            iterations: 1,
            window_size: 3,
        };
        Spectrum::smooth_intensities(&mut intensities, algorithm);
        assert_eq!(intensities, vec![1.5, 1.5, 1.75, 1.75, 1.875]);
    }
}
