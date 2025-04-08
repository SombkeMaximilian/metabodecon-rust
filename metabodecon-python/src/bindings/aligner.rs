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
    pub(crate) fn new(max_distance: f64, min_similarity: f64) -> Self {
        Self {
            inner: alignment::Aligner::new(
                alignment::FilteringSettings::DistanceSimilarity {
                    similarity_metric: alignment::SimilarityMetric::Shape,
                    max_distance,
                    min_similarity,
                },
                alignment::SolvingSettings::LinearProgramming,
            ),
        }
    }

    pub(crate) fn align_deconvolutions(&self, deconvolutions: Vec<Deconvolution>) -> Alignment {
        self.inner
            .align_deconvolutions(&deconvolutions)
            .into()
    }
}
