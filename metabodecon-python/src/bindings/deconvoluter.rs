use crate::bindings::{Deconvolution, Spectrum};
use metabodecon::deconvolution;
use pyo3::prelude::*;

#[pyclass]
#[derive(Copy, Clone, Debug)]
pub struct Deconvoluter {
    inner: deconvolution::Deconvoluter,
}

#[pymethods]
impl Deconvoluter {
    #[new]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            inner: deconvolution::Deconvoluter::new(
                deconvolution::SmoothingAlgo::MovingAverage {
                    iterations: 0,
                    window_size: 0,
                },
                deconvolution::SelectionAlgo::NoiseScoreFilter {
                    scoring_algo: deconvolution::ScoringAlgo::MinimumSum,
                    threshold: 0.0,
                },
                deconvolution::FittingAlgo::Analytical { iterations: 0 },
            ),
        }
    }

    pub fn with_ma_smoother(&mut self, iterations: usize, window_size: usize) -> Self {
        self.inner
            .set_smoothing_algo(deconvolution::SmoothingAlgo::MovingAverage {
                iterations,
                window_size,
            });
        *self
    }

    pub fn with_def_selector(&mut self, threshold: f64) -> Self {
        self.inner
            .set_selection_algo(deconvolution::SelectionAlgo::NoiseScoreFilter {
                scoring_algo: deconvolution::ScoringAlgo::MinimumSum,
                threshold,
            });
        *self
    }

    pub fn with_analytical_fitter(&mut self, iterations: usize) -> Self {
        self.inner
            .set_fitting_algo(deconvolution::FittingAlgo::Analytical { iterations });
        *self
    }

    pub fn deconvolute_spectrum(&self, spectrum: &mut Spectrum) -> Deconvolution {
        Deconvolution::from_inner(
            self.inner
                .deconvolute_spectrum(spectrum.inner_mut())
                .unwrap(),
        )
    }

    pub fn par_deconvolute_spectrum(&self, spectrum: &mut Spectrum) -> Deconvolution {
        Deconvolution::from_inner(
            self.inner
                .par_deconvolute_spectrum(spectrum.inner_mut())
                .unwrap(),
        )
    }
}
