use super::toml_config::Config;
use super::{Theme,ThemeType,ThemeId};

use std::path::PathBuf;
use anyhow::Result;

#[derive(Debug,Clone)]
pub struct InstalledTheme {
    pub config: Config,

    pub partial: Theme,
}

impl InstalledTheme {
    pub fn from_file(path: &PathBuf) -> Result<InstalledTheme> {
        match Config::from_toml_file(path) {
            Ok(config) => Ok(InstalledTheme {
                config:config.clone(),
                partial: Theme::new(
                    config.name,
                    config.repo,
                    config.branch,
                    config.desc,
                    Vec::new(),
                ),
            }),
            Err(e) => Err(e),
        }
    }
}

impl ThemeType for InstalledTheme {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_id(&self) -> ThemeId {
        ThemeId{
            repo: self.partial.repo.clone(),
            branch: self.partial.branch.clone(),
        }
    }

    fn get_name(&self) -> String {
        self.partial.name.clone()
    }

    fn get_repo(&self) -> String {
        self.partial.repo.clone()
    }

    fn get_branch(&self) -> Option<String> {
        self.partial.branch.clone()
    }

    fn get_desc(&self) -> String {
        self.partial.desc.clone()
    }

    fn get_images(&self) -> Vec<String> {
        self.partial.images.clone()
    }

}