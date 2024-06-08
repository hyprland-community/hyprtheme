use anyhow::{anyhow, Result};
use clap::Parser;
use regex::RegexBuilder;
use std::path::PathBuf;

use crate::theme::{self, installed, online, saved};

use super::list;

#[derive(Parser)]
#[command(version, name = "hyprtheme")]
pub enum CliCommands {
    /// List all saved themes
    /// and all featured on the official Hyprtheme site
    List(list::List),

    /// Install a theme from a repository
    ///
    /// Accepted values are:
    /// - Theme name for themes featured on "https://hyprland-community/hyprtheme/browse"
    /// - A git url
    /// - For Github: author/reponame
    Install(Install),

    /// Uninstall the installed theme
    Uninstall(Uninstall),

    /// Update the installed theme
    Update(Update),

    /// Update all saved themes, including the currently installed one
    UpdateAll(Update),

    /// Remove a saved theme from the data directory
    Remove(Remove),

    /// Removes all saved themes, excluding the currently installed one
    Clean(CleanAll),
    // Uri(Uri),
    // Enable(Enable),
    // Disable(Disable),
    // Init,
}

#[derive(Parser)]
pub struct Remove {
    /// The name of the theme to remove from the data directory
    #[arg(short, long)]
    pub theme_name: String,

    /// The data directory of Hyprtheme, by default in `~/.local/share/hyprtheme/`
    #[arg(short, long, default_value = "~/.local/share/hyprtheme",value_parser=parse_path)]
    pub data_dir: PathBuf,
}

#[derive(Parser)]
pub struct Install {
    /// Either:
    /// - Name of a theme featured on www.hyprland-community.org/hyprtheme/browse
    /// - Git repository: git@github.com:hyprland-community/hyprtheme.git
    /// - For Github: author/repo-name
    #[arg(short,long,value_parser=ThemeName::parse)]
    pub name: ThemeName,

    /// The branch of the repository to install
    #[arg(short, long)]
    pub branch: Option<String>,

    /// The data directory of Hyprtheme by default "~/.local/share/hyprtheme/"
    /// The theme will be saved in the sub-directory "themes"
    #[arg(short,long,default_value="~/.local/share/hyprtheme/",value_parser=parse_path)]
    pub data_dir: PathBuf,

    /// The path to the the Hyprland config directory, where the theme will be installed to.
    #[arg(short,long,default_value="~/.config/hypr/themes",value_parser=parse_path)]
    pub hypr_dir: PathBuf,
}

#[derive(Clone)]
pub enum ThemeName {
    /// Name of a theme featured on the Hyprtheme website
    Featured(String),
    /// Repository of a theme, like: git@github.com:hyprland-community/hyprtheme.git
    Git(String),
    /// Short version of a Github repository:
    /// author/repo-name
    ///
    /// Holds a vector of (author, repo-name)
    Github((String, String)),
}
impl ThemeName {
    pub fn parse(string: &str) -> Result<Self> {
        let github_regex = RegexBuilder::new(
            r"^[a-z\d](?:[a-z\d]|-(?=[a-z\d])){0,38}\/[a-z\d](?:[a-z\d]|-(?=[a-z\d]))*$",
        )
        .case_insensitive(true)
        .build()
        .unwrap();
        if github_regex.is_match(string) {
            let (name, repo) = string
                .split_once("/")
                .expect("Git repo regex failed. Could not split at /");

            return Ok(Self::Github((name.to_owned(), repo.to_owned())));
        }

        let git_repo_regex = RegexBuilder::new(
            r"^((git|ssh|http(s)?)|(git@[\w\.-]+))(:(//)?)([\w\.@\:/\-~]+)(\.git)(/)?$",
        )
        .case_insensitive(true)
        .build()
        .unwrap();
        if git_repo_regex.is_match(string) {
            return Ok(Self::Git(string.to_owned()));
        }

        // We cannot fetch theme names here,
        // as this would be async and Clap doesnt like that
        // so we just assume it's a featured theme name
        Ok(Self::Featured(string.to_owned()))
    }
}

#[derive(Parser)]
pub struct Uninstall {
    #[arg(short,long,default_value="~/.config/hypr/",value_parser=parse_path)]
    pub hypr_dir: PathBuf,
}

#[derive(Parser)]
pub struct Update {
    /// Optional: The path to the hyprland config directory. By default "~/.config/hypr/"
    #[arg(short,long,default_value="~/.config/hypr/",value_parser=parse_path)]
    pub hypr_dir: PathBuf,

    /// Optional: The path to the hyprtheme data directory. By default "~/.local/share/hyprtheme/"
    #[arg(short, long, default_value = "~/.local/share/hyprtheme/themes")]
    pub data_dir: PathBuf,
}

#[derive(Parser)]
pub struct CleanAll {
    /// Optional: The path to the hyprtheme data directory. By default "~/.local/share/hyprtheme/"
    #[arg(short,long,default_value="~/.local/share/hyprtheme/themes",value_parser=parse_path)]
    pub data_dir: PathBuf,

    /// Optional: The path to the hyprland config directory. By default "~/.config/hypr/"
    #[arg(short,long,default_value="~/.config/hypr/",value_parser=parse_path)]
    pub hypr_dir: PathBuf,
}

// #[derive(Parser)]
// pub struct Enable {
//     pub theme: String,

//     #[arg(short, long, default_value = "~/.config/hypr/themes/hyprtheme.conf")]
//     pub config: PathBuf,
// }

// #[derive(Parser)]
// pub struct Disable {
//     pub theme: String,

//     #[arg(short, long, default_value = "~/.config/hypr/themes/hyprtheme.conf")]
//     pub config: PathBuf,
// }

// #[derive(Parser)]
// pub struct Uri {
//     pub uri: String,
// }
