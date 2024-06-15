use std::path::PathBuf;

use super::InstallStatus;

pub struct Partial {
    pub install_dir: PathBuf,
}

pub struct Installed {
    pub install_dir: PathBuf,
}

impl InstallStatus for Installed {}
impl InstallStatus for Partial {}