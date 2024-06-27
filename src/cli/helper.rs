use anyhow::{anyhow, Result};
use expanduser::expanduser;
use std::path::PathBuf;

/// Parse an user-inputted path as PathBuf and verify that it exists
///
/// Useful for commands like `remove` where a non-existing path would not make sense
pub fn parse_path(path: &str) -> Result<PathBuf> {
    let path: PathBuf = expanduser(path).expect("Failed to expand path");
    if path.try_exists()? {
        Ok(path)
    } else {
        Err(anyhow!(format!("Path does not exist: {}", path.display())))
    }
}

pub fn parse_paths(paths: Vec<&str>) -> Result<Vec<PathBuf>> {
    let mut parsed_paths = Vec::new();
    for path in paths {
        parsed_paths.push(parse_path(path)?);
    }
    Ok(parsed_paths)
}