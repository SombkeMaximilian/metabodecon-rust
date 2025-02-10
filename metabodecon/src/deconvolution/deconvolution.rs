use crate::deconvolution::fitting::FittingSettings;
use crate::deconvolution::lorentzian::Lorentzian;
use crate::deconvolution::peak_selection::SelectionSettings;
use crate::deconvolution::smoothing::SmoothingSettings;
use std::sync::Arc;

#[cfg(feature = "serde")]
use crate::deconvolution::SerializedDeconvolution;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Data structure representing the result of a deconvolution.
#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(into = "SerializedDeconvolution", try_from = "SerializedDeconvolution")
)]
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

impl AsRef<Deconvolution> for Deconvolution {
    fn as_ref(&self) -> &Deconvolution {
        self
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deconvolution::ScoringMethod;
    use crate::{assert_send, assert_sync};
    use float_cmp::assert_approx_eq;

    #[test]
    fn thread_safety() {
        assert_send!(Deconvolution);
        assert_sync!(Deconvolution);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serialization_round_trip() {
        let lorentzians = vec![
            Lorentzian::new(5.5, 0.25, 3.0),
            Lorentzian::new(7.0, 0.16, 5.0),
            Lorentzian::new(5.5, 0.25, 7.0),
        ];
        let deconvolution = Deconvolution::new(
            lorentzians.clone(),
            SmoothingSettings::default(),
            SelectionSettings::default(),
            FittingSettings::default(),
            0.5,
        );
        let serialized = serde_json::to_string(&deconvolution).unwrap();
        let deserialized = serde_json::from_str::<Deconvolution>(&serialized).unwrap();
        deconvolution
            .lorentzians
            .iter()
            .zip(deserialized.lorentzians())
            .for_each(|(init, rec)| {
                assert_approx_eq!(f64, init.sfhw(), rec.sfhw());
                assert_approx_eq!(f64, init.hw2(), rec.hw2());
                assert_approx_eq!(f64, init.maxp(), rec.maxp());
            });
        match deserialized.smoothing_settings() {
            SmoothingSettings::MovingAverage {
                iterations,
                window_size,
            } => {
                assert_eq!(iterations, 2);
                assert_eq!(window_size, 5);
            }
        };
        match deserialized.selection_settings() {
            SelectionSettings::NoiseScoreFilter {
                scoring_method,
                threshold,
            } => {
                match scoring_method {
                    ScoringMethod::MinimumSum => {}
                }
                assert_approx_eq!(f64, threshold, 6.4);
            }
        };
        match deserialized.fitting_settings() {
            FittingSettings::Analytical { iterations } => {
                assert_eq!(iterations, 10);
            }
        };
    }
}
