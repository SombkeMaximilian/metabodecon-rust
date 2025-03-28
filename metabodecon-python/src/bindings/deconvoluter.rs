use crate::bindings::{Deconvolution, Spectrum};
use crate::error::MetabodeconError;
use metabodecon::deconvolution;
use pyo3::prelude::*;

#[pyclass]
#[derive(Clone, Debug, Default)]
pub(crate) struct Deconvoluter {
    inner: deconvolution::Deconvoluter,
}

#[pymethods]
impl Deconvoluter {
    #[new]
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn set_moving_average_smoother(
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
            Err(error) => Err(MetabodeconError::from(error).into()),
        }
    }

    pub(crate) fn set_noise_score_selector(&mut self, threshold: f64) -> PyResult<()> {
        match self.inner.set_selection_settings(
            deconvolution::SelectionSettings::NoiseScoreFilter {
                scoring_method: deconvolution::ScoringMethod::MinimumSum,
                threshold,
            },
        ) {
            Ok(_) => Ok(()),
            Err(error) => Err(MetabodeconError::from(error).into()),
        }
    }

    pub(crate) fn set_analytical_fitter(&mut self, iterations: usize) -> PyResult<()> {
        match self
            .inner
            .set_fitting_settings(deconvolution::FittingSettings::Analytical { iterations })
        {
            Ok(_) => Ok(()),
            Err(error) => Err(MetabodeconError::from(error).into()),
        }
    }

    pub(crate) fn add_ignore_region(&mut self, new: (f64, f64)) -> PyResult<()> {
        match self.inner.add_ignore_region(new) {
            Ok(_) => Ok(()),
            Err(error) => Err(MetabodeconError::from(error).into()),
        }
    }

    pub(crate) fn clear_ignore_regions(&mut self) {
        self.inner.clear_ignore_regions();
    }

    pub(crate) fn deconvolute_spectrum(&self, spectrum: &Spectrum) -> PyResult<Deconvolution> {
        match self.inner.deconvolute_spectrum(spectrum.as_ref()) {
            Ok(deconvolution) => Ok(deconvolution.into()),
            Err(error) => Err(MetabodeconError::from(error).into()),
        }
    }

    pub(crate) fn par_deconvolute_spectrum(&self, spectrum: &Spectrum) -> PyResult<Deconvolution> {
        match self
            .inner
            .par_deconvolute_spectrum(spectrum.as_ref())
        {
            Ok(deconvolution) => Ok(deconvolution.into()),
            Err(error) => Err(MetabodeconError::from(error).into()),
        }
    }

    pub(crate) fn deconvolute_spectra(
        &self,
        spectra: Vec<Spectrum>,
    ) -> PyResult<Vec<Deconvolution>> {
        match self.inner.deconvolute_spectra(&spectra) {
            Ok(deconvolutions) => Ok(deconvolutions
                .into_iter()
                .map(|deconvolution| deconvolution.into())
                .collect()),
            Err(error) => Err(MetabodeconError::from(error).into()),
        }
    }

    pub(crate) fn par_deconvolute_spectra(
        &self,
        spectra: Vec<Spectrum>,
    ) -> PyResult<Vec<Deconvolution>> {
        match self.inner.par_deconvolute_spectra(&spectra) {
            Ok(deconvolutions) => Ok(deconvolutions
                .into_iter()
                .map(|deconvolution| deconvolution.into())
                .collect()),
            Err(error) => Err(MetabodeconError::from(error).into()),
        }
    }

    pub(crate) fn optimize_settings(&mut self, reference: &Spectrum) -> PyResult<f64> {
        match self.inner.optimize_settings(reference.as_ref()) {
            Ok(mse) => Ok(mse),
            Err(error) => Err(MetabodeconError::from(error).into()),
        }
    }
}
