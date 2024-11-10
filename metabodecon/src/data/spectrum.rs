#[derive(Debug, Clone)]
pub struct Spectrum {
    chemical_shifts: Vec<f64>,
    intensities: Vec<f64>,
    intensities_raw: Vec<f64>,
    signal_boundaries: (f64, f64),
    water_width: f64,
}

impl Spectrum {
    pub fn new(
        chemical_shifts: Vec<f64>,
        intensities: Vec<f64>,
        signal_boundaries: (f64, f64),
        water_width: f64,
    ) -> Self {
        let intensities_raw = intensities.clone();
        let signal_boundaries_sorted = (
            f64::min(signal_boundaries.0, signal_boundaries.1),
            f64::max(signal_boundaries.0, signal_boundaries.1),
        );

        Self {
            chemical_shifts,
            intensities,
            intensities_raw,
            signal_boundaries: signal_boundaries_sorted,
            water_width,
        }
    }

    pub fn chemical_shifts(&self) -> &Vec<f64> {
        &self.chemical_shifts
    }

    pub fn intensities(&self) -> &Vec<f64> {
        &self.intensities
    }

    pub fn intensities_mut(&mut self) -> &mut Vec<f64> {
        &mut self.intensities
    }

    pub fn intensities_raw(&self) -> &Vec<f64> {
        &self.intensities_raw
    }

    pub fn signal_boundaries(&self) -> (f64, f64) {
        self.signal_boundaries
    }

    pub fn water_width(&self) -> f64 {
        self.water_width
    }

    pub fn len(&self) -> usize {
        self.chemical_shifts.len()
    }

    pub fn width(&self) -> f64 {
        self.chemical_shifts.last().unwrap() - self.chemical_shifts.first().unwrap()
    }

    pub fn center(&self) -> f64 {
        self.chemical_shifts.first().unwrap() + self.width() / 2.
    }

    pub fn signal_boundaries_indices(&self) -> (usize, usize) {
        let step = self.chemical_shifts[1] - self.chemical_shifts[0];
        (
            ((self.signal_boundaries.0 - self.chemical_shifts[0]) / step).floor() as usize,
            ((self.signal_boundaries.1 - self.chemical_shifts[0]) / step).ceil() as usize,
        )
    }

    pub fn water_boundaries_indices(&self) -> (usize, usize) {
        let step = self.chemical_shifts[1] - self.chemical_shifts[0];
        let half_width = self.water_width / 2.;
        let center = self.len() as f64 / 2.;
        (
            (center - half_width / step).floor() as usize,
            (center + half_width / step).ceil() as usize,
        )
    }
}
