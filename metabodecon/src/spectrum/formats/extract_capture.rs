use crate::Result;
use crate::spectrum::error::{Error, Kind};
use regex::Regex;
use std::path::Path;

#[cfg(any(feature = "bruker", feature = "jdx"))]
pub(crate) fn extract_capture<T: std::str::FromStr, P: AsRef<Path>>(
    regex: Regex,
    name: &str,
    text: &str,
    path: P,
) -> Result<T> {
    let make_error = || {
        Error::new(Kind::MissingMetadata {
            path: std::path::PathBuf::from(path.as_ref()),
            regex: regex.to_string(),
        })
    };
    let result = regex
        .captures(text)
        .ok_or_else(make_error)?
        .name(name)
        .ok_or_else(make_error)?
        .as_str()
        .parse::<T>()
        .map_err(|_| make_error())?;

    Ok(result)
}
