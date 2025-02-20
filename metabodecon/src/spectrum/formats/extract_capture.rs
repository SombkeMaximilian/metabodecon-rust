use crate::Result;
use crate::spectrum::error::{Error, Kind};
use regex::Regex;
use std::path::Path;

#[cfg(any(feature = "bruker", feature = "jdx"))]
pub(crate) fn extract_capture<T: std::str::FromStr, P: AsRef<Path>>(
    regex: &Regex,
    name: &str,
    text: &str,
    path: P,
    key: &str,
) -> Result<T> {
    let missing_error = || {
        Error::new(Kind::MissingMetadata {
            path: std::path::PathBuf::from(path.as_ref()),
            key: key.to_string(),
        })
    };
    let malformed_error = || {
        Error::new(Kind::MalformedMetadata {
            path: std::path::PathBuf::from(path.as_ref()),
            key: key.to_string(),
            details: format!("Could not parse {}", std::any::type_name::<T>()),
        })
    };

    let result = regex
        .captures(text)
        .ok_or_else(missing_error)?
        .name(name)
        .ok_or_else(missing_error)?
        .as_str()
        .parse::<T>()
        .map_err(|_| malformed_error())?;

    Ok(result)
}
