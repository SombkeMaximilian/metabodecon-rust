/// Acquisition mode of the spectroscopy experiment.
#[derive(Copy, Clone, Debug)]
pub enum AcquisitionMode {
    /// Single channel detection.
    Qf,
    /// Quadrature detection in sequential mode.
    Qsim,
    /// Quadrature detection in simultaneous mode.
    Qseq,
    /// Digital quadrature detection.
    Dqd,
}
