use crate::data::lorentzian::Lorentzian;
use crate::data::spectrum::Spectrum;

pub struct Deconvolution {
    lorentzians: Vec<Lorentzian>,
    spectrum: Spectrum,
}

impl Deconvolution {
    pub fn new() -> Self {
        Self {
            lorentzians: Vec::new(),
            spectrum: Spectrum::new(),
        }
    }

    pub fn lorenztians(&self) -> &Vec<Lorentzian> {
        &self.lorentzians
    }

    pub fn spectrum(&self) -> &Spectrum {
        &self.spectrum
    }
}
