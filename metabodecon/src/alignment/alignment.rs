use crate::deconvolution::Deconvolution;
use std::sync::Arc;

#[derive(Debug)]
pub struct Alignment {
    deconvolutions: Arc<[Deconvolution]>,
}

impl<A: Into<Deconvolution>> FromIterator<A> for Alignment {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        Self {
            deconvolutions: iter.into_iter().map(Into::into).collect(),
        }
    }
}

impl Alignment {
    pub fn deconvolutions(&self) -> &[Deconvolution] {
        &self.deconvolutions
    }
}
