mod fitter;
mod fitter_analytical;
mod lorentzian;
mod peak_stencil;
mod reduced_spectrum;

pub(crate) use fitter::Fitter;
pub(crate) use fitter_analytical::FitterAnalytical;
pub(crate) use peak_stencil::PeakStencil;
pub(crate) use reduced_spectrum::ReducedSpectrum;

pub use fitter::FittingSettings;
pub use lorentzian::Lorentzian;
