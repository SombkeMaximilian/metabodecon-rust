use crate::data::{Lorentzian, Peak, Spectrum};

pub enum FittingAlgo {
    Analytical { iterations: usize },
}

pub trait Fitter {
    fn fit_lorentzian(&self, spectrum: &Spectrum, peaks: &[Peak]) -> Vec<Lorentzian>;
}
