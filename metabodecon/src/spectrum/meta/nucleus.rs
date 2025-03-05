#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

/// The NMR nucleus.
#[derive(Clone, Debug, Eq, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Nucleus {
    /// Proton NMR.
    #[default]
    Hydrogen1,
    /// Carbon-13 NMR.
    Carbon13,
    /// Nitrogen-15 NMR.
    Nitrogen15,
    /// Fluorine-19 NMR.
    Fluorine19,
    /// Silicon-29 NMR.
    Silicon29,
    /// Phosphorus-31 NMR.
    Phosphorus31,
    /// Other NMR nuclei.
    ///
    /// This should most likely just be treated as a malformed field.
    Other(String),
}

impl std::str::FromStr for Nucleus {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s
            .trim()
            .replace(" ", "")
            .replace("^", "")
            .replace("-", "")
            .replace("_", "")
            .as_str()
            .to_uppercase();
        let nucleus = match value.as_str() {
            "1H" | "PROTON" | "HYDROGEN1" => Nucleus::Hydrogen1,
            "13C" | "CARBON13" => Nucleus::Carbon13,
            "15N" | "NITROGEN15" => Nucleus::Nitrogen15,
            "19F" | "FLUORINE19" => Nucleus::Fluorine19,
            "29SI" | "SILICON29" => Nucleus::Silicon29,
            "31P" | "PHOSPHORUS31" => Nucleus::Phosphorus31,
            _ => Nucleus::Other(value),
        };

        Ok(nucleus)
    }
}

impl std::fmt::Display for Nucleus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let nucleus = match self {
            Nucleus::Hydrogen1 => "1H",
            Nucleus::Carbon13 => "13C",
            Nucleus::Nitrogen15 => "15N",
            Nucleus::Fluorine19 => "19F",
            Nucleus::Silicon29 => "29Si",
            Nucleus::Phosphorus31 => "31P",
            Nucleus::Other(value) => value.as_str(),
        };
        write!(f, "{}", nucleus)
    }
}
