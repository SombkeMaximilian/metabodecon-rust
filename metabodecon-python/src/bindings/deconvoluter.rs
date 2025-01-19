use crate::bindings::{Deconvolution, Spectrum};
use metabodecon::deconvolution;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyclass]
#[derive(Clone, Debug, Default)]
pub struct Deconvoluter {
    inner: deconvolution::Deconvoluter,
}

#[pymethods]
impl Deconvoluter {
    #[new]
    pub fn new() -> Self {
        Deconvoluter::default()
    }

    pub fn with_moving_average_smoother(
        &mut self,
        iterations: usize,
        window_size: usize,
    ) -> PyResult<Self> {
        match self
            .inner
            .set_smoothing_algo(deconvolution::SmoothingAlgo::MovingAverage {
                iterations,
                window_size,
            }) {
            Ok(_) => Ok(self.clone()),
            Err(e) => Err(PyValueError::new_err(e.to_string())),
        }
    }

    pub fn with_noise_score_selector(&mut self, threshold: f64) -> PyResult<Self> {
        match self
            .inner
            .set_selection_algo(deconvolution::SelectionAlgo::NoiseScoreFilter {
                scoring_algo: deconvolution::ScoringAlgo::MinimumSum,
                threshold,
            }) {
            Ok(_) => Ok(self.clone()),
            Err(e) => Err(PyValueError::new_err(e.to_string())),
        }
    }

    pub fn with_analytical_fitter(&mut self, iterations: usize) -> PyResult<Self> {
        match self
            .inner
            .set_fitting_algo(deconvolution::FittingAlgo::Analytical { iterations })
        {
            Ok(_) => Ok(self.clone()),
            Err(e) => Err(PyValueError::new_err(e.to_string())),
        }
    }

    pub fn add_ignore_region(&mut self, new: (f64, f64)) -> PyResult<()> {
        match self.inner.add_ignore_region(new) {
            Ok(_) => Ok(()),
            Err(e) => Err(PyValueError::new_err(e.to_string())),
        }
    }

    pub fn clear_ignore_regions(&mut self) {
        self.inner.clear_ignore_regions();
    }

    pub fn deconvolute_spectrum(&self, spectrum: &Spectrum) -> PyResult<Deconvolution> {
        match self.inner.deconvolute_spectrum(spectrum.inner()) {
            Ok(deconvolution) => Ok(Deconvolution::from_inner(deconvolution)),
            Err(e) => Err(PyValueError::new_err(e.to_string())),
        }
    }

    pub fn par_deconvolute_spectrum(&self, spectrum: &Spectrum) -> PyResult<Deconvolution> {
        match self
            .inner
            .par_deconvolute_spectrum(spectrum.inner())
        {
            Ok(deconvolution) => Ok(Deconvolution::from_inner(deconvolution)),
            Err(e) => Err(PyValueError::new_err(e.to_string())),
        }
    }
}
