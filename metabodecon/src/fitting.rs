mod fitter;
mod fitter_analytical;
mod fitter_gradient_descent;
mod peak_stencil_data;
mod reduced_spectrum;

pub use fitter::Fitter;
pub use fitter::FittingAlgo;
pub use fitter_analytical::FitterAnalytical;
pub use peak_stencil_data::PeakStencilData;
pub use reduced_spectrum::ReducedSpectrum;
