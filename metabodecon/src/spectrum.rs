// file: src/spectrum.rs

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
}
