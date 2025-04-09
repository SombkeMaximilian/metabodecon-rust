use crate::bindings::{Alignment, Deconvolution};
use crate::error::MetabodeconError;
use metabodecon::alignment;
use pyo3::prelude::*;

#[pyclass]
#[derive(Clone, Debug, Default)]
pub struct Aligner {
    inner: alignment::Aligner,
}

#[pymethods]
impl Aligner {
    #[new]
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn reference_alignment(&mut self, reference: usize) -> PyResult<()> {
        match self
            .inner
            .set_alignment_strategy(alignment::AlignmentStrategy::Reference(reference))
        {
            Ok(_) => Ok(()),
            Err(error) => Err(MetabodeconError::from(error).into()),
        }
    }

    pub(crate) fn pairwise_alignment(&mut self) {
        self.inner
            .set_alignment_strategy(alignment::AlignmentStrategy::Pairwise)
            .unwrap();
    }

    pub(crate) fn distance_similarity_filter(
        &mut self,
        max_distance: f64,
        min_similarity: f64,
    ) -> PyResult<()> {
        match self
            .inner
            .set_filtering_settings(alignment::FilteringSettings::DistanceSimilarity {
                similarity_metric: alignment::SimilarityMetric::ShapeDistance,
                max_distance,
                min_similarity,
            }) {
            Ok(_) => Ok(()),
            Err(error) => Err(MetabodeconError::from(error).into()),
        }
    }

    pub(crate) fn linear_programming_solver(&mut self) {
        self.inner
            .set_solving_settings(alignment::SolvingSettings::LinearProgramming)
            .unwrap();
    }

    pub(crate) fn align_deconvolutions(&self, deconvolutions: Vec<Deconvolution>) -> Alignment {
        self.inner
            .align_deconvolutions(&deconvolutions)
            .into()
    }
}
