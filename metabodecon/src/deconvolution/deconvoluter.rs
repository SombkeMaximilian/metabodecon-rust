use crate::deconvolution::{Deconvolution, Settings};
use crate::error::Result;
use crate::fitting::{Fitter, FitterAnalytical, FittingAlgo, Lorentzian};
use crate::peak_selection::{NoiseScoreFilter, SelectionAlgo, Selector};
use crate::smoothing::{MovingAverage, Smoother, SmoothingAlgo};
use crate::spectrum::Spectrum;

/// Deconvolution pipeline that applies smoothing, peak selection, and fitting
/// to a spectrum to deconvolute it into individual signals.
///
/// The output of the pipeline is a [`Deconvolution`] struct containing the
/// deconvoluted signals, the deconvolution settings, and the [MSE] between the
/// superposition of signals and the raw intensities within the signal region.
///
/// [MSE]: https://en.wikipedia.org/wiki/Mean_squared_error
///
/// # Example: Deconvoluting a [`Spectrum`]
///
/// ```
/// use metabodecon::deconvolution::{Deconvoluter, Deconvolution, Lorentzian};
/// use metabodecon::spectrum::BrukerReader;
///
/// # fn main() -> metabodecon::Result<()> {
/// // Read a spectrum in Bruker TopSpin format.
/// let reader = BrukerReader::new();
/// let path = "path/to/spectrum";
/// # let path = "../data/bruker/blood/blood_01";
/// let mut spectrum = reader.read_spectrum(
///     path,
///     // Experiment number
///     10,
///     // Processing number
///     10,
///     // Signal boundaries
///     (-2.2, 11.8),
/// )?;
///
/// // Deconvolute the spectrum.
/// let deconvoluter = Deconvoluter::default();
/// let deconvolution = deconvoluter.deconvolute_spectrum(&mut spectrum)?;
/// # Ok(())
/// # }
/// ```
///
/// # Example: Parallelized Deconvolution
///
/// The most expensive parts of the deconvolution process can also be performed
/// in parallel by enabling the `parallel` feature (part of the `default`
/// features). This adds [rayon] as a dependency.
///
/// [rayon]: https://docs.rs/rayon/latest/rayon/
///
/// ```
/// use metabodecon::deconvolution::Deconvoluter;
/// use metabodecon::spectrum::BrukerReader;
///
/// # fn main() -> metabodecon::Result<()> {
/// // Read a spectrum in Bruker TopSpin format.
/// let reader = BrukerReader::new();
/// let path = "path/to/spectrum";
/// # let path = "../data/bruker/blood/blood_01";
/// let mut spectrum = reader.read_spectrum(
///     path,
///     // Experiment number
///     10,
///     // Processing number
///     10,
///     // Signal boundaries
///     (-2.2, 11.8),
/// )?;
///
/// // Deconvolute the spectrum in parallel.
/// let deconvoluter = Deconvoluter::default();
/// let deconvolution = deconvoluter.par_deconvolute_spectrum(&mut spectrum)?;
/// # Ok(())
/// # }
/// ```
///
/// # Example: Configuring the Deconvoluter
///
/// `Deconvoluter` is modular and allows you to configure the smoothing, peak
/// selection, and fitting algorithms independently. Currently, there is only
/// one method available for each part of the pipeline, but more may be added in
/// the future. It may also be possible to use your own implementations of the
/// algorithms by implementing the corresponding traits in the future.
///
/// ```
/// use metabodecon::deconvolution::{
///     Deconvoluter, FittingAlgo, ScoringAlgo, SelectionAlgo, SmoothingAlgo,
/// };
///
/// # fn main() -> metabodecon::Result<()> {
/// let mut deconvoluter = Deconvoluter::default();
///
/// // Change the smoothing algorithm.
/// deconvoluter.set_smoothing_algo(SmoothingAlgo::MovingAverage {
///     iterations: 3,
///     window_size: 5,
/// })?;
///
/// // Change the peak selection algorithm.
/// deconvoluter.set_selection_algo(SelectionAlgo::NoiseScoreFilter {
///     scoring_algo: ScoringAlgo::MinimumSum,
///     threshold: 5.0,
/// })?;
///
/// // Change the fitting algorithm.
/// deconvoluter
///     .set_fitting_algo(FittingAlgo::Analytical { iterations: 20 })?;
///
/// // Configure everything at once.
/// let deconvoluter = Deconvoluter::new(
///     SmoothingAlgo::MovingAverage {
///         iterations: 3,
///         window_size: 3,
///     },
///     SelectionAlgo::NoiseScoreFilter {
///         scoring_algo: ScoringAlgo::MinimumSum,
///         threshold: 5.0,
///     },
///     FittingAlgo::Analytical { iterations: 20 },
/// )?;
/// # Ok(())
/// # }
/// ```
#[derive(Copy, Clone, Debug, Default)]
pub struct Deconvoluter {
    /// The smoothing algorithm to use.
    smoothing_algo: SmoothingAlgo,
    /// The peak selection algorithm to use.
    selection_algo: SelectionAlgo,
    /// The fitting algorithm to use.
    fitting_algo: FittingAlgo,
}

impl Deconvoluter {
    /// Constructs a new `Deconvoluter` with the provided settings.
    ///
    /// # Errors
    ///
    /// The Deconvolution settings are checked for validity. The following
    /// errors are possible if the respective checks fail:
    /// - [`InvalidSmoothingSettings`]: The provided smoothing settings are
    ///   invalid. For example, a `window_size` of 0 for a moving average filter
    ///   would mean that no smoothing is applied.
    /// - [`InvalidSelectionSettings`]: The provided peak selection settings are
    ///   invalid. For example, a negative `threshold` for a noise score filter
    ///   wouldn't make sense.
    /// - [`InvalidFittingSettings`]: The provided fitting settings are invalid.
    ///   For example, 0 `iterations` for an analytical fitting algorithm would
    ///   mean that the fitting algorithm doesn't do anything.
    ///
    /// [`InvalidSmoothingSettings`]: crate::deconvolution::error::Kind::InvalidSmoothingSettings
    /// [`InvalidSelectionSettings`]: crate::deconvolution::error::Kind::InvalidSelectionSettings
    /// [`InvalidFittingSettings`]: crate::deconvolution::error::Kind::InvalidFittingSettings
    ///
    /// # Example
    ///
    /// ```
    /// use metabodecon::deconvolution::{
    ///     Deconvoluter, FittingAlgo, ScoringAlgo, SelectionAlgo, SmoothingAlgo,
    /// };
    ///
    /// let deconvoluter = Deconvoluter::new(
    ///     SmoothingAlgo::MovingAverage {
    ///         iterations: 3,
    ///         window_size: 3,
    ///     },
    ///     SelectionAlgo::NoiseScoreFilter {
    ///         scoring_algo: ScoringAlgo::MinimumSum,
    ///         threshold: 5.0,
    ///     },
    ///     FittingAlgo::Analytical { iterations: 20 },
    /// );
    /// ```
    pub fn new(
        smoothing_algo: SmoothingAlgo,
        selection_algo: SelectionAlgo,
        fitting_algo: FittingAlgo,
    ) -> Result<Self> {
        smoothing_algo.validate()?;
        selection_algo.validate()?;
        fitting_algo.validate()?;

        Ok(Self {
            smoothing_algo,
            selection_algo,
            fitting_algo,
        })
    }

    /// Returns the smoothing settings.
    pub fn smoothing_algo(&self) -> SmoothingAlgo {
        self.smoothing_algo
    }

    /// Returns the peak selection settings.
    pub fn selection_algo(&self) -> SelectionAlgo {
        self.selection_algo
    }

    /// Returns the fitting settings.
    pub fn fitting_algo(&self) -> FittingAlgo {
        self.fitting_algo
    }

    /// Sets the smoothing settings.
    pub fn set_smoothing_algo(&mut self, smoothing_algo: SmoothingAlgo) -> Result<()> {
        smoothing_algo.validate()?;
        self.smoothing_algo = smoothing_algo;

        Ok(())
    }

    /// Sets the peak selection settings.
    pub fn set_selection_algo(&mut self, selection_algo: SelectionAlgo) -> Result<()> {
        selection_algo.validate()?;
        self.selection_algo = selection_algo;

        Ok(())
    }

    /// Sets the fitting settings.
    pub fn set_fitting_algo(&mut self, fitting_algo: FittingAlgo) -> Result<()> {
        fitting_algo.validate()?;
        self.fitting_algo = fitting_algo;

        Ok(())
    }

    /// Deconvolutes the provided spectrum into individual signals.
    pub fn deconvolute_spectrum(&self, spectrum: &mut Spectrum) -> Result<Deconvolution> {
        spectrum.set_intensities(spectrum.intensities_raw().to_vec())?;
        let mut smoother = match self.smoothing_algo {
            SmoothingAlgo::MovingAverage {
                iterations,
                window_size,
            } => MovingAverage::new(iterations, window_size),
        };
        smoother.smooth_values(spectrum.intensities_mut());
        let peaks = {
            let selector = match self.selection_algo {
                SelectionAlgo::NoiseScoreFilter {
                    scoring_algo,
                    threshold,
                } => NoiseScoreFilter::new(scoring_algo, threshold),
            };
            selector.select_peaks(spectrum)?
        };
        let mut lorentzians = {
            let fitter = match self.fitting_algo {
                FittingAlgo::Analytical { iterations } => FitterAnalytical::new(iterations),
            };
            fitter.fit_lorentzian(spectrum, &peaks)
        };
        lorentzians.retain(|lorentzian| lorentzian.sfhw() > 0.0 && lorentzian.hw2() > 0.0);
        let mse = Self::compute_mse(
            spectrum,
            Lorentzian::superposition_vec(spectrum.chemical_shifts(), &lorentzians),
        );

        Ok(Deconvolution::new(
            lorentzians,
            self.smoothing_algo,
            self.selection_algo,
            self.fitting_algo,
            mse,
        ))
    }

    /// Deconvolutes the provided spectrum into individual signals in parallel.
    #[cfg(feature = "parallel")]
    pub fn par_deconvolute_spectrum(&self, spectrum: &mut Spectrum) -> Result<Deconvolution> {
        spectrum.set_intensities(spectrum.intensities_raw().to_vec())?;
        let mut smoother = match self.smoothing_algo {
            SmoothingAlgo::MovingAverage {
                iterations,
                window_size,
            } => MovingAverage::new(iterations, window_size),
        };
        smoother.smooth_values(spectrum.intensities_mut());
        let peaks = {
            let selector = match self.selection_algo {
                SelectionAlgo::NoiseScoreFilter {
                    threshold,
                    scoring_algo,
                } => NoiseScoreFilter::new(scoring_algo, threshold),
            };
            selector.select_peaks(spectrum)?
        };
        let mut lorentzians = {
            let fitter = match self.fitting_algo {
                FittingAlgo::Analytical { iterations } => FitterAnalytical::new(iterations),
            };
            fitter.par_fit_lorentzian(spectrum, &peaks)
        };
        lorentzians.retain(|lorentzian| lorentzian.sfhw() > 0.0 && lorentzian.hw2() > 0.0);
        let mse = Self::compute_mse(
            spectrum,
            Lorentzian::par_superposition_vec(spectrum.chemical_shifts(), &lorentzians),
        );

        Ok(Deconvolution::new(
            lorentzians,
            self.smoothing_algo,
            self.selection_algo,
            self.fitting_algo,
            mse,
        ))
    }

    /// Internal helper function to compute the MSE within the signal region.
    fn compute_mse(spectrum: &Spectrum, superpositions: Vec<f64>) -> f64 {
        let signal_boundaries = spectrum.signal_boundaries_indices();
        let residuals = superpositions[signal_boundaries.0..signal_boundaries.1]
            .iter()
            .zip(spectrum.intensities_raw()[signal_boundaries.0..signal_boundaries.1].iter())
            .map(|(superposition, raw)| (superposition - raw).powi(2))
            .sum::<f64>();
        let length = signal_boundaries.1 - signal_boundaries.0;

        residuals / (length as f64)
    }
}
