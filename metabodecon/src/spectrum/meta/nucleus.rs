/// The NMR nucleus.
#[derive(Clone, Debug)]
pub enum Nucleus {
    /// Proton NMR.
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
