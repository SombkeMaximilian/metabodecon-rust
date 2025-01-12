/// Macro that extracts a capture group from a regular expression and returns
/// the parsed value or an error if the capture group is missing or could not
/// be parsed.
macro_rules! extract_capture {
    ($re:expr, $text:expr, $name:expr, $path:expr) => {
        $re.captures($text)
            .ok_or_else(|| {
                Error::new(Kind::MissingMetadata {
                    path: std::path::PathBuf::from($path.as_ref()),
                    regex: $re.to_string(),
                })
            })?
            .name($name)
            .ok_or_else(|| {
                Error::new(Kind::MissingMetadata {
                    path: std::path::PathBuf::from($path.as_ref()),
                    regex: $re.to_string(),
                })
            })?
            .as_str()
            .parse()
            .map_err(|_| {
                Error::new(Kind::MissingMetadata {
                    path: std::path::PathBuf::from($path.as_ref()),
                    regex: $re.to_string(),
                })
            })?
    };
}

/// Test utility macro to check if the simulated spectrum was read correctly.
#[cfg(test)]
macro_rules! check_sim_spectrum {
    ($spectrum:expr) => {
        assert_eq!($spectrum.chemical_shifts().len(), 2048);
        assert_eq!($spectrum.intensities_raw().len(), 2048);
        assert!($spectrum.intensities().is_empty());
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
        assert_approx_eq!(
            f64,
            $spectrum.water_boundaries().0,
            3.444939,
            epsilon = 1e-6
        );
        assert_approx_eq!(
            f64,
            $spectrum.water_boundaries().1,
            3.448010,
            epsilon = 1e-6
        );
    };
}

pub(crate) use extract_capture;

#[cfg(test)]
pub(crate) use check_sim_spectrum;
