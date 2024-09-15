// file: src/lorentzian.rs

pub struct Lorentzian {
    scaling_factor: f64,
    half_width: f64,
    maximum_position: f64
}

impl Lorentzian {
    pub fn new() -> Self {
        Self {
            scaling_factor: 0.,
            half_width: 0.,
            maximum_position: 0.
        }
    }
}
