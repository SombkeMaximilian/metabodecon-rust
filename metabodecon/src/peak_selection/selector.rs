use crate::peak_selection::peak::Peak;
use crate::peak_selection::scorer::ScoringAlgo;
use crate::spectrum::Spectrum;

#[derive(Copy, Clone, Debug)]
pub enum SelectionAlgo {
    Default {
        scoring_algo: ScoringAlgo,
        threshold: f64,
    },
}

pub trait Selector {
    fn select_peaks(&self, spectrum: &Spectrum) -> Vec<Peak>;
}
