pub struct Spectrum {
    chemical_shifts: Vec<f64>,
    intensities: Vec<f64>,
    signal_region_boundaries: (f64, f64),
    water_region_width: f64
}

impl Spectrum {
    pub fn new() -> Self {
        Self {
            chemical_shifts: Vec::new(),
            intensities: Vec::new(),
            signal_region_boundaries: (0., 0.),
            water_region_width: 0.,
        }
    }

    pub fn from_data(
        chemical_shifts: Vec<f64>,
        intensities: Vec<f64>,
        signal_region_boundaries: (f64, f64),
        water_region_width: f64
    ) -> Self {
        Self {
            chemical_shifts,
            intensities,
            signal_region_boundaries,
            water_region_width
        }
    }

    pub fn chemical_shifts(&self) -> &Vec<f64> {
        &self.chemical_shifts
    }

    pub fn intensities(&self) -> &Vec<f64> {
        &self.intensities
    }

    pub fn signal_region_boundaries(&self) -> (f64, f64) {
        self.signal_region_boundaries
    }

    pub fn water_region_width(&self) -> f64 {
        self.water_region_width
    }
}
