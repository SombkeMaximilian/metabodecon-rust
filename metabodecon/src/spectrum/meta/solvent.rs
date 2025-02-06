/// The solvent used in the NMR experiment.
#[derive(Debug)]
pub struct Solvent {
    /// The name of the solvent.
    name: String,
}

impl Solvent {
    /// Creates a new `Solvent`.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    /// Returns the name of the solvent.
    pub fn name(&self) -> &str {
        &self.name
    }
}
