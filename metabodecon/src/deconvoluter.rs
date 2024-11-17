use crate::data_structures::{Deconvolution, Spectrum};
use crate::fitting::{Fitter, FitterAnalytical, FittingAlgo};
use crate::peak_selection::select_peaks;
use crate::preprocessing::preprocess_spectrum;
use crate::smoothing::SmoothingAlgo;

#[derive(Debug, Clone, Copy)]
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

    pub fn deconvolute_spectrum(&self, spectrum: &mut Spectrum) -> Deconvolution {
        preprocess_spectrum(spectrum, self.smoothing_algo);
        let peaks = select_peaks(spectrum.clone(), self.noise_threshold);
        let lorentzians = {
            match self.fitting_algo {
                FittingAlgo::Analytical { iterations } => {
                    let fitter = FitterAnalytical::new(iterations);
                    fitter.fit_lorentzian(spectrum, &peaks)
                }
            }
        };
        let mse = lorentzians
            .iter()
            .map(|l| l.evaluate_vec(spectrum.chemical_shifts()))
            .fold(vec![0.; spectrum.chemical_shifts().len()], |acc, x| {
                acc.iter()
                    .zip(x.iter())
                    .map(|(a, b)| a + b)
                    .collect::<Vec<_>>()
            })
            .into_iter()
            .zip(spectrum.intensities_raw().iter())
            .map(|(superposition, raw)| (superposition - raw).powi(2))
            .sum::<f64>()
            / spectrum.intensities_raw().len() as f64;

        Deconvolution::new(
            lorentzians,
            self.smoothing_algo,
            self.noise_threshold,
            self.fitting_algo,
            mse,
        )
    }
}
