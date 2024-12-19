use crate::error::Result;
use crate::smoothing::{MovingAverageSmoother, Smoother, SmoothingAlgo};
use crate::spectrum::error::{Error, Kind};
use crate::spectrum::{
    bruker_reader::BrukerReader, hdf5_reader::Hdf5Reader, jdx_reader::JdxReader,
};
use std::path::Path;

/// `Monotonicity` is a type that represents the ordering of the 1D NMR spectrum
/// data.
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

#[derive(Clone, Debug, Default)]
pub struct Spectrum {
    chemical_shifts: Box<[f64]>,
    intensities: Box<[f64]>,
    intensities_raw: Box<[f64]>,
    signal_boundaries: (f64, f64),
    water_boundaries: (f64, f64),
    monotonicity: Monotonicity,
}

impl Spectrum {
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

    pub fn from_bruker<P: AsRef<Path>>(
        path: P,
        experiment: u32,
        processing: u32,
        signal_boundaries: (f64, f64),
        water_boundaries: (f64, f64),
    ) -> Result<Self> {
        let reader = BrukerReader::new();
        let spectrum = reader.read_spectrum(
            path,
            experiment,
            processing,
            signal_boundaries,
            water_boundaries,
        )?;

        Ok(spectrum)
    }

    pub fn from_bruker_set<P: AsRef<Path>>(
        path: P,
        experiment: u32,
        processing: u32,
        signal_boundaries: (f64, f64),
        water_boundaries: (f64, f64),
    ) -> Result<Vec<Self>> {
        let reader = BrukerReader::new();
        let spectra = reader.read_spectra(
            path,
            experiment,
            processing,
            signal_boundaries,
            water_boundaries,
        )?;

        Ok(spectra)
    }

    pub fn from_jcampdx<P: AsRef<Path>>(
        path: P,
        signal_boundaries: (f64, f64),
        water_boundaries: (f64, f64),
    ) -> Result<Self> {
        let reader = JdxReader::new();
        let mut spectrum = reader.read_spectrum(path, signal_boundaries, water_boundaries)?;
        spectrum.set_signal_boundaries(signal_boundaries);
        spectrum.set_water_boundaries(water_boundaries);

        Ok(spectrum)
    }

    pub fn from_hdf5<P: AsRef<Path>>(path: P, dataset: &str) -> Result<Self> {
        let reader = Hdf5Reader::new();
        reader.read_spectrum(path, dataset)
    }

    pub fn from_hdf5_set<P: AsRef<Path>>(path: P) -> Result<Vec<Self>> {
        let reader = Hdf5Reader::new();
        reader.read_spectra(path)
    }

    pub fn chemical_shifts(&self) -> &[f64] {
        &self.chemical_shifts
    }

    pub fn chemical_shifts_mut(&mut self) -> &mut [f64] {
        &mut self.chemical_shifts
    }

    pub fn intensities(&self) -> &[f64] {
        &self.intensities
    }

    pub fn intensities_mut(&mut self) -> &mut [f64] {
        &mut self.intensities
    }

    pub fn intensities_raw(&self) -> &[f64] {
        &self.intensities_raw
    }

    pub fn intensities_raw_mut(&mut self) -> &mut [f64] {
        &mut self.intensities_raw
    }

    pub fn signal_boundaries(&self) -> (f64, f64) {
        self.signal_boundaries
    }

    pub fn water_boundaries(&self) -> (f64, f64) {
        self.water_boundaries
    }

    pub fn monotonicity(&self) -> Monotonicity {
        self.monotonicity
    }

    pub fn set_chemical_shifts(&mut self, chemical_shifts: Vec<f64>) {
        self.chemical_shifts = chemical_shifts.into_boxed_slice();
    }

    pub fn set_intensities(&mut self, intensities: Vec<f64>) {
        self.intensities = intensities.into_boxed_slice();
    }

    pub fn set_intensities_raw(&mut self, intensities_raw: Vec<f64>) {
        self.intensities_raw = intensities_raw.into_boxed_slice();
    }

    pub fn set_signal_boundaries(&mut self, signal_boundaries: (f64, f64)) {
        self.signal_boundaries = signal_boundaries;
    }

    pub fn set_water_boundaries(&mut self, water_boundaries: (f64, f64)) {
        self.water_boundaries = water_boundaries;
    }

    pub fn len(&self) -> usize {
        self.chemical_shifts.len()
    }

    pub fn is_empty(&self) -> bool {
        self.chemical_shifts.is_empty()
            || self.intensities.is_empty()
            || self.intensities_raw.is_empty()
    }

    pub fn step(&self) -> f64 {
        self.chemical_shifts[1] - self.chemical_shifts[0]
    }

    pub fn width(&self) -> f64 {
        self.chemical_shifts.last().unwrap() - self.chemical_shifts.first().unwrap()
    }

    pub fn center(&self) -> f64 {
        self.chemical_shifts.first().unwrap() + self.width() / 2.
    }

    pub fn signal_boundaries_indices(&self) -> (usize, usize) {
        (
            ((self.signal_boundaries.0 - self.chemical_shifts[0]) / self.step()).floor() as usize,
            ((self.signal_boundaries.1 - self.chemical_shifts[0]) / self.step()).ceil() as usize,
        )
    }

    pub fn water_boundaries_indices(&self) -> (usize, usize) {
        (
            ((self.water_boundaries.0 - self.chemical_shifts[0]) / self.step()).floor() as usize,
            ((self.water_boundaries.1 - self.chemical_shifts[0]) / self.step()).ceil() as usize,
        )
    }

    pub fn apply_preprocessing(&mut self, smoothing_algo: SmoothingAlgo) {
        let water_boundaries_indices = self.water_boundaries_indices();
        let mut intensities = self.intensities_raw().to_vec();
        Self::remove_water_signal(&mut intensities, water_boundaries_indices);
        Self::remove_negative_values(&mut intensities);
        Self::smooth_intensities(&mut intensities, smoothing_algo);
        self.set_intensities(intensities);
    }

    fn remove_water_signal(intensities: &mut [f64], boundary_indices: (usize, usize)) {
        let min_intensity = *intensities
            .iter()
            .min_by(|a, b| a.total_cmp(b))
            .unwrap_or(&0.);
        let water_region = &mut intensities[boundary_indices.0..boundary_indices.1];
        water_region.fill(min_intensity);
    }

    fn remove_negative_values(intensities: &mut [f64]) {
        intensities
            .iter_mut()
            .filter(|intensity| **intensity < 0.0)
            .for_each(|intensity| *intensity = -*intensity);
    }

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
    use crate::MovingAverageAlgo;
    use assert_approx_eq::assert_approx_eq;

    macro_rules! check_sim_spectrum {
        ($spectrum:expr) => {
            assert_eq!($spectrum.chemical_shifts().len(), 2048);
            assert_eq!($spectrum.intensities().len(), 0);
            assert_eq!($spectrum.intensities_raw().len(), 2048);
            assert_approx_eq!($spectrum.signal_boundaries.0, 3.339007);
            assert_approx_eq!($spectrum.signal_boundaries.1, 3.553942);
            assert_approx_eq!($spectrum.water_boundaries.0, 3.444939);
            assert_approx_eq!($spectrum.water_boundaries.1, 3.448010);
        };
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
        spectrum
            .intensities_mut()
            .iter_mut()
            .for_each(|intensity| *intensity = -*intensity);
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
        assert_eq!(spectrum.len(), 5);
        assert!(spectrum.is_empty());
        spectrum.set_intensities(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        assert!(!spectrum.is_empty());
        assert_approx_eq!(spectrum.step(), 1.0);
        assert_approx_eq!(spectrum.width(), 4.0);
        assert_approx_eq!(spectrum.center(), 3.0);
        assert_eq!(spectrum.signal_boundaries_indices(), (0, 4));
        assert_eq!(spectrum.water_boundaries_indices(), (1, 3));
    }

    #[test]
    fn read_from_bruker() {
        let bruker_path = "../data/bruker/sim/sim_01";
        let spectrum = Spectrum::from_bruker(
            bruker_path,
            10,
            10,
            (3.339007, 3.553942),
            (3.444939, 3.448010),
        )
        .unwrap();
        check_sim_spectrum!(spectrum);
    }

    #[test]
    fn read_from_bruker_set() {
        let bruker_path = "../data/bruker/sim";
        let spectra = Spectrum::from_bruker_set(
            bruker_path,
            10,
            10,
            (3.339007, 3.553942),
            (3.444939, 3.448010),
        )
        .unwrap();
        assert_eq!(spectra.len(), 16);
        spectra.iter().for_each(|spectrum| {
            check_sim_spectrum!(spectrum);
        });
    }

    #[test]
    #[should_panic(expected = "Reading JCAMP-DX files is not yet implemented")]
    fn read_from_jcampdx() {
        unimplemented!("Reading JCAMP-DX files is not yet implemented");
    }

    #[test]
    fn read_from_hdf5() {
        let hdf5_path = "../data/hdf5/sim.h5";
        let spectrum = Spectrum::from_hdf5(hdf5_path, "sim_01").unwrap();
        check_sim_spectrum!(spectrum);
    }

    #[test]
    fn read_from_hdf5_set() {
        let hdf5_path = "../data/hdf5/sim.h5";
        let spectra = Spectrum::from_hdf5_set(hdf5_path).unwrap();
        assert_eq!(spectra.len(), 16);
        spectra.iter().for_each(|spectrum| {
            check_sim_spectrum!(spectrum);
        });
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
