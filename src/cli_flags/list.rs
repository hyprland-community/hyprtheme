use super::helper::parse_path;
use crate::theme::{create_theme_id, installed, online, saved, ThemeId};
use anyhow::{Context, Result};
use clap::Parser;
use std::{collections::HashMap, path::PathBuf};

#[derive(Parser)]
pub struct List {
    /// Wether to list featured downloadable themes
    #[arg(short, long, default_value = "false")]
    pub featured: bool,

    /// The path to the the Hyprland config directory
    #[arg(long,default_value="~/.config/hypr/themes",value_parser=parse_path)]
    pub hypr_dir: PathBuf,

    /// The path to the the Hyprtheme data directory
    #[arg(short,long,default_value="~/.config/hypr/themes",value_parser=parse_path)]
    pub data_dir: PathBuf,
    // TODO add as_json: bool,
}

impl List {
    pub async fn get_formatted_list(&self) -> Result<String> {
        let installed_theme: Option<Item> =
            installed::get(Some(&self.hypr_dir))
                .await?
                .map(|theme| Item {
                    installed: true,
                    saved: false,
                    featured: false,
                    name: theme.config.meta.name.clone(),
                    repo: theme.config.meta.repo.clone(),
                    branch: theme.config.meta.branch.clone(),
                });

        let saved_themes: Vec<Item> = saved::get_all(Some(&self.data_dir))
            .await?
            .into_iter()
            .map(|theme| Item {
                saved: true,
                featured: false,
                installed: false,
                name: theme.config.meta.name.clone(),
                repo: theme.config.meta.repo.clone(),
                branch: theme.config.meta.branch.clone(),
            })
            .collect();

        let featured_themes: Option<Vec<Item>> = if self.featured {
            Some(
                online::fetch_themes(None)
                    .await
                    .context("Failed to lookup featured themes.")?
                    .into_iter()
                    .map(|theme| Item {
                        branch: theme.branch,
                        featured: true,
                        installed: false,
                        name: theme.name.clone(),
                        repo: theme.repo.clone(),
                        saved: false,
                    })
                    .collect(),
            )
        } else {
            None
        };

        let all = saved_themes
            .into_iter()
            .chain(featured_themes.into_iter().flatten())
            .chain(installed_theme)
            .fold(HashMap::<ThemeId, Item>::new(), |mut acc, new| {
                let id = new.get_id();

                match acc.get_mut(&id) {
                    Some(theme) => {
                        theme.installed = new.installed || theme.installed;
                        theme.saved = new.saved || theme.saved;
                        theme.featured = new.featured || theme.featured;
                    }
                    None => {
                        acc.insert(id, new);
                    }
                }

                acc
            })
            .into_iter()
            .map(|(_, item)| item);

        let result: String = all
            .map(|item| item.display())
            .collect::<Vec<String>>()
            .join("\n");

        Ok(result)
    }
}

struct Item {
    pub name: String,
    pub installed: bool,
    pub saved: bool,
    pub repo: String,
    pub branch: Option<String>,
    pub featured: bool,
}

impl Item {
    /// Formats the item as `"<theme_name> [installed] [saved]"`
    fn display(&self) -> String {
        let mut flags: Vec<String> = Vec::new();

        if self.installed {
            flags.push("Installed".to_string());
        }
        if self.saved {
            flags.push("Saved".to_string());
        }
        if self.featured {
            flags.push("Featured".to_string());
        }

        let flags = flags
            .into_iter()
            .map(|flag| format!("[{}]", flag))
            .collect::<Vec<String>>()
            .join(" ");

        format!("{} {}", self.name, flags)
    }

    fn get_id(&self) -> ThemeId {
        create_theme_id(&self.repo, self.branch.as_deref())
    }

    // For the future this would be nice
    // fn as_json(&self) -> serde_json::Value {}
}
