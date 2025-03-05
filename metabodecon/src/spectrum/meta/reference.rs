#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// The referencing method used in the NMR experiment.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ReferencingMethod {
    /// An internal reference was used.
    Internal,
    /// An external reference was used.
    External,
}

impl std::str::FromStr for ReferencingMethod {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.trim().to_uppercase();
        let method = match value.as_str() {
            "INTERNAL" => Self::Internal,
            "EXTERNAL" => Self::External,
            _ => return Err(format!("invalid referencing method: {}", s)),
        };

        Ok(method)
    }
}

impl std::fmt::Display for ReferencingMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let method = match self {
            Self::Internal => "internal",
            Self::External => "external",
        };
        write!(f, "{}", method)
    }
}

/// The reference compound used in the NMR experiment.
#[derive(Clone, Debug, Default)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct ReferenceCompound {
    /// The chemical shift of the reference compound in ppm.
    chemical_shift: f64,
    /// The index of the reference compound in the NMR experiment.
    index: usize,
    /// The name of the reference compound.
    name: Option<String>,
    /// The referencing method used in the NMR experiment.
    referencing_method: Option<ReferencingMethod>,
}

impl From<f64> for ReferenceCompound {
    fn from(value: f64) -> Self {
        Self {
            chemical_shift: value,
            ..Default::default()
        }
    }
}

impl From<(f64, usize)> for ReferenceCompound {
    fn from(value: (f64, usize)) -> Self {
        Self {
            chemical_shift: value.0,
            index: value.1,
            ..Default::default()
        }
    }
}

impl ReferenceCompound {
    /// Creates a new `ReferenceCompound`.
    pub fn new(
        chemical_shift: f64,
        index: usize,
        name: Option<String>,
        referencing_method: Option<ReferencingMethod>,
    ) -> Self {
        Self {
            chemical_shift,
            index,
            name,
            referencing_method,
        }
    }

    /// Returns the chemical shift of the reference compound.
    pub fn chemical_shift(&self) -> f64 {
        self.chemical_shift
    }

    /// Returns the index of the reference compound.
    pub fn index(&self) -> usize {
        self.index
    }

    /// Returns the name of the reference compound.
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Returns the referencing method used in the NMR experiment.
    pub fn referencing_method(&self) -> Option<ReferencingMethod> {
        self.referencing_method
    }

    /// Sets the chemical shift of the reference compound.
    pub fn set_chemical_shift(&mut self, chemical_shift: f64) {
        self.chemical_shift = chemical_shift;
    }

    /// Sets the index of the reference compound.
    pub fn set_index(&mut self, index: usize) {
        self.index = index;
    }

    /// Sets the name of the reference compound.
    pub fn set_name(&mut self, name: Option<String>) {
        self.name = name;
    }

    /// Sets the referencing method used in the NMR experiment.
    pub fn set_referencing_method(&mut self, referencing_method: Option<ReferencingMethod>) {
        self.referencing_method = referencing_method;
    }
}
