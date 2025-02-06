/// The NMR nucleus.
#[derive(Copy, Clone, Debug)]
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
}
