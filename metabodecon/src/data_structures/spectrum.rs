#[derive(Debug, Clone)]
pub struct Spectrum {
    chemical_shifts: Vec<f64>,
    intensities: Vec<f64>,
    intensities_raw: Vec<f64>,
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
        let intensities_raw = intensities.clone();
        let signal_boundaries_sorted = (
            f64::min(signal_boundaries.0, signal_boundaries.1),
            f64::max(signal_boundaries.0, signal_boundaries.1),
        );
        let water_boundaries_sorted = (
            f64::min(water_boundaries.0, water_boundaries.1),
            f64::max(water_boundaries.0, water_boundaries.1),
        );

        Self {
            chemical_shifts,
            intensities,
            intensities_raw,
            signal_boundaries: signal_boundaries_sorted,
            water_boundaries: water_boundaries_sorted,
        }
    }

    pub fn from_hdf5(path: &str, dataset: &str) -> Result<Self, hdf5::Error> {
        let file = hdf5::File::open(path)?;
        let spectrum_group = file.group(dataset)?.group("spectrum")?;
        let data_group = spectrum_group.group("data")?;
        let meta_group = spectrum_group.group("meta")?;

        let chemical_shifts: Vec<f64> = data_group.dataset("chemical_shifts")?.read_1d()?.to_vec();
        let intensities: Vec<f64> = data_group.dataset("signal_intensities")?.read_1d()?.to_vec();
        let intensities_raw: Vec<f64> = intensities.clone();
        let signal_boundaries: Vec<f64> =
            meta_group.dataset("signal_boundaries")?.read_1d()?.to_vec();
        let water_boundaries: Vec<f64> =
            meta_group.dataset("water_boundaries")?.read_1d()?.to_vec();

        Ok(Self {
            chemical_shifts,
            intensities,
            intensities_raw,
            signal_boundaries: (signal_boundaries[0], signal_boundaries[1]),
            water_boundaries: (water_boundaries[0], water_boundaries[1]),
        })
    }

    pub fn chemical_shifts(&self) -> &[f64] {
        &self.chemical_shifts
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

    pub fn signal_boundaries(&self) -> (f64, f64) {
        self.signal_boundaries
    }

    pub fn water_boundaries(&self) -> (f64, f64) {
        self.water_boundaries
    }

    pub fn len(&self) -> usize {
        self.chemical_shifts.len()
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn read_from_hdf5() {
        let spectrum = Spectrum::from_hdf5("data/sim.h5", "sim_01").unwrap();
        let (signal_start, signal_end) = spectrum.signal_boundaries();
        let (water_start, water_end) = spectrum.water_boundaries();
        assert_eq!(spectrum.chemical_shifts().len(), 2048);
        assert_eq!(spectrum.intensities().len(), 2048);
        assert_eq!(spectrum.intensities_raw().len(), 2048);
        assert_approx_eq!(signal_start, 3.339007);
        assert_approx_eq!(signal_end, 3.553942);
        assert_approx_eq!(water_start, 3.444939);
        assert_approx_eq!(water_end, 3.448010);
    }
}
