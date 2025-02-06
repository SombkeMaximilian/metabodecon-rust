/// Acquisition mode of the spectroscopy experiment.
#[derive(Copy, Clone, Debug)]
pub enum AcquisitionMode {
    /// Single channel detection.
    Qf,
    /// Quadrature detection in sequential mode.
    Qseq,
    /// Quadrature detection in simultaneous mode.
    Qsim,
    /// Digital quadrature detection.
    Dqd,
}
