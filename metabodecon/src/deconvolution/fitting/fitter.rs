use crate::deconvolution::Settings;
use crate::deconvolution::error::{Error, Kind};
use crate::deconvolution::fitting::lorentzian::Lorentzian;
use crate::deconvolution::peak_selection::Peak;
use crate::spectrum::Spectrum;

/// Trait interface for fitting algorithms.
pub(crate) trait Fitter {
    /// Fits Lorentzian functions to a spectrum using the given peaks.
    fn fit_lorentzian(&self, spectrum: &Spectrum, peaks: &[Peak]) -> Vec<Lorentzian>;

    /// Fits Lorentzian functions to a spectrum using the given peaks in
    /// parallel.
    #[cfg(feature = "parallel")]
    fn par_fit_lorentzian(&self, spectrum: &Spectrum, peaks: &[Peak]) -> Vec<Lorentzian>;
}

/// Fitting methods.
#[non_exhaustive]
#[derive(Copy, Clone, Debug)]
pub enum FittingSettings {
    /// Fitting by solving a system of linear equations analytically.
    Analytical {
        /// The number of iterations to refine the fit.
        iterations: usize,
    },
}

impl Default for FittingSettings {
    fn default() -> Self {
        FittingSettings::Analytical { iterations: 10 }
    }
}

impl Settings for FittingSettings {
    fn validate(&self) -> crate::Result<()> {
        match self {
            FittingSettings::Analytical { iterations } => {
                if *iterations == 0 {
                    return Err(Error::new(Kind::InvalidFittingSettings { settings: *self }).into());
                }
            }
        }

        Ok(())
    }
}
