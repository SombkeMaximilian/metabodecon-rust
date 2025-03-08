/// Test utility macro to check if the simulated spectrum was read correctly.
#[macro_export]
#[cfg(test)]
macro_rules! check_sim_spectrum {
    ($spectrum:expr) => {
        assert_eq!($spectrum.chemical_shifts().len(), 2048);
        assert_eq!($spectrum.intensities().len(), 2048);
        assert_approx_eq!(
            f64,
            f64::min(
                $spectrum.signal_boundaries().0,
                $spectrum.signal_boundaries().1
            ),
            3.339007
        );
        assert_approx_eq!(
            f64,
            f64::max(
                $spectrum.signal_boundaries().0,
                $spectrum.signal_boundaries().1
            ),
            3.553942
        );
        assert_eq!(
            $spectrum.nucleus(),
            $crate::spectrum::meta::Nucleus::Hydrogen1
        );
        assert_approx_eq!(f64, $spectrum.frequency(), 600.2528069499997);
        assert_approx_eq!(
            f64,
            $spectrum.reference_compound().chemical_shift(),
            $spectrum.chemical_shifts()[0]
        );
        assert_eq!($spectrum.reference_compound().index(), 0);
        assert_eq!($spectrum.reference_compound().name(), None);
        assert_eq!($spectrum.reference_compound().method(), None);
    };
}
