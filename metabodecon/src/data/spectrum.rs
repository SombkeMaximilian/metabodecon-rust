pub struct Spectrum {
    chemical_shifts: Vec<f64>,
    intensities: Vec<f64>,
    intensities_raw: Vec<f64>,
    signal_boundaries: (f64, f64),
    water_width: f64
}

impl Spectrum {
    pub fn new() -> Self {
        Self {
            chemical_shifts: Vec::new(),
            intensities: Vec::new(),
            intensities_raw: Vec::new(),
            signal_boundaries: (0., 0.),
            water_width: 0.,
        }
    }

    pub fn from_data(
        chemical_shifts: Vec<f64>,
        intensities: Vec<f64>,
        signal_boundaries: (f64, f64),
        water_width: f64
    ) -> Self {
        let intensities_raw = intensities.clone();

        Self {
            chemical_shifts,
            intensities,
            intensities_raw,
            signal_boundaries: signal_boundaries_sorted,
            water_width
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

    pub fn width(&self) -> f64 {
        self.chemical_shifts.last().unwrap() - self.chemical_shifts.first().unwrap()
    }

    pub fn center(&self) -> f64 {
        self.chemical_shifts.first().unwrap() + self.width() / 2.
    }
}
