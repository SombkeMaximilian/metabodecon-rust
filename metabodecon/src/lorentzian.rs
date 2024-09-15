// file: src/lorentzian.rs

pub struct Lorentzian {
    sfhw: f64, // A * lambda
    hw2: f64,  // lambda^2
    maxp: f64  // x_0
}

impl Lorentzian {
    pub fn new() -> Self {
        Self { sfhw: 0., hw2: 0., maxp: 0. }
    }

    pub fn from_param(scale_factor: f64, half_width: f64, maximum_position: f64) -> Self {
        Self {
            sfhw: scale_factor * half_width,
            hw2: half_width.powi(2),
            maxp: maximum_position
        }
    }

    pub fn evaluate(&self, x: f64) -> f64 {
        self.sfhw / (self.hw2 + (x - self.maxp).powi(2))
    }
}
