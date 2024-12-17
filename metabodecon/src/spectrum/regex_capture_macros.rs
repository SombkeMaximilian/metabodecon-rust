macro_rules! extract_capture {
    ($re:expr, $text:expr, $name:expr, $path:expr) => {
        $re.captures($text)
            .ok_or_else(|| {
                Error::new(Kind::MissingMetaData {
                    path: std::path::PathBuf::from($path.as_ref()),
                    regex: $re.to_string(),
                })
            })?
            .name($name)
            .ok_or_else(|| {
                Error::new(Kind::MissingMetaData {
                    path: std::path::PathBuf::from($path.as_ref()),
                    regex: $re.to_string(),
                })
            })?
            .as_str()
            .parse()
            .map_err(|_| {
                Error::new(Kind::MissingMetaData {
                    path: std::path::PathBuf::from($path.as_ref()),
                    regex: $re.to_string(),
                })
            })?
    };
}
