/// Test utility macro to check if the simulated spectrum was read correctly.
#[macro_export]
macro_rules! check_sim_spectrum {
    ($spectrum:expr) => {
        assert_eq!($spectrum.chemical_shifts().len(), 2048);
        assert_eq!($spectrum.intensities().len(), 2048);
        assert_approx_eq!(
            f64,
            $spectrum.signal_boundaries().0,
            3.339007,
            epsilon = 1e-6
        );
        assert_approx_eq!(
            f64,
            $spectrum.signal_boundaries().1,
            3.553942,
            epsilon = 1e-6
        );
    };
}
