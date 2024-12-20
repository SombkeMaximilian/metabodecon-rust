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

#[cfg(test)]
macro_rules! check_sim_spectrum {
    ($spectrum:expr) => {
        assert_eq!($spectrum.chemical_shifts().len(), 2048);
        assert_eq!($spectrum.intensities().len(), 0);
        assert_eq!($spectrum.intensities_raw().len(), 2048);
        assert_approx_eq!($spectrum.signal_boundaries().0, 3.339007);
        assert_approx_eq!($spectrum.signal_boundaries().1, 3.553942);
        assert_approx_eq!($spectrum.water_boundaries().0, 3.444939);
        assert_approx_eq!($spectrum.water_boundaries().1, 3.448010);
    };
}
