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
}
