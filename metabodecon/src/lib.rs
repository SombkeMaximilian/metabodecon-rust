/*!
An implementation of the algorithm proposed by [Koh et al.] for the deconvolution of 1D NMR spectra.

[Koh et al.]: https://pubmed.ncbi.nlm.nih.gov/19804999
*/

mod deconvolution;
mod fitting;
#[rustfmt::skip] #[allow(dead_code, unused_imports)] mod parameter_optimization;
mod peak_selection;
mod smoothing;
mod spectrum;

pub use deconvolution::{Deconvoluter, Deconvolution};
pub use fitting::{FittingAlgo, Lorentzian};
pub use peak_selection::{ScoringAlgo, SelectionAlgo};
pub use smoothing::{MovingAverageAlgo, SmoothingAlgo};
pub use spectrum::{BrukerReader, Hdf5Reader, JdxReader, Spectrum};
