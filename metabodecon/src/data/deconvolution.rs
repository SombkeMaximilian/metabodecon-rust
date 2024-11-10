use crate::data::Lorentzian;

pub struct Deconvolution {
    lorentzians: Vec<Lorentzian>,
    mse: f64,
}

impl Deconvolution {
    pub fn new() -> Self {
        Self {
            lorentzians: Vec::new(),
            mse: 0.0,
        }
    }

    pub fn lorenztians(&self) -> &Vec<Lorentzian> {
        &self.lorentzians
    }
    pub fn mse(&self) -> f64 {
        self.mse
    }
}
