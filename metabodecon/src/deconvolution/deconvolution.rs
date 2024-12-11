use crate::fitting::{FittingAlgo, Lorentzian};
use crate::peak_selection::SelectionAlgo;
use crate::smoothing::SmoothingAlgo;

#[derive(Clone, Debug)]
pub struct Deconvolution {
    lorentzians: Vec<Lorentzian>,
    smoothing_algo: SmoothingAlgo,
    selection_algo: SelectionAlgo,
    fitting_algo: FittingAlgo,
    mse: f64,
}

impl Deconvolution {
    pub fn new(
        lorentzians: Vec<Lorentzian>,
        smoothing_algo: SmoothingAlgo,
        selection_algo: SelectionAlgo,
        fitting_algo: FittingAlgo,
        mse: f64,
    ) -> Self {
        Self {
            lorentzians,
            smoothing_algo,
            selection_algo,
            fitting_algo,
            mse,
        }
    }

    pub fn lorentzians(&self) -> &[Lorentzian] {
        &self.lorentzians
    }

    pub fn smoothing_algo(&self) -> &SmoothingAlgo {
        &self.smoothing_algo
    }

    pub fn selection_algo(&self) -> &SelectionAlgo {
        &self.selection_algo
    }

    pub fn fitting_algo(&self) -> &FittingAlgo {
        &self.fitting_algo
    }

    pub fn mse(&self) -> f64 {
        self.mse
    }
}
