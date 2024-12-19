use crate::bindings::{Deconvolution, Spectrum};
use pyo3::prelude::*;

#[pyclass]
#[derive(Copy, Clone, Debug)]
pub struct Deconvoluter {
    inner: metabodecon::Deconvoluter,
}

#[pymethods]
impl Deconvoluter {
    #[new]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            inner: metabodecon::Deconvoluter::new(
                metabodecon::SmoothingAlgo::MovingAverage {
                    algo: metabodecon::MovingAverageAlgo::SumCache,
                    iterations: 0,
                    window_size: 0,
                },
                metabodecon::SelectionAlgo::Default {
                    scoring_algo: metabodecon::ScoringAlgo::MinimumSum,
                    threshold: 0.0,
                },
                metabodecon::FittingAlgo::Analytical { iterations: 0 },
            ),
        }
    }

    pub fn with_ma_smoother(&mut self, iterations: usize, window_size: usize) -> Self {
        self.inner
            .set_smoothing_algo(metabodecon::SmoothingAlgo::MovingAverage {
                algo: metabodecon::MovingAverageAlgo::SumCache,
                iterations,
                window_size,
            });
        *self
    }

    pub fn with_def_selector(&mut self, threshold: f64) -> Self {
        self.inner
            .set_selection_algo(metabodecon::SelectionAlgo::Default {
                scoring_algo: metabodecon::ScoringAlgo::MinimumSum,
                threshold,
            });
        *self
    }

    pub fn with_analytical_fitter(&mut self, iterations: usize) -> Self {
        self.inner
            .set_fitting_algo(metabodecon::FittingAlgo::Analytical { iterations });
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
