use std::path::PathBuf;

use super::{installed::InstalledTheme, Theme, ThemeId, ThemeType};
use anyhow::Result;

use git2::build::RepoBuilder;

#[derive(Debug,Clone)]
pub struct OnlineTheme {
    pub partial: Theme,
}

impl OnlineTheme {
    pub fn from_theme(partial: Theme) -> OnlineTheme {
        OnlineTheme {
            partial,
        }
    }

    pub fn download(self, theme_dir:&PathBuf) -> Result<InstalledTheme> {
        let url = &self.partial.repo;
        let branch = self.partial.branch.clone().unwrap_or("master".to_string());

        let into = theme_dir.join(&self.partial.name);

        match RepoBuilder::new()
            .branch(branch.as_str())
            .clone(url, &into) {
                Ok(_) => {
                    InstalledTheme::from_file(&into.join("hyprtheme.toml"))
                },
                Err(e) => {
                    return Err(anyhow::anyhow!("Failed to clone theme: {}", e));
                }
            }
    }
}

impl ThemeType for OnlineTheme {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_type_string(&self) -> String {
        "online".to_string()
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