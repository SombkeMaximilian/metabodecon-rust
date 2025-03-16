use crate::bindings::Deconvolution;
use crate::bindings::alignment::Alignment;
use metabodecon::alignment;
use pyo3::prelude::*;

#[pyclass]
pub struct Aligner {
    inner: alignment::Aligner,
}

#[pymethods]
impl Aligner {
    #[new]
    pub fn new(max_distance: f64, min_similarity: f64) -> Self {
        Self {
            inner: alignment::Aligner::new(max_distance, min_similarity),
        }
    }

    pub fn align_deconvolutions(&self, deconvolutions: Vec<Deconvolution>) -> Alignment {
        self.inner
            .align_deconvolutions(&deconvolutions)
            .into()
    }
}
