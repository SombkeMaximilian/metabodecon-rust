use crate::bindings::{Deconvolution, Spectrum};
use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone, Copy)]
pub struct Deconvoluter {
    inner: metabodecon::Deconvoluter,
}

#[pymethods]
impl Deconvoluter {
    #[new]
    pub fn new(nfit: usize, sm_iter: usize, sm_ws: usize, delta: f64) -> Self {
        Deconvoluter {
            inner: metabodecon::Deconvoluter::new(
                metabodecon::SmoothingAlgo::MovingAverage {
                    algo: metabodecon::MovingAverageAlgo::SumCache,
                    iterations: sm_iter,
                    window_size: sm_ws,
                },
                metabodecon::SelectionAlgo::Default {
                    scoring_algo: metabodecon::ScoringAlgo::MinimumSum,
                    threshold: delta
                },
                metabodecon::FittingAlgo::Analytical {
                    iterations: nfit
                }
            )
        }
    }

    pub fn deconvolute_spectrum(&self, spectrum: &mut Spectrum) -> Deconvolution {
        Deconvolution::from_inner(self.inner.deconvolute_spectrum(spectrum.inner_mut()))
    }

    pub fn par_deconvolute_spectrum(&self, spectrum: &mut Spectrum) -> Deconvolution {
        Deconvolution::from_inner(self.inner.par_deconvolute_spectrum(spectrum.inner_mut()))
    }
}
