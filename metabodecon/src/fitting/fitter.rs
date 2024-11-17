use crate::data_structures::{Lorentzian, Peak, Spectrum};

#[derive(Clone, Copy, Debug)]
pub enum FittingAlgo {
    Analytical { iterations: usize },
}

pub trait Fitter {
    fn fit_lorentzian(&self, spectrum: &Spectrum, peaks: &[Peak]) -> Vec<Lorentzian>;
}
