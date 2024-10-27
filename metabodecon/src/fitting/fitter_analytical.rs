use crate::data::{Lorentzian, Spectrum};
use crate::fitting::Fitter;

pub struct FitterAnalytical {}

impl Fitter for FitterAnalytical {
    fn fit_lorentzian(&self, _spectrum: &Spectrum) -> Vec<Lorentzian> {
        unimplemented!()
    }
}

impl FitterAnalytical {
    pub fn new() -> Self {
        Self {}
    }
}
