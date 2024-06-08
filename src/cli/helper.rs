use anyhow::{anyhow, Result};
use clap::Parser;
use regex::RegexBuilder;
use std::path::PathBuf;

use crate::theme::{self, installed, online, saved};

pub fn parse_path(path: &str) -> Result<PathBuf> {
    let path: PathBuf = shellexpand::tilde(path).as_ref().into();
    if path.try_exists()? {
        Ok(path)
    } else {
        Err(anyhow!(format!("Path does not exist: {}", path.display())))
    }
}
