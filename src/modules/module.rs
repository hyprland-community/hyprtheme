use std::path::PathBuf;

use super::theme::Theme;

use super::installed::{Installed,Partial};

use super::ModuleType;
use crate::util::ansi::{green, reset, bold};

pub struct Module<T: ModuleType> {
    pub standardized_name: String,
    pub name: String,
    pub repo: String,
    pub branch: String,
    pub desc: String,
    pub images: Vec<String>,

    pub module_data: T,
    pub installed: bool,
}

pub fn partial_theme(
    name: String,
    repo: String,
    branch: String,
    desc: String,
    images: Vec<String>,
    config: String,
    install_dir: PathBuf,
) -> Module<Theme<Partial>> {
    Module {
        standardized_name: name.to_lowercase().replace(" ", "_"),
        name: name.clone(),
        repo,
        branch,
        desc,
        images,
        module_data: Theme { config, install_status: Partial { install_dir:install_dir.join(&name) },},
        installed: false,
    }
}

pub fn installed_theme(
    name: String,
    repo: String,
    branch: String,
    desc: String,
    images: Vec<String>,
    config: String,
    install_dir: PathBuf
) -> Module<Theme<Installed>> {
    Module {
        standardized_name: name.to_lowercase().replace(" ", "_"),
        name: name.clone(),
        repo,
        branch,
        desc,
        images,
        module_data: Theme { config, install_status: Installed { install_dir:install_dir.join(&name) },},
        installed: true,
    }
}


impl<T> std::fmt::Display for Module<T> where T:ModuleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut images = String::new();
        for image in &self.images {
            images.push_str(&format!("{}\n", image));
        }
        write!(
            f,
            "{}{}{}{}",
            reset(),match self.installed {
                true => green(false)+&bold()+"● ",
                false => String::from("○ "),
            },self.name,reset()
        )
    }
}
