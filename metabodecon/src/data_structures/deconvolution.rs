use crate::data_structures::Lorentzian;
use crate::{FittingAlgo, SmoothingAlgo};

#[derive(Debug, Clone)]
pub struct Deconvolution {
    lorentzians: Vec<Lorentzian>,
    smoothing_algo: SmoothingAlgo,
    noise_threshold: f64,
    fitting_algo: FittingAlgo,
    mse: f64,
}

impl Deconvolution {
    pub fn new(
        lorentzians: Vec<Lorentzian>,
        smoothing_algo: SmoothingAlgo,
        noise_threshold: f64,
        fitting_algo: FittingAlgo,
        mse: f64,
    ) -> Self {
        Self {
            lorentzians,
            smoothing_algo,
            noise_threshold,
            fitting_algo,
            mse,
        }
    }

    pub fn lorenztians(&self) -> &[Lorentzian] {
        &self.lorentzians
    }

    pub fn smoothing_algo(&self) -> &SmoothingAlgo {
        &self.smoothing_algo
    }

    pub fn noise_threshold(&self) -> f64 {
        self.noise_threshold
    }

    pub fn fitting_algo(&self) -> &FittingAlgo {
        &self.fitting_algo
    }

    pub fn mse(&self) -> f64 {
        self.mse
    }
}
