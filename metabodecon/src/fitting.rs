mod fitter;
mod fitter_analytical;
mod lorentzian;
mod peak_stencil_data;
mod reduced_spectrum;

pub use fitter::Fitter;
pub use fitter::FittingAlgo;
pub use fitter_analytical::FitterAnalytical;
pub use lorentzian::Lorentzian;
pub use peak_stencil_data::PeakStencilData;
pub use reduced_spectrum::ReducedSpectrum;
