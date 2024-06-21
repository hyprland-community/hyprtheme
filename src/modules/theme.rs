use std::{fmt::Display, path::{Path, PathBuf}};

pub mod legacy;
pub mod toml_config;
pub mod installed;
pub mod online;


use anyhow::Result;
use expanduser::expanduser;
use online::OnlineTheme;
use serde::Deserialize;
use std::any::Any;

use reqwest::Client;


use installed::InstalledTheme;
use legacy::LegacyTheme;



#[derive(Hash, Eq, Default, Clone)]
pub struct ThemeId{
    pub repo: String,
    pub branch: Option<String>,
}

impl ThemeId {
    pub fn new(repo: String, branch: Option<String>) -> ThemeId {
        ThemeId {
            repo,
            branch,
        }
    }

    pub fn to_string(&self) -> String {
        match &self.branch {
            Some(branch) => format!("{}@{}", self.repo, branch),
            None => self.repo.clone(),
        }
    }

    pub fn from_theme(theme: Box<dyn ThemeType>) -> ThemeId {
        match theme {
            t if t.as_any().is::<LegacyTheme>() => {
                let t = t.as_any().downcast_ref::<LegacyTheme>().unwrap();
                ThemeId::new(t.partial.repo.clone(), t.partial.branch.clone())
            }
            t if t.as_any().is::<InstalledTheme>() => {
                let t = t.as_any().downcast_ref::<InstalledTheme>().unwrap();
                ThemeId::new(t.partial.repo.clone(), t.partial.branch.clone())
            }
            t if t.as_any().is::<OnlineTheme>() => {
                let t = t.as_any().downcast_ref::<OnlineTheme>().unwrap();
                ThemeId::new(t.partial.repo.clone(), t.partial.branch.clone())
            }
            _ => ThemeId::default(),
        }
    }
}

impl Display for ThemeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.to_string());
    }
}

impl PartialEq for ThemeId {
    fn eq(&self, other: &Self) -> bool {

        if self.repo == "unknown" || other.repo == "unknown" {
            return false;
        }

        self.repo == other.repo && self.branch == other.branch
    }
}


pub trait ThemeType : Any {
    fn as_any(&self) -> &dyn Any;

    fn get_id(&self) -> ThemeId;

    fn get_name(&self) -> String;
    fn get_repo(&self) -> String;
    fn get_branch(&self) -> Option<String>;
    fn get_desc(&self) -> String;
    fn get_images(&self) -> Vec<String>;
}


#[derive(Deserialize, Debug,Clone)]
pub struct Theme{
    pub name: String,
    pub repo: String,
    pub branch: Option<String>,
    pub config: Option<String>,
    pub desc: String,
    pub images: Vec<String>,
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            name: String::new(),
            repo: String::new(),
            branch: None,
            config: None,
            desc: String::new(),
            images: Vec::new(),
        }
    }
}

impl Theme{
    pub fn new(name: String, repo: String, branch: Option<String>, desc: String, images: Vec<String>) -> Theme {
        Theme {
            name,
            repo,
            branch,
            desc,
            images,
            ..Default::default()
        }
    }
}

pub async fn legacy_themes(themes_dir: &PathBuf) -> Result<Vec<LegacyTheme>> {
    let mut themes: Vec<LegacyTheme> = Vec::new();

    for entry in std::fs::read_dir(themes_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let config_path = path.join("theme.conf");
            if config_path.exists() && config_path.is_file() {
                themes.push(
                    match path.file_name() {
                        Some(name) => {
                            let partial = Theme {
                                name: name.to_string_lossy().to_string(),
                                repo: String::new(),
                                branch: None,
                                config: None,
                                desc: String::new(),
                                images: Vec::new(),
                            };
                            LegacyTheme {
                                installed: true,
                                partial,
                            }
                        },
                        None => {
                            eprintln!("failed parsing theme name for path: {:?}", path);
                            continue
                        },
                    }
                );
            }
        }
    }

    Ok(themes)
}


pub async fn installed_themes(themes_dir: &PathBuf) -> Result<Vec<InstalledTheme>> {
    let mut themes: Vec<InstalledTheme> = Vec::new();

    for entry in std::fs::read_dir(themes_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let config_path = path.join("hyprtheme.toml");
            if config_path.exists() && config_path.is_file() {
                match InstalledTheme::from_file(&config_path) {
                    Ok(theme) => {
                        themes.push(theme);
                    }
                    Err(e) => {
                        eprintln!("skipping broken theme config: {:?}", e);
                    }
                }
            }
        }
    }

    Ok(themes)
}

pub async fn online_themes(urls:Vec<&str>,blacklist_ids:Option<Vec<ThemeId>>) -> Result<Vec<OnlineTheme>> {
    let client = Client::new();

    let blacklist_ids = blacklist_ids.unwrap_or(Vec::new());

    #[derive(Deserialize, Debug)]
    struct ThemesData {
        themes: Vec<Theme>,
    }

    let mut themes: Vec<OnlineTheme> = Vec::new();    
    
    for url in urls {
        match client.get(url).send().await {
            Ok(response) => {
                match serde_json::from_str::<ThemesData>(&response.text().await?) {
                    Ok(data) => {

                        for theme in data.themes {
                            let theme_id = ThemeId::new(theme.repo.clone(), theme.branch.clone());
                            if blacklist_ids.contains(&theme_id) {
                                continue;
                            }
                            themes.push(OnlineTheme::from_theme(theme));
                        }
    
                    }
                    Err(e) => return Err(anyhow::anyhow!("Failed to parse themes json({:?}) : {:?}", url, e))
                }
            }
            Err(e) => return Err(anyhow::anyhow!("Failed to fetch themes json({:?}) : {:?}", url, e))
        }
    }
    Ok(themes)
}

pub async fn fetch_themes(urls:Option<Vec<&str>>, directories: Option<Vec<PathBuf>>) -> Result<Vec<Box<dyn ThemeType>>> {
    let mut themes:Vec<Box<dyn ThemeType>> = Vec::new();

    let mut theme_ids: Vec<ThemeId> = Vec::new();

    let urls = urls.unwrap_or(vec![
        "https://github.com/hyprland-community/theme-repo/blob/main/themes.json?raw=true",
    ]);
    
    let directories = directories.unwrap_or(vec![
        expanduser("~/.config/hypr/themes").expect("Failed to expand themes directory"),
    ]);
    

    for dir in directories {
        match legacy_themes(&dir).await {
            Ok(legacy_themes) => {
                for theme in legacy_themes{
                    themes.push(Box::new(theme.clone()));
                    theme_ids.push(ThemeId::from_theme(Box::new(theme)));
                }
            }
            Err(e) => {
                eprintln!("Failed to fetch legacy themes({:?}) : {:?}", dir,e);
            }
        }
        
        match installed_themes(&dir).await {
            Ok(installed_themes) => {
                for theme in installed_themes{
                    themes.push(Box::new(theme.clone()));
                    theme_ids.push(ThemeId::from_theme(Box::new(theme)));
                }
            }
            Err(e) => {
                eprintln!("Failed to fetch installed themes({:?}) : {:?}", dir,e);
            }
        }
    }
    
    for theme in online_themes(urls, Some(theme_ids)).await? {
        themes.push(Box::new(theme.clone()));
    }

    Ok(themes)
}
        