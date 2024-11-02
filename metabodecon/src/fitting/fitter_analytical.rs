use crate::data::{Lorentzian, Spectrum};
use crate::fitting::Fitter;

pub struct FitterAnalytical {
    iterations: usize,
}

impl Fitter for FitterAnalytical {
    fn fit_lorentzian(&self, _spectrum: &Spectrum) -> Vec<Lorentzian> {
        unimplemented!()
    }
}

impl FitterAnalytical {
    pub fn new(iterations: usize) -> Self {
        Self { iterations }
    }

    pub fn iterations(&self) -> usize {
        self.iterations
    }

    pub fn set_iterations(&mut self, iterations: usize) {
        self.iterations = iterations;
    }

}
