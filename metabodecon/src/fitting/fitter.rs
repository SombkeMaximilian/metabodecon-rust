use crate::data::{Lorentzian, Spectrum};

pub enum FittingAlgo {
    Analytical { iterations: usize },
}

pub trait Fitter {
    fn fit_lorentzian(&self, spectrum: &Spectrum) -> Vec<Lorentzian>;
}
