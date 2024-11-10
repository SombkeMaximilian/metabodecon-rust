use crate::data::{Deconvolution, Spectrum};
use crate::fitting::FittingAlgo;
use crate::smoothing::SmoothingAlgo;

pub struct Deconvoluter {
    smoothing_algo: SmoothingAlgo,
    noise_threshold: f64,
    fitting_algo: FittingAlgo,
}

impl Deconvoluter {
    pub fn new(
        smoothing_algo: SmoothingAlgo,
        noise_threshold: f64,
        fitting_algo: FittingAlgo,
    ) -> Deconvoluter {
        Deconvoluter {
            smoothing_algo,
            noise_threshold,
            fitting_algo,
        }
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

    pub fn set_smoothing_algo(&mut self, smoothing_algo: SmoothingAlgo) {
        self.smoothing_algo = smoothing_algo;
    }

    pub fn set_noise_threshold(&mut self, noise_threshold: f64) {
        self.noise_threshold = noise_threshold;
    }

    pub fn set_fitting_algo(&mut self, fitting_algo: FittingAlgo) {
        self.fitting_algo = fitting_algo;
    }

    pub fn deconvolute_spectrum(&self, _spectrum: &mut Spectrum) -> Deconvolution {
        unimplemented!()
    }
}
