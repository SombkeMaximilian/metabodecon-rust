use crate::deconvolution::Settings;
use crate::deconvolution::error::{Error, Kind};
use crate::deconvolution::lorentzian::Lorentzian;
use crate::deconvolution::peak_selection::Peak;
use crate::spectrum::Spectrum;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Trait interface for fitting algorithms.
pub(crate) trait Fitter: Send + Sync + std::fmt::Debug {
    /// Fits Lorentzian functions to a spectrum using the given peaks.
    fn fit_lorentzian(&self, spectrum: &Spectrum, peaks: &[Peak]) -> Vec<Lorentzian>;

    /// Fits Lorentzian functions to a spectrum using the given peaks in
    /// parallel.
    #[cfg(feature = "parallel")]
    fn par_fit_lorentzian(&self, spectrum: &Spectrum, peaks: &[Peak]) -> Vec<Lorentzian>;

    /// Returns the settings of the trait object.
    fn settings(&self) -> FittingSettings;
}

/// Fitting methods.
#[non_exhaustive]
#[derive(Copy, Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(tag = "method", rename_all_fields = "camelCase")
)]
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

impl std::fmt::Display for FittingSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FittingSettings::Analytical { iterations } => {
                write!(
                    f,
                    "Analytical Fitter [number of iterations: {}]",
                    iterations
                )
            }
        }
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

    #[cfg(test)]
    fn compare(&self, other: &Self) -> bool {
        match (self, other) {
            (
                FittingSettings::Analytical {
                    iterations: iterations1,
                },
                FittingSettings::Analytical {
                    iterations: iterations2,
                },
            ) => *iterations1 == *iterations2,
        }
    }
}
