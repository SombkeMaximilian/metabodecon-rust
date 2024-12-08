use crate::spectrum::Spectrum;
use crate::peak_selection::peak::Peak;
use crate::peak_selection::scorer::ScoringAlgo;

#[derive(Debug, Clone, Copy)]
pub enum SelectionAlgo {
    Default {
        scoring_algo: ScoringAlgo,
        threshold: f64,
    },
}

pub trait Selector {
    fn select_peaks(&self, spectrum: &Spectrum) -> Vec<Peak>;
}
