use super::online::OnlineTheme;
use super::toml_config::Config;
use super::{Theme,ThemeType,ThemeId};

use std::path::PathBuf;
use anyhow::Result;
use git2::Repository;

#[derive(Debug,Clone)]
pub struct InstalledTheme {
    pub config: Config,
    pub path: PathBuf,
    pub partial: Theme,

    pub parent_dir: PathBuf,
}

impl InstalledTheme {

    pub fn from_dir(path: &PathBuf) -> Result<InstalledTheme> {
        
        let config_path = path.join("theme.toml");
        match Config::from_toml_file(&config_path) {
            Ok(config) => {
                Ok(InstalledTheme {
                    config:config.clone(),
                    path:config_path.clone(),
                    parent_dir: path.clone(),
                    partial: Theme::new(
                        config.name,
                        config.repo,
                        config.branch,
                        config.desc,
                        Vec::new(),
                    ),
                })
            }
            Err(e) => {
                Err(e.into())
            }
        }
    }

    pub fn from_file(path: &PathBuf) -> Result<InstalledTheme> {
        match Config::from_toml_file(path) {
            Ok(config) => Ok(InstalledTheme {
                config:config.clone(),
                path:path.clone(),
                parent_dir: path.parent().unwrap().to_path_buf(),
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

    pub fn update(&self) -> Result<()> {
        let repo = Repository::open(&self.parent_dir)?;
        let mut remote = repo.find_remote("origin")?;
        match remote.fetch(&[&self.partial.branch.clone().unwrap_or("master".to_string())], None, None) {
            Ok(_) => Ok(()),
            Err(e) => {
                Err(e.into())
            }
        }
    }

    pub fn uninstall(&self) -> Result<OnlineTheme> {
        let theme = OnlineTheme::from_theme(self.partial.clone());
        std::fs::remove_dir_all(&self.parent_dir)?;
        Ok(theme)
    }

}

impl ThemeType for InstalledTheme {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_type_string(&self) -> String {
        "installed".to_string()
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