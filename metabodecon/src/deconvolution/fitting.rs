mod fitter;
mod fitter_analytical;
mod lorentzian;
mod peak_stencil_data;
mod reduced_spectrum;

pub(crate) use fitter::Fitter;
pub(crate) use fitter_analytical::FitterAnalytical;
pub(crate) use peak_stencil_data::PeakStencilData;
pub(crate) use reduced_spectrum::ReducedSpectrum;

pub use fitter::FittingAlgo;
pub use lorentzian::Lorentzian;
