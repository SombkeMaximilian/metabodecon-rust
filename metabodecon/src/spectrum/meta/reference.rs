/// The referencing method used in the NMR experiment.
#[derive(Copy, Clone, Debug)]
pub enum ReferencingMethod {
    /// An internal reference was used.
    Internal,
    /// An external reference was used.
    External,
}

/// The reference compound used in the NMR experiment.
#[derive(Clone, Debug)]
pub struct ReferenceCompound {
    /// The chemical shift of the reference compound in ppm.
    chemical_shift: f64,
    /// The name of the reference compound.
    name: Option<String>,
    /// The index of the reference compound in the NMR experiment.
    index: Option<usize>,
    /// The referencing method used in the NMR experiment.
    referencing_method: Option<ReferencingMethod>,
}

impl ReferenceCompound {
    /// Creates a new `ReferenceCompound`.
    pub fn new(
        chemical_shift: f64,
        name: Option<String>,
        index: Option<usize>,
        referencing_method: Option<ReferencingMethod>,
    ) -> Self {
        Self {
            chemical_shift,
            name,
            index,
            referencing_method,
        }
    }

    /// Returns the name of the reference compound.
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Returns the chemical shift of the reference compound.
    pub fn chemical_shift(&self) -> f64 {
        self.chemical_shift
    }

    /// Returns the index of the reference compound.
    pub fn index(&self) -> Option<usize> {
        self.index
    }

    /// Returns the referencing method used in the NMR experiment.
    pub fn referencing_method(&self) -> Option<ReferencingMethod> {
        self.referencing_method
    }
}
