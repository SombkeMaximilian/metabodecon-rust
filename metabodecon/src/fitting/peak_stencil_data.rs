pub struct PeakStencilData {
    left_chemical_shift: f64,
    center_chemical_shift: f64,
    right_chemical_shift: f64,
    left_intensity: f64,
    center_intensity: f64,
    right_intensity: f64,
}

impl PeakStencilData {
    pub fn new() -> Self {
        Self {
            left_chemical_shift: 0.,
            center_chemical_shift: 0.,
            right_chemical_shift: 0.,
            left_intensity: 0.,
            center_intensity: 0.,
            right_intensity: 0.,
        }
    }

    pub fn from_data(
        left_chemical_shift: f64,
        center_chemical_shift: f64,
        right_chemical_shift: f64,
        left_intensity: f64,
        center_intensity: f64,
        right_intensity: f64,
    ) -> Self {
        Self {
            left_chemical_shift,
            center_chemical_shift,
            right_chemical_shift,
            left_intensity,
            center_intensity,
            right_intensity,
        }
    }

    pub fn x_1(&self) -> f64 {
        self.left_chemical_shift
    }

    pub fn x_2(&self) -> f64 {
        self.center_chemical_shift
    }

    pub fn x_3(&self) -> f64 {
        self.right_chemical_shift
    }

    pub fn y_1(&self) -> f64 {
        self.left_intensity
    }

    pub fn y_2(&self) -> f64 {
        self.center_intensity
    }

    pub fn y_3(&self) -> f64 {
        self.right_intensity
    }

    pub fn set_x_1(&mut self, x_1: f64) {
        self.left_chemical_shift = x_1;
    }

    pub fn set_x_2(&mut self, x_2: f64) {
        self.center_chemical_shift = x_2;
    }

    pub fn set_x_3(&mut self, x_3: f64) {
        self.right_chemical_shift = x_3;
    }

    pub fn set_y_1(&mut self, y_1: f64) {
        self.left_intensity = y_1;
    }

    pub fn set_y_2(&mut self, y_2: f64) {
        self.center_intensity = y_2;
    }

    pub fn set_y_3(&mut self, y_3: f64) {
        self.right_intensity = y_3;
    }

    pub fn mirror_shoulder(&mut self) {
        let increasing = self.left_intensity < self.center_intensity
            && self.center_intensity < self.right_intensity;
        let decreasing = self.left_intensity > self.center_intensity
            && self.center_intensity > self.right_intensity;
        match (increasing, decreasing) {
            (true, _) => {
                self.right_intensity = self.left_intensity;
                self.right_chemical_shift = 2. * self.center_chemical_shift - self.left_chemical_shift;
            },
            (_, true) => {
                self.left_intensity = self.right_intensity;
                self.left_chemical_shift = 2. * self.center_chemical_shift - self.right_chemical_shift;
            },
            _ => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mirror_shoulder() {
        let mut peak = PeakStencilData::from_data(1., 2., 3., 1., 2., 3.);
        peak.mirror_shoulder();
        assert_eq!(peak.x_1(), 1.);
        assert_eq!(peak.x_2(), 2.);
        assert_eq!(peak.x_3(), 3.);
        assert_eq!(peak.y_1(), 1.);
        assert_eq!(peak.y_2(), 2.);
        assert_eq!(peak.y_3(), 1.);

        let mut peak = PeakStencilData::from_data(1., 2., 3., 3., 2., 1.);
        peak.mirror_shoulder();
        assert_eq!(peak.x_1(), 1.);
        assert_eq!(peak.x_2(), 2.);
        assert_eq!(peak.x_3(), 3.);
        assert_eq!(peak.y_1(), 1.);
        assert_eq!(peak.y_2(), 2.);
        assert_eq!(peak.y_3(), 1.);
    }
}
