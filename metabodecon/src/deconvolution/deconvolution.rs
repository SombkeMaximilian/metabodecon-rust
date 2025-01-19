use crate::fitting::{FittingAlgo, Lorentzian};
use crate::peak_selection::SelectionAlgo;
use crate::smoothing::SmoothingAlgo;

/// Data structure representing the result of a deconvolution.
#[derive(Clone, Debug)]
pub struct Deconvolution {
    /// The deconvoluted signals.
    lorentzians: Vec<Lorentzian>,
    /// The smoothing parameters used.
    smoothing_algo: SmoothingAlgo,
    /// The peak selection parameters used.
    selection_algo: SelectionAlgo,
    /// The fitting parameters used.
    fitting_algo: FittingAlgo,
    /// The mean squared error of the deconvolution.
    mse: f64,
}

impl Deconvolution {
    /// Constructs a new `Deconvolution`.
    pub fn new(
        lorentzians: Vec<Lorentzian>,
        smoothing_algo: SmoothingAlgo,
        selection_algo: SelectionAlgo,
        fitting_algo: FittingAlgo,
        mse: f64,
    ) -> Self {
        Self {
            lorentzians,
            smoothing_algo,
            selection_algo,
            fitting_algo,
            mse,
        }
    }

    /// Returns the deconvoluted signals as a slice of [`Lorentzian`].
    ///
    /// [`Lorentzian`]: Lorentzian
    pub fn lorentzians(&self) -> &[Lorentzian] {
        &self.lorentzians
    }

    /// Returns the smoothing settings used.
    pub fn smoothing_algo(&self) -> &SmoothingAlgo {
        &self.smoothing_algo
    }

    /// Returns the peak selection settings used.
    pub fn selection_algo(&self) -> &SelectionAlgo {
        &self.selection_algo
    }

    /// Returns the fitting settings used.
    pub fn fitting_algo(&self) -> &FittingAlgo {
        &self.fitting_algo
    }

    /// Returns the mean squared error of the deconvolution.
    pub fn mse(&self) -> f64 {
        self.mse
    }
}
