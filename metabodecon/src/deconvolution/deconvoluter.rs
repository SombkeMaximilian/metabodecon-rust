use crate::deconvolution::error::{Error, Kind};
use crate::deconvolution::fitting::{Fitter, FitterAnalytical, FittingAlgo, Lorentzian};
use crate::deconvolution::peak_selection::{NoiseScoreFilter, Peak, SelectionAlgo, Selector};
use crate::deconvolution::smoothing::{MovingAverage, Smoother, SmoothingAlgo};
use crate::deconvolution::{Deconvolution, Settings};
use crate::error::Result;
use crate::spectrum::Spectrum;

#[cfg(feature = "parallel")]
use rayon::prelude::*;

/// Deconvolution pipeline that applies smoothing, peak selection, and fitting
/// to a spectrum to deconvolute it into individual signals.
///
/// The output of the pipeline is a [`Deconvolution`] struct containing the
/// deconvoluted signals, the deconvolution settings, and the [MSE] between the
/// superposition of signals and the intensities within the signal region.
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
/// let spectrum = reader.read_spectrum(
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
/// let deconvolution = deconvoluter.deconvolute_spectrum(&spectrum)?;
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
/// let spectrum = reader.read_spectrum(
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
/// let deconvolution = deconvoluter.par_deconvolute_spectrum(&spectrum)?;
/// # Ok(())
/// # }
/// ```
///
/// # Example: Configuring the Deconvoluter
///
/// `Deconvoluter` is modular and allows you to configure the smoothing, peak
/// selection, and fitting algorithms independently. Additionally, you can
/// specify ppm regions to be ignored during the deconvolution. This may be
/// useful for compounds like stabilizing agents or a water signal.
///
/// ```
/// use metabodecon::deconvolution::{
///     Deconvoluter, FittingAlgo, ScoringAlgo, SelectionAlgo, SmoothingAlgo,
/// };
///
/// # fn main() -> metabodecon::Result<()> {
/// // Create a new Deconvoluter with the desired settings.
/// let mut deconvoluter = Deconvoluter::new(
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
///
/// // Add a region to ignore during deconvolution.
/// deconvoluter.add_ignore_region((4.7, 4.9))?;
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Debug, Default)]
pub struct Deconvoluter {
    /// The smoothing algorithm to use.
    smoothing_algo: SmoothingAlgo,
    /// The peak selection algorithm to use.
    selection_algo: SelectionAlgo,
    /// The fitting algorithm to use.
    fitting_algo: FittingAlgo,
    /// The regions to ignore during deconvolution.
    ignore_regions: Option<Vec<(f64, f64)>>,
}

impl Deconvoluter {
    /// Constructs a new `Deconvoluter` with the provided settings.
    ///
    /// # Errors
    ///
    /// An error is returned if any of the deconvolution settings are invalid.
    /// For Example:
    /// - A `window_size` of 0 for a moving average filter would mean that no
    ///   smoothing is applied.
    /// - Negative `threshold`s for a noise score filter wouldn't make sense.
    /// - 0 `iterations` for an analytical fitting algorithm would mean that the
    ///   fitting algorithm doesn't do anything.
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
            ignore_regions: None,
        })
    }

    /// Returns the smoothing settings.
    ///
    /// # Example
    ///
    /// ```
    /// use metabodecon::deconvolution::{Deconvoluter, SmoothingAlgo};
    ///
    /// let deconvoluter = Deconvoluter::default();
    ///
    /// match deconvoluter.smoothing_algo() {
    ///     SmoothingAlgo::MovingAverage {
    ///         iterations,
    ///         window_size,
    ///     } => {
    ///         assert_eq!(iterations, 2);
    ///         assert_eq!(window_size, 5);
    ///     }
    ///     _ => panic!("Unexpected smoothing algorithm"),
    /// };
    /// ```
    pub fn smoothing_algo(&self) -> SmoothingAlgo {
        self.smoothing_algo
    }

    /// Returns the peak selection settings.
    ///
    /// # Example
    ///
    /// ```
    /// use metabodecon::deconvolution::{
    ///     Deconvoluter, ScoringAlgo, SelectionAlgo,
    /// };
    ///
    /// let deconvoluter = Deconvoluter::default();
    ///
    /// match deconvoluter.selection_algo() {
    ///     SelectionAlgo::NoiseScoreFilter {
    ///         scoring_algo,
    ///         threshold,
    ///     } => {
    ///         match scoring_algo {
    ///             ScoringAlgo::MinimumSum => {}
    ///             _ => panic!("Unexpected scoring algorithm"),
    ///         };
    ///         assert_eq!(threshold, 6.4);
    ///     }
    ///     _ => panic!("Unexpected selection algorithm"),
    /// };
    /// ```
    pub fn selection_algo(&self) -> SelectionAlgo {
        self.selection_algo
    }

    /// Returns the fitting settings.
    ///
    /// # Example
    ///
    /// ```
    /// use metabodecon::deconvolution::{Deconvoluter, FittingAlgo};
    ///
    /// let deconvoluter = Deconvoluter::default();
    ///
    /// match deconvoluter.fitting_algo() {
    ///     FittingAlgo::Analytical { iterations } => {
    ///         assert_eq!(iterations, 10);
    ///     }
    ///     _ => panic!("Unexpected fitting algorithm"),
    /// };
    /// ```
    pub fn fitting_algo(&self) -> FittingAlgo {
        self.fitting_algo
    }

    /// Returns the regions to ignore during deconvolution.
    ///
    /// # Example
    ///
    /// ```
    /// use float_cmp::assert_approx_eq;
    /// use metabodecon::deconvolution::Deconvoluter;
    ///
    /// # fn main() -> metabodecon::Result<()> {
    /// let mut deconvoluter = Deconvoluter::default();
    ///
    /// assert!(deconvoluter.ignore_regions().is_none());
    /// # Ok(())
    /// # }
    /// ```
    pub fn ignore_regions(&self) -> Option<&Vec<(f64, f64)>> {
        self.ignore_regions.as_ref()
    }

    /// Sets the smoothing settings.
    ///
    /// # Errors
    ///
    /// An error is returned if the provided smoothing settings are invalid. For
    /// example, a `window_size` of 0 for a moving average filter would mean
    /// that no smoothing is applied.
    ///
    /// # Example
    ///
    /// ```
    /// use metabodecon::deconvolution::{Deconvoluter, SmoothingAlgo};
    ///
    /// # fn main() -> metabodecon::Result<()> {
    /// let mut deconvoluter = Deconvoluter::default();
    ///
    /// deconvoluter.set_smoothing_algo(SmoothingAlgo::MovingAverage {
    ///     iterations: 3,
    ///     window_size: 3,
    /// })?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_smoothing_algo(&mut self, smoothing_algo: SmoothingAlgo) -> Result<()> {
        smoothing_algo.validate()?;
        self.smoothing_algo = smoothing_algo;

        Ok(())
    }

    /// Sets the peak selection settings.
    ///
    /// # Errors
    ///
    /// An error is returned if the provided peak selection settings are
    /// invalid. For example, a negative `threshold` for a noise score filter
    /// wouldn't make sense.
    ///
    /// [`InvalidSelectionSettings`]: Kind::InvalidSelectionSettings
    ///
    /// # Example
    ///
    /// ```
    /// use metabodecon::deconvolution::{
    ///     Deconvoluter, ScoringAlgo, SelectionAlgo,
    /// };
    ///
    /// # fn main() -> metabodecon::Result<()> {
    /// let mut deconvoluter = Deconvoluter::default();
    ///
    /// deconvoluter.set_selection_algo(SelectionAlgo::NoiseScoreFilter {
    ///     scoring_algo: ScoringAlgo::MinimumSum,
    ///     threshold: 5.0,
    /// })?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_selection_algo(&mut self, selection_algo: SelectionAlgo) -> Result<()> {
        selection_algo.validate()?;
        self.selection_algo = selection_algo;

        Ok(())
    }

    /// Sets the fitting settings.
    ///
    /// # Errors
    ///
    /// An error is returned if the provided fitting settings are invalid. For
    /// example, 0 `iterations` would mean that the fitting algorithm doesn't
    /// do anything.
    ///
    /// [`InvalidFittingSettings`]: Kind::InvalidFittingSettings
    ///
    /// # Example
    ///
    /// ```
    /// use metabodecon::deconvolution::{Deconvoluter, FittingAlgo};
    ///
    /// # fn main() -> metabodecon::Result<()> {
    /// let mut deconvoluter = Deconvoluter::default();
    ///
    /// deconvoluter
    ///     .set_fitting_algo(FittingAlgo::Analytical { iterations: 20 })?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_fitting_algo(&mut self, fitting_algo: FittingAlgo) -> Result<()> {
        fitting_algo.validate()?;
        self.fitting_algo = fitting_algo;

        Ok(())
    }

    /// Adds a region to ignore during deconvolution.
    ///
    /// Some samples contain compounds that are not of interest, such as a water
    /// signal. Regions where these compounds are expected can be ignored during
    /// the deconvolution.
    ///
    /// # Errors
    ///
    /// An error is returned if the start or end value is not finite or if they
    /// are (nearly) equal.
    ///
    /// # Example
    ///
    /// ```
    /// use metabodecon::deconvolution::{Deconvoluter, SmoothingAlgo};
    ///
    /// # fn main() -> metabodecon::Result<()> {
    /// let mut deconvoluter = Deconvoluter::default();
    ///
    /// // No regions are ignored by default.
    /// assert!(deconvoluter.ignore_regions().is_none());
    ///
    /// // Add regions to ignore during deconvolution.
    /// deconvoluter.add_ignore_region((4.7, 4.9))?;
    /// assert!(deconvoluter.ignore_regions().is_some());
    /// assert_eq!(deconvoluter.ignore_regions().unwrap().len(), 1);
    /// deconvoluter.add_ignore_region((5.2, 5.6))?;
    /// assert_eq!(deconvoluter.ignore_regions().unwrap().len(), 2);
    ///
    /// // Overlapping regions are combined.
    /// deconvoluter.add_ignore_region((4.8, 5.4))?;
    /// assert_eq!(deconvoluter.ignore_regions().unwrap().len(), 1);
    /// # Ok(())
    /// # }
    /// ```
    pub fn add_ignore_region(&mut self, new: (f64, f64)) -> Result<()> {
        if let Some(ignore_regions) = self.ignore_regions.as_mut() {
            if !new.0.is_finite()
                || !new.1.is_finite()
                || f64::abs(new.0 - new.1) < 100.0 * f64::EPSILON
            {
                return Err(Error::new(Kind::InvalidIgnoreRegion { region: new }).into());
            }
            ignore_regions.push((f64::min(new.0, new.1), f64::max(new.0, new.1)));
            ignore_regions.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
            while let Some(overlap_position) = ignore_regions
                .windows(2)
                .position(|w| w[1].0 < w[0].1 || f64::abs(w[0].1 - w[1].0) < 100.0 * f64::EPSILON)
            {
                let combined = (
                    f64::min(
                        ignore_regions[overlap_position].0,
                        ignore_regions[overlap_position + 1].0,
                    ),
                    f64::max(
                        ignore_regions[overlap_position].1,
                        ignore_regions[overlap_position + 1].1,
                    ),
                );
                ignore_regions.remove(overlap_position);
                ignore_regions.remove(overlap_position);
                ignore_regions.insert(overlap_position, combined);
            }
        } else {
            self.ignore_regions = Some(vec![new]);
        }

        Ok(())
    }

    /// Clears the regions to ignore during deconvolution.
    ///
    /// # Example
    ///
    /// ```
    /// use metabodecon::deconvolution::{Deconvoluter, SmoothingAlgo};
    ///
    /// # fn main() -> metabodecon::Result<()> {
    /// let mut deconvoluter = Deconvoluter::default();
    ///
    /// deconvoluter.add_ignore_region((4.7, 4.9))?;
    /// deconvoluter.clear_ignore_regions();
    /// assert!(deconvoluter.ignore_regions().is_none());
    /// # Ok(())
    /// # }
    /// ```
    pub fn clear_ignore_regions(&mut self) {
        self.ignore_regions = None;
    }

    /// Deconvolutes the provided spectrum into individual signals.
    ///
    /// # Errors
    ///
    /// During the deconvolution process, the algorithm relies on finding peaks
    /// in the `Spectrum`. If no peaks are found, an error is returned. The
    /// peaks outside the signal boundaries of the `Spectrum` are used to filter
    /// out noise within the signal region. If no peaks are found outside or
    /// within the signal region, an error is returned.
    ///
    /// # Example
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
    /// let spectrum = reader.read_spectrum(
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
    /// let deconvolution = deconvoluter.deconvolute_spectrum(&spectrum)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn deconvolute_spectrum(&self, spectrum: &Spectrum) -> Result<Deconvolution> {
        let peaks = self.select_peaks(spectrum)?;
        let lorentzians = {
            let fitter = match self.fitting_algo {
                FittingAlgo::Analytical { iterations } => FitterAnalytical::new(iterations),
            };
            fitter.fit_lorentzian(spectrum, &peaks)
        };
        let mse = self.compute_mse(
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
    ///
    /// # Errors
    ///
    /// During the deconvolution process, the algorithm relies on finding peaks
    /// in the `Spectrum`. If no peaks are found, an error is returned. The
    /// peaks outside the signal boundaries of the `Spectrum` are used to filter
    /// out noise within the signal region. If no peaks are found outside or
    /// within the signal region, an error is returned.
    ///
    /// # Example
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
    /// let spectrum = reader.read_spectrum(
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
    /// let deconvolution = deconvoluter.par_deconvolute_spectrum(&spectrum)?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "parallel")]
    pub fn par_deconvolute_spectrum(&self, spectrum: &Spectrum) -> Result<Deconvolution> {
        let peaks = self.select_peaks(spectrum)?;
        let lorentzians = {
            let fitter = match self.fitting_algo {
                FittingAlgo::Analytical { iterations } => FitterAnalytical::new(iterations),
            };
            fitter.par_fit_lorentzian(spectrum, &peaks)
        };
        let mse = self.compute_mse(
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

    /// Deconvolutes the provided spectra into individual signals.
    ///
    /// # Errors
    ///
    /// During the deconvolution process, the algorithm relies on finding peaks
    /// in the `Spectrum`. If no peaks are found, an error is returned. The
    /// peaks outside the signal boundaries of the `Spectrum` are used to filter
    /// out noise within the signal region. If no peaks are found outside or
    /// within the signal region, an error is returned.
    ///
    /// # Example
    ///
    /// ```
    /// use metabodecon::deconvolution::{Deconvoluter, Deconvolution, Lorentzian};
    /// use metabodecon::spectrum::BrukerReader;
    ///
    /// # fn main() -> metabodecon::Result<()> {
    /// // Read all spectra from Bruker TopSpin format directories within the root.
    /// let reader = BrukerReader::new();
    /// let path = "path/to/root";
    /// # let path = "../data/bruker/blood";
    /// let spectra = reader.read_spectra(
    ///     path,
    ///     // Experiment number
    ///     10,
    ///     // Processing number
    ///     10,
    ///     // Signal boundaries
    ///     (-2.2, 11.8),
    /// )?;
    ///
    /// // Deconvolute the spectra.
    /// let deconvoluter = Deconvoluter::default();
    /// let deconvolution = deconvoluter.deconvolute_spectra(&spectra)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn deconvolute_spectra(&self, spectra: &[Spectrum]) -> Result<Vec<Deconvolution>> {
        let deconvolutions = spectra
            .iter()
            .map(|spectrum| self.deconvolute_spectrum(spectrum))
            .collect::<Result<Vec<Deconvolution>>>()?;

        Ok(deconvolutions)
    }

    /// Deconvolutes the provided spectra into individual signals in parallel.
    ///
    /// # Errors
    ///
    /// During the deconvolution process, the algorithm relies on finding peaks
    /// in the `Spectrum`. If no peaks are found, an error is returned. The
    /// peaks outside the signal boundaries of the `Spectrum` are used to filter
    /// out noise within the signal region. If no peaks are found outside or
    /// within the signal region, an error is returned.
    ///
    /// # Example
    ///
    /// ```
    /// use metabodecon::deconvolution::{Deconvoluter, Deconvolution, Lorentzian};
    /// use metabodecon::spectrum::BrukerReader;
    ///
    /// # fn main() -> metabodecon::Result<()> {
    /// // Read all spectra from Bruker TopSpin format directories within the root.
    /// let reader = BrukerReader::new();
    /// let path = "path/to/root";
    /// # let path = "../data/bruker/blood";
    /// let spectra = reader.read_spectra(
    ///     path,
    ///     // Experiment number
    ///     10,
    ///     // Processing number
    ///     10,
    ///     // Signal boundaries
    ///     (-2.2, 11.8),
    /// )?;
    ///
    /// // Deconvolute the spectra.
    /// let deconvoluter = Deconvoluter::default();
    /// let deconvolution = deconvoluter.par_deconvolute_spectra(&spectra)?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "parallel")]
    pub fn par_deconvolute_spectra(&self, spectra: &[Spectrum]) -> Result<Vec<Deconvolution>> {
        let deconvolutions = spectra
            .par_iter()
            .map(|spectrum| self.par_deconvolute_spectrum(spectrum))
            .collect::<Result<Vec<Deconvolution>>>()?;

        Ok(deconvolutions)
    }

    /// Internal helper function to perform the peak selection step.
    fn select_peaks(&self, spectrum: &Spectrum) -> Result<Vec<Peak>> {
        let mut smoother = match self.smoothing_algo {
            SmoothingAlgo::MovingAverage {
                iterations,
                window_size,
            } => MovingAverage::new(iterations, window_size),
        };
        let mut intensities = spectrum.intensities().to_vec();
        smoother.smooth_values(&mut intensities);
        let selector = match self.selection_algo {
            SelectionAlgo::NoiseScoreFilter {
                scoring_algo,
                threshold,
            } => NoiseScoreFilter::new(scoring_algo, threshold),
        };
        let ignore_regions = self.ignore_region_indices(spectrum);
        let peaks = selector.select_peaks(
            &intensities,
            spectrum.signal_boundaries_indices(),
            &ignore_regions,
        )?;

        Ok(peaks)
    }

    /// Internal helper function to compute the MSE within the signal region.
    fn compute_mse(&self, spectrum: &Spectrum, superpositions: Vec<f64>) -> f64 {
        let regions = match self.ignore_region_indices(spectrum) {
            Some(ignore_regions) => {
                let iter = std::iter::once(spectrum.signal_boundaries_indices().0)
                    .chain(
                        ignore_regions
                            .iter()
                            .flat_map(|(start, end)| vec![*start, *end]),
                    )
                    .chain(std::iter::once(spectrum.signal_boundaries_indices().1));

                iter.clone()
                    .step_by(2)
                    .zip(iter.skip(1).step_by(2))
                    .collect::<Vec<(usize, usize)>>()
            }
            None => vec![spectrum.signal_boundaries_indices()],
        };
        let residuals = regions
            .iter()
            .map(|(start, end)| {
                superpositions[*start..*end]
                    .iter()
                    .zip(spectrum.intensities()[*start..*end].iter())
                    .map(|(superposition, intensity)| (superposition - intensity).powi(2))
                    .sum::<f64>()
            })
            .sum::<f64>();
        let length = regions
            .iter()
            .map(|(start, end)| end - start)
            .sum::<usize>();

        residuals / (length as f64)
    }

    /// Internal helper function to convert the ignore regions to indices.
    fn ignore_region_indices(&self, spectrum: &Spectrum) -> Option<Vec<(usize, usize)>> {
        if let Some(ignore_regions) = self.ignore_regions.as_ref() {
            let step = spectrum.step();
            let first = spectrum.chemical_shifts()[0];
            let boundaries = spectrum.signal_boundaries_indices();
            let (lower, upper) = (
                usize::min(boundaries.0, boundaries.1),
                usize::max(boundaries.0, boundaries.1),
            );
            let indices = ignore_regions
                .iter()
                .filter_map(|(start, end)| {
                    let min = f64::min(*start, *end);
                    let max = f64::max(*start, *end);
                    let boundaries = (
                        usize::max(((min - first) / step).floor() as usize, lower),
                        usize::min(((max - first) / step).ceil() as usize, upper),
                    );
                    if boundaries.0 < boundaries.1 - 1 {
                        Some(boundaries)
                    } else {
                        None
                    }
                })
                .collect();

            Some(indices)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::assert_approx_eq;

    #[test]
    fn add_ignore_region() {
        let mut deconvoluter = Deconvoluter::default();
        deconvoluter
            .add_ignore_region((1.0, 2.0))
            .unwrap();
        deconvoluter
            .add_ignore_region((3.0, 4.0))
            .unwrap();

        assert!(deconvoluter.ignore_regions().is_some());
        assert_eq!(deconvoluter.ignore_regions().unwrap().len(), 2);

        deconvoluter
            .add_ignore_region((2.0, 3.0))
            .unwrap();
        assert_eq!(deconvoluter.ignore_regions().unwrap().len(), 1);
        assert_approx_eq!(f64, deconvoluter.ignore_regions().unwrap()[0].0, 1.0);
        assert_approx_eq!(f64, deconvoluter.ignore_regions().unwrap()[0].1, 4.0);
    }

    #[test]
    fn clear_ignore_regions() {
        let mut deconvoluter = Deconvoluter::default();
        deconvoluter
            .add_ignore_region((1.0, 2.0))
            .unwrap();
        deconvoluter
            .add_ignore_region((3.0, 4.0))
            .unwrap();
        deconvoluter.clear_ignore_regions();

        assert!(deconvoluter.ignore_regions().is_none());
    }
}
