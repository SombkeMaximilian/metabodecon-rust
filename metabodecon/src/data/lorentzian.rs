pub struct Lorentzian {
    scale_factor: f64,
    half_width: f64,
    max_position: f64
}

impl Lorentzian {
    pub fn new() -> Self {
        Self {
            scale_factor: 0.,
            half_width: 0.,
            max_position: 0.
        }
    }

    pub fn from_param(scale_factor: f64, half_width: f64, max_position: f64) -> Self {
        Self {
            scale_factor,
            half_width,
            max_position
        }
    }

    pub fn scale_factor(&self) -> f64 {
        self.scale_factor
    }

    pub fn half_width(&self) -> f64 {
        self.half_width
    }

    pub fn maximum_position(&self) -> f64 {
        self.max_position
    }

    pub fn parameters(&self) -> (f64, f64, f64) {
        (self.scale_factor(), self.half_width(), self.maximum_position())
    }

    pub fn set_scale_factor(&mut self, scale_factor: f64) {
        self.scale_factor = scale_factor;
    }

    pub fn set_half_width(&mut self, half_width: f64) {
        self.half_width = half_width;
    }

    pub fn set_maximum_position(&mut self, maximum_position: f64) {
        self.max_position = maximum_position;
    }

    pub fn set_parameters(&mut self, scale_factor: f64, half_width: f64, maximum_position: f64) {
        self.scale_factor = scale_factor;
        self.half_width = half_width;
        self.max_position = maximum_position;
    }

    pub fn evaluate(&self, x: f64) -> f64 {
        self.scale_factor * self.half_width /
            (self.half_width.powi(2) + (x - self.max_position).powi(2))
    }
}
