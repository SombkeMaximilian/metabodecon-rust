use crate::deconvolution::Deconvolution;
use crate::error::Result;
use crate::fitting::{Fitter, FitterAnalytical, FittingAlgo, Lorentzian};
use crate::peak_selection::{NoiseScoreFilter, SelectionAlgo, Selector};
use crate::smoothing::{MovingAverage, Smoother, SmoothingAlgo};
use crate::spectrum::Spectrum;

/// Deconvolution pipeline that applies smoothing, peak selection, and fitting
/// to a spectrum to deconvolute it into individual signals.
#[derive(Copy, Clone, Debug, Default)]
pub struct Deconvoluter {
    /// The smoothing algorithm to use.
    smoothing_algo: SmoothingAlgo,
    /// The peak selection algorithm to use.
    selection_algo: SelectionAlgo,
    /// The fitting algorithm to use.
    fitting_algo: FittingAlgo,
}

impl Deconvoluter {
    /// Constructs a new `Deconvoluter` with the provided settings.
    pub fn new(
        smoothing_algo: SmoothingAlgo,
        selection_algo: SelectionAlgo,
        fitting_algo: FittingAlgo,
    ) -> Self {
        Self {
            smoothing_algo,
            selection_algo,
            fitting_algo,
        }
    }

    /// Returns the smoothing algorithm settings.
    pub fn smoothing_algo(&self) -> SmoothingAlgo {
        self.smoothing_algo
    }

    /// Returns the peak selection algorithm settings.
    pub fn selection_algo(&self) -> SelectionAlgo {
        self.selection_algo
    }

    /// Returns the fitting algorithm settings.
    pub fn fitting_algo(&self) -> FittingAlgo {
        self.fitting_algo
    }

    /// Sets the smoothing algorithm settings.
    pub fn set_smoothing_algo(&mut self, smoothing_algo: SmoothingAlgo) {
        self.smoothing_algo = smoothing_algo;
    }

    /// Sets the peak selection algorithm settings.
    pub fn set_selection_algo(&mut self, selection_algo: SelectionAlgo) {
        self.selection_algo = selection_algo;
    }

    /// Sets the fitting algorithm settings.
    pub fn set_fitting_algo(&mut self, fitting_algo: FittingAlgo) {
        self.fitting_algo = fitting_algo;
    }

    /// Deconvolutes the provided spectrum into individual signals.
    pub fn deconvolute_spectrum(&self, spectrum: &mut Spectrum) -> Result<Deconvolution> {
        spectrum.set_intensities(spectrum.intensities_raw().to_vec())?;
        let mut smoother = match self.smoothing_algo {
            SmoothingAlgo::MovingAverage {
                iterations,
                window_size,
            } => MovingAverage::new(iterations, window_size),
        };
        smoother.smooth_values(spectrum.intensities_mut());
        let peaks = {
            let selector = match self.selection_algo {
                SelectionAlgo::NoiseScoreFilter {
                    scoring_algo,
                    threshold,
                } => NoiseScoreFilter::new(scoring_algo, threshold),
            };
            selector.select_peaks(spectrum)?
        };
        let mut lorentzians = {
            let fitter = match self.fitting_algo {
                FittingAlgo::Analytical { iterations } => FitterAnalytical::new(iterations),
            };
            fitter.fit_lorentzian(spectrum, &peaks)
        };
        let mse = Self::compute_mse(
            spectrum,
            Lorentzian::superposition_vec(spectrum.chemical_shifts(), &lorentzians),
        );
        lorentzians
            .retain(|lorentzian| lorentzian.sfhw() > 0.0 && lorentzian.hw2() > 0.0);

        Ok(Deconvolution::new(
            lorentzians,
            self.smoothing_algo,
            self.selection_algo,
            self.fitting_algo,
            mse,
        ))
    }

    /// Deconvolutes the provided spectrum into individual signals in parallel.
    #[cfg(feature = "parallel")]
    pub fn par_deconvolute_spectrum(&self, spectrum: &mut Spectrum) -> Result<Deconvolution> {
        spectrum.set_intensities(spectrum.intensities_raw().to_vec())?;
        let mut smoother = match self.smoothing_algo {
            SmoothingAlgo::MovingAverage {
                iterations,
                window_size,
            } => MovingAverage::new(iterations, window_size),
        };
        smoother.smooth_values(spectrum.intensities_mut());
        let peaks = {
            let selector = match self.selection_algo {
                SelectionAlgo::NoiseScoreFilter {
                    threshold,
                    scoring_algo,
                } => NoiseScoreFilter::new(scoring_algo, threshold),
            };
            selector.select_peaks(spectrum)?
        };
        let mut lorentzians = {
            let fitter = match self.fitting_algo {
                FittingAlgo::Analytical { iterations } => FitterAnalytical::new(iterations),
            };
            fitter.par_fit_lorentzian(spectrum, &peaks)
        };
        let mse = Self::compute_mse(
            spectrum,
            Lorentzian::par_superposition_vec(spectrum.chemical_shifts(), &lorentzians),
        );
        lorentzians
            .retain(|lorentzian| lorentzian.sfhw() > 0.0 && lorentzian.hw2() > 0.0);

        Ok(Deconvolution::new(
            lorentzians,
            self.smoothing_algo,
            self.selection_algo,
            self.fitting_algo,
            mse,
        ))
    }

    /// Internal helper function to compute the MSE within the signal region.
    fn compute_mse(spectrum: &Spectrum, superpositions: Vec<f64>) -> f64 {
        let signal_boundaries = spectrum.signal_boundaries_indices();
        let residuals = superpositions[signal_boundaries.0..signal_boundaries.1]
            .iter()
            .zip(spectrum.intensities_raw()[signal_boundaries.0..signal_boundaries.1].iter())
            .map(|(superposition, raw)| (superposition - raw).powi(2))
            .sum::<f64>();
        let length = signal_boundaries.1 - signal_boundaries.0;

        residuals / (length as f64)
    }
}
