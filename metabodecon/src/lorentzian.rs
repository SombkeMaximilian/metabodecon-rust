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

    pub fn scale_factor(&self) -> f64 {
        self.sfhw / self.hw2.sqrt()
    }

    pub fn half_width(&self) -> f64 {
        self.hw2.sqrt()
    }

    pub fn maximum_position(&self) -> f64 {
        self.maxp
    }

    pub fn parameters(&self) -> (f64, f64, f64) {
        (self.scale_factor(), self.half_width(), self.maximum_position())
    }

    pub fn set_scale_factor(&mut self, scale_factor: f64) {
        self.sfhw = scale_factor * self.hw2.sqrt();
    }

    pub fn set_half_width(&mut self, half_width: f64) {
        self.sfhw = self.sfhw / self.hw2.sqrt() * half_width.sqrt();
        self.hw2 = half_width.powi(2);
    }

    pub fn set_maximum_position(&mut self, maximum_position: f64) {
        self.maxp = maximum_position;
    }

    pub fn set_parameters(&mut self, scale_factor: f64, half_width: f64, maximum_position: f64) {
        self.set_scale_factor(scale_factor);
        self.set_half_width(half_width);
        self.set_maximum_position(maximum_position);
    }

    pub fn evaluate(&self, x: f64) -> f64 {
        self.sfhw / (self.hw2 + (x - self.maxp).powi(2))
    }
}
