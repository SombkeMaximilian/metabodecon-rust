use crate::smoothing::{MovingAverageSmoother, Smoother, SmoothingAlgo};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Spectrum {
    chemical_shifts: Box<[f64]>,
    intensities: Box<[f64]>,
    intensities_raw: Box<[f64]>,
    signal_boundaries: (f64, f64),
    water_boundaries: (f64, f64),
}

impl Spectrum {
    pub fn new(
        chemical_shifts: Vec<f64>,
        intensities: Vec<f64>,
        signal_boundaries: (f64, f64),
        water_boundaries: (f64, f64),
    ) -> Self {
        let signal_boundaries_sorted = (
            f64::min(signal_boundaries.0, signal_boundaries.1),
            f64::max(signal_boundaries.0, signal_boundaries.1),
        );
        let water_boundaries_sorted = (
            f64::min(water_boundaries.0, water_boundaries.1),
            f64::max(water_boundaries.0, water_boundaries.1),
        );

        Self {
            chemical_shifts: chemical_shifts.into_boxed_slice(),
            intensities: Box::new([]),
            intensities_raw: intensities.into_boxed_slice(),
            signal_boundaries: signal_boundaries_sorted,
            water_boundaries: water_boundaries_sorted,
        }
    }

    pub fn from_hdf5<P: AsRef<Path>>(path: P, dataset: &str) -> Result<Self, hdf5::Error> {
        let file = hdf5::File::open(path.as_ref())?;
        let spectrum_group = file.group(dataset)?.group("spectrum")?;
        let data_group = spectrum_group.group("data")?;
        let meta_group = spectrum_group.group("meta")?;

        let chemical_shifts: Vec<f64> = data_group.dataset("chemical_shifts")?.read_1d()?.to_vec();
        let intensities: Vec<f64> = data_group
            .dataset("signal_intensities")?
            .read_1d()?
            .to_vec();
        let signal_boundaries: Vec<f64> =
            meta_group.dataset("signal_boundaries")?.read_1d()?.to_vec();
        let water_boundaries: Vec<f64> =
            meta_group.dataset("water_boundaries")?.read_1d()?.to_vec();

        Ok(Self {
            chemical_shifts: chemical_shifts.into_boxed_slice(),
            intensities: Box::new([]),
            intensities_raw: intensities.into_boxed_slice(),
            signal_boundaries: (signal_boundaries[0], signal_boundaries[1]),
            water_boundaries: (water_boundaries[0], water_boundaries[1]),
        })
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

    #[test]
    fn accessors() {
        let spectrum = Spectrum::new(
            vec![1.0, 2.0, 3.0],
            vec![1.0, 2.0, 3.0],
            (1.0, 3.0),
            (2.0, 2.5),
        );
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
        );
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
        );
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
    fn read_from_hdf5() {
        let spectrum = Spectrum::from_hdf5("../data/sim.h5", "sim_01").unwrap();
        let (signal_start, signal_end) = spectrum.signal_boundaries();
        let (water_start, water_end) = spectrum.water_boundaries();
        assert_eq!(spectrum.chemical_shifts().len(), 2048);
        assert_eq!(spectrum.intensities().len(), 0);
        assert_eq!(spectrum.intensities_raw().len(), 2048);
        assert_approx_eq!(signal_start, 3.339007);
        assert_approx_eq!(signal_end, 3.553942);
        assert_approx_eq!(water_start, 3.444939);
        assert_approx_eq!(water_end, 3.448010);
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
