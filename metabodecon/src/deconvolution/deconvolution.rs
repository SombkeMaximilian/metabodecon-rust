use crate::deconvolution::fitting::{FittingSettings, Lorentzian};
use crate::deconvolution::peak_selection::SelectionSettings;
use crate::deconvolution::smoothing::SmoothingSettings;
use std::sync::Arc;

/// Data structure representing the result of a deconvolution.
#[derive(Clone, Debug)]
pub struct Deconvolution {
    /// The deconvoluted signals.
    lorentzians: Arc<[Lorentzian]>,
    /// The smoothing parameters used.
    smoothing_settings: SmoothingSettings,
    /// The peak selection parameters used.
    selection_settings: SelectionSettings,
    /// The fitting parameters used.
    fitting_settings: FittingSettings,
    /// The mean squared error of the deconvolution.
    mse: f64,
}

impl Deconvolution {
    /// Constructs a new `Deconvolution`.
    pub fn new(
        lorentzians: Vec<Lorentzian>,
        smoothing_settings: SmoothingSettings,
        selection_settings: SelectionSettings,
        fitting_settings: FittingSettings,
        mse: f64,
    ) -> Self {
        Self {
            lorentzians: lorentzians.into(),
            smoothing_settings,
            selection_settings,
            fitting_settings,
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
    pub fn smoothing_settings(&self) -> SmoothingSettings {
        self.smoothing_settings
    }

    /// Returns the peak selection settings used.
    pub fn selection_settings(&self) -> SelectionSettings {
        self.selection_settings
    }

    /// Returns the fitting settings used.
    pub fn fitting_settings(&self) -> FittingSettings {
        self.fitting_settings
    }

    /// Returns the mean squared error of the deconvolution.
    pub fn mse(&self) -> f64 {
        self.mse
    }
}
