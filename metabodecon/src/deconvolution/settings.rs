use crate::error::Result;

/// Trait interface for deconvolution settings enums.
pub(crate) trait Settings: Default + Clone {
    /// Validates the settings.
    fn validate(&self) -> Result<()>;
}
