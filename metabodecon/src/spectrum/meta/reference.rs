/// The referencing method used in the NMR experiment.
#[derive(Copy, Clone, Debug)]
pub enum ReferencingMethod {
    /// An internal reference was used.
    Internal,
    /// An external reference was used.
    External,
}

/// The reference compound used in the NMR experiment.
#[derive(Debug)]
pub struct ReferenceCompound {
    /// The name of the reference compound.
    name: String,
    /// The chemical shift of the reference compound in ppm.
    chemical_shift: f64,
}

impl ReferenceCompound {
    /// Creates a new `ReferenceCompound`.
    pub fn new(name: &str, shift: f64) -> Self {
        Self {
            name: name.to_string(),
            chemical_shift: shift,
        }
    }

    /// Returns the name of the reference compound.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the chemical shift of the reference compound.
    pub fn chemical_shift(&self) -> f64 {
        self.chemical_shift
    }
}
