use crate::deconvolution::Deconvolution;
use std::sync::Arc;

#[derive(Debug)]
pub struct Alignment {
    deconvolutions: Arc<[Deconvolution]>,
}

impl Alignment {
    pub fn new<I: IntoIterator<Item = Deconvolution>>(deconvolutions: I) -> Self {
        Self {
            deconvolutions: deconvolutions.into_iter().collect(),
        }
    }

    pub fn deconvolutions(&self) -> &[Deconvolution] {
        &self.deconvolutions
    }
}
