use crate::deconvolution::Result;
use crate::peak_selection::peak::Peak;
use crate::peak_selection::scorer::ScoringAlgo;
use crate::spectrum::Spectrum;

#[non_exhaustive]
#[derive(Copy, Clone, Debug)]
pub enum SelectionAlgo {
    Default {
        scoring_algo: ScoringAlgo,
        threshold: f64,
    },
}

pub trait Selector {
    fn select_peaks(&self, spectrum: &Spectrum) -> Result<Vec<Peak>>;
}
