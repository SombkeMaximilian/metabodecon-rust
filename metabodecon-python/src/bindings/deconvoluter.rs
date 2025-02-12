use crate::MetabodeconError;
use crate::bindings::{Deconvolution, Spectrum};
use metabodecon::deconvolution;
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
        Self::default()
    }

    pub fn set_moving_average_smoother(
        &mut self,
        iterations: usize,
        window_size: usize,
    ) -> PyResult<()> {
        match self
            .inner
            .set_smoothing_settings(deconvolution::SmoothingSettings::MovingAverage {
                iterations,
                window_size,
            }) {
            Ok(_) => Ok(()),
            Err(e) => Err(MetabodeconError::from(e).into()),
        }
    }

    pub fn set_noise_score_selector(&mut self, threshold: f64) -> PyResult<()> {
        match self.inner.set_selection_settings(
            deconvolution::SelectionSettings::NoiseScoreFilter {
                scoring_method: deconvolution::ScoringMethod::MinimumSum,
                threshold,
            },
        ) {
            Ok(_) => Ok(()),
            Err(e) => Err(MetabodeconError::from(e).into()),
        }
    }

    pub fn set_analytical_fitter(&mut self, iterations: usize) -> PyResult<()> {
        match self
            .inner
            .set_fitting_settings(deconvolution::FittingSettings::Analytical { iterations })
        {
            Ok(_) => Ok(()),
            Err(e) => Err(MetabodeconError::from(e).into()),
        }
    }

    pub fn add_ignore_region(&mut self, new: (f64, f64)) -> PyResult<()> {
        match self.inner.add_ignore_region(new) {
            Ok(_) => Ok(()),
            Err(e) => Err(MetabodeconError::from(e).into()),
        }
    }

    pub fn clear_ignore_regions(&mut self) {
        self.inner.clear_ignore_regions();
    }

    pub fn deconvolute_spectrum(&self, spectrum: &Spectrum) -> PyResult<Deconvolution> {
        match self.inner.deconvolute_spectrum(spectrum.as_ref()) {
            Ok(deconvolution) => Ok(deconvolution.into()),
            Err(e) => Err(MetabodeconError::from(e).into()),
        }
    }

    pub fn par_deconvolute_spectrum(&self, spectrum: &Spectrum) -> PyResult<Deconvolution> {
        match self
            .inner
            .par_deconvolute_spectrum(spectrum.as_ref())
        {
            Ok(deconvolution) => Ok(deconvolution.into()),
            Err(e) => Err(MetabodeconError::from(e).into()),
        }
    }

    pub fn deconvolute_spectra(&self, spectra: Vec<Spectrum>) -> PyResult<Vec<Deconvolution>> {
        match self.inner.deconvolute_spectra(&spectra) {
            Ok(deconvolutions) => Ok(deconvolutions
                .into_iter()
                .map(|deconvolution| deconvolution.into())
                .collect()),
            Err(e) => Err(MetabodeconError::from(e).into()),
        }
    }

    pub fn par_deconvolute_spectra(&self, spectra: Vec<Spectrum>) -> PyResult<Vec<Deconvolution>> {
        match self.inner.par_deconvolute_spectra(&spectra) {
            Ok(deconvolutions) => Ok(deconvolutions
                .into_iter()
                .map(|deconvolution| deconvolution.into())
                .collect()),
            Err(e) => Err(MetabodeconError::from(e).into()),
        }
    }

    pub fn optimize_settings(&mut self, reference: &Spectrum) -> PyResult<f64> {
        match self.inner.optimize_settings(reference.as_ref()) {
            Ok(mse) => Ok(mse),
            Err(e) => Err(MetabodeconError::from(e).into()),
        }
    }
}
