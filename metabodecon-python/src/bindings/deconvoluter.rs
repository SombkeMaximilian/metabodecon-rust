use crate::bindings::{Deconvolution, Spectrum};
use metabodecon::deconvolution;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyclass]
#[derive(Copy, Clone, Debug)]
pub struct Deconvoluter {
    inner: deconvolution::Deconvoluter,
}

#[pymethods]
impl Deconvoluter {
    #[new]
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_moving_average_smoother(&mut self, iterations: usize, window_size: usize) -> Self {
        self.inner
            .set_smoothing_algo(deconvolution::SmoothingAlgo::MovingAverage {
                iterations,
                window_size,
            });
        *self
    }

    pub fn with_noise_score_selector(&mut self, threshold: f64) -> Self {
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

    pub fn deconvolute_spectrum(&self, spectrum: &mut Spectrum) -> PyResult<Deconvolution> {
        match self
            .inner
            .deconvolute_spectrum(spectrum.inner_mut())
        {
            Ok(deconvolution) => Ok(Deconvolution::from_inner(deconvolution)),
            Err(e) => Err(PyValueError::new_err(e.to_string())),
        }
    }

    pub fn par_deconvolute_spectrum(&self, spectrum: &mut Spectrum) -> PyResult<Deconvolution> {
        match self
            .inner
            .par_deconvolute_spectrum(spectrum.inner_mut())
        {
            Ok(deconvolution) => Ok(Deconvolution::from_inner(deconvolution)),
            Err(e) => Err(PyValueError::new_err(e.to_string())),
        }
    }
}

impl Default for Deconvoluter {
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}
