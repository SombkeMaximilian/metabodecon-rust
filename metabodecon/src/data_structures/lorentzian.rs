#[derive(Debug, Clone, Copy)]
pub struct Lorentzian {
    scale_factor_half_width: f64, // A * lambda
    half_width_squared: f64,      // lambda^2
    maximum_position: f64,        // x_0
}

impl Lorentzian {
    pub fn new(sfhw: f64, hw2: f64, maxp: f64) -> Self {
        Self {
            scale_factor_half_width: sfhw,
            half_width_squared: hw2,
            maximum_position: maxp,
        }
    }

    pub fn sfhw(&self) -> f64 {
        self.scale_factor_half_width
    }

    pub fn hw2(&self) -> f64 {
        self.half_width_squared
    }

    pub fn maxp(&self) -> f64 {
        self.maximum_position
    }

    pub fn parameters(&self) -> (f64, f64, f64) {
        (self.sfhw(), self.hw2(), self.maxp())
    }

    pub fn set_sfhw(&mut self, scale_factor: f64) {
        self.scale_factor_half_width = scale_factor;
    }

    pub fn set_hw2(&mut self, half_width: f64) {
        self.half_width_squared = half_width;
    }

    pub fn set_maxp(&mut self, max_position: f64) {
        self.maximum_position = max_position;
    }

    pub fn set_parameters(&mut self, sfhw: f64, hw2: f64, maxp: f64) {
        self.scale_factor_half_width = sfhw;
        self.half_width_squared = hw2;
        self.maximum_position = maxp;
    }

    pub fn sf(&self) -> f64 {
        self.scale_factor_half_width / self.hw()
    }

    pub fn hw(&self) -> f64 {
        self.half_width_squared.sqrt()
    }

    pub fn retransformed_parameters(&self) -> (f64, f64, f64) {
        (self.sf(), self.hw(), self.maxp())
    }

    pub fn evaluate(&self, x: f64) -> f64 {
        self.scale_factor_half_width
            / (self.half_width_squared + (x - self.maximum_position).powi(2))
    }

    pub fn evaluate_vec(&self, x: &[f64]) -> Vec<f64> {
        x.iter().map(|&x| self.evaluate(x)).collect()
    }

    pub fn integral(&self) -> f64 {
        std::f64::consts::PI * self.sf()
    }
}