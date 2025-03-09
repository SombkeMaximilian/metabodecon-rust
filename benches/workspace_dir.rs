use std::path::PathBuf;

pub(crate) fn workspace_dir() -> PathBuf {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    PathBuf::from(manifest_dir).join("..")
}
