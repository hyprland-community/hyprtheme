use anyhow::{anyhow, Result};
use std::path::PathBuf;

/// Parse an user-inputted path as PathBuf and verify that it exists
///
/// Useful for commands like `remove` where a non-existing path would not make sense
pub fn parse_path(path: &str) -> Result<PathBuf> {
    let path: PathBuf = shellexpand::tilde(path).as_ref().into();
    if path.try_exists()? {
        Ok(path)
    } else {
        Err(anyhow!(format!("Path does not exist: {}", path.display())))
    }
}
