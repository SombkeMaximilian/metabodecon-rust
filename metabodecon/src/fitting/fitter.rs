use crate::fitting::lorentzian::Lorentzian;
use crate::peak_selection::Peak;
use crate::spectrum::Spectrum;

#[non_exhaustive]
#[derive(Copy, Clone, Debug)]
pub enum FittingAlgo {
    Analytical { iterations: usize },
}

pub trait Fitter {
    fn fit_lorentzian(&self, spectrum: &Spectrum, peaks: &[Peak]) -> Vec<Lorentzian>;

    #[cfg(feature = "parallel")]
    fn par_fit_lorentzian(&self, spectrum: &Spectrum, peaks: &[Peak]) -> Vec<Lorentzian>;
}
