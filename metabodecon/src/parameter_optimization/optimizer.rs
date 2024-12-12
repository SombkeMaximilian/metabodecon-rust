use crate::deconvolution::{Deconvoluter, Deconvolution};
use crate::fitting::FittingAlgo;
use crate::peak_selection::SelectionAlgo;
use crate::smoothing::SmoothingAlgo;
use crate::spectrum::Spectrum;

#[derive(Copy, Clone, Debug)]
pub struct ParameterOptimizer {
    smoothing_algo: SmoothingAlgo,
    selection_algo: SelectionAlgo,
    fitting_algo: FittingAlgo,
}
