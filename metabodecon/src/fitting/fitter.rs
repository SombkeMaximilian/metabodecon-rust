use crate::data::{Lorentzian, Spectrum};

pub enum FittingAlgo {
    Analytical {
        iterations: usize,
    },
    GradientDescent {
        iterations: usize,
        learning_rate: f64,
    },
}

pub trait Fitter {
    fn fit_lorentzian(&self, spectrum: &Spectrum) -> Vec<Lorentzian>;
}
