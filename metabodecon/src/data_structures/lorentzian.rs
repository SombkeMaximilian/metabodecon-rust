#[derive(Debug, Clone, Copy)]
pub struct Lorentzian {
    scale_factor_half_width: f64,
    half_width_squared: f64,
    maximum_position: f64,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accessors() {
        let lorentzian = Lorentzian::new(1.0, 1.0, 0.0);
        assert_eq!(lorentzian.sfhw(), 1.0);
        assert_eq!(lorentzian.hw2(), 1.0);
        assert_eq!(lorentzian.maxp(), 0.0);
        assert_eq!(lorentzian.parameters(), (1.0, 1.0, 0.0));
        assert_eq!(lorentzian.sf(), 1.0);
        assert_eq!(lorentzian.hw(), 1.0);
        assert_eq!(lorentzian.retransformed_parameters(), (1.0, 1.0, 0.0));
    }

    #[test]
    fn mutators() {
        let mut lorentzian = Lorentzian::new(1.0, 1.0, 0.0);
        lorentzian.set_sfhw(2.0);
        lorentzian.set_hw2(2.0);
        lorentzian.set_maxp(1.0);
        assert_eq!(lorentzian.sfhw(), 2.0);
        assert_eq!(lorentzian.hw2(), 2.0);
        assert_eq!(lorentzian.maxp(), 1.0);
        lorentzian.set_parameters(1.0, 1.0, 0.0);
        assert_eq!(lorentzian.parameters(), (1.0, 1.0, 0.0));
    }

    #[test]
    fn evaluate() {
        let lorentzian = Lorentzian::new(1.0, 1.0, 0.0);
        assert_eq!(lorentzian.evaluate(0.0), 1.0);
        assert_eq!(lorentzian.evaluate(1.0), 0.5);
        assert_eq!(lorentzian.evaluate(2.0), 0.2);
    }

    #[test]
    fn evaluate_vec() {
        let lorentzian = Lorentzian::new(1.0, 1.0, 0.0);
        assert_eq!(lorentzian.evaluate_vec(&[0.0, 1.0, 2.0]), vec![1.0, 0.5, 0.2]);
    }
}
