use anyhow::{anyhow, Result};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, name = "hyprtheme")]
pub enum CliCommands {
    /// List all saved themes
    /// and all featured on the official Hyprtheme site
    List(List),

    /// Install a theme from a repository
    ///
    /// Accepted values are:
    /// - Theme name for themes featured on "https://hyprland-community/hyprtheme/browse"
    /// - A git url
    /// - For Github: author/reponame
    // TODO implement the accepted values
    Install(Install),

    /// Uninstall the installed theme
    Uninstall(Uninstall),

    /// Update the installed theme
    Update(Update),

    /// Update all saved themes, including the currently installed one
    UpdateAll(Update),

    /// Remove a saved theme from the data directory
    Clean(Clean),

    /// Removes all saved themes, excluding the currently installed one
    CleanAll(CleanAll),
    // Uri(Uri),
    // Enable(Enable),
    // Disable(Disable),
    // Init,
}

#[derive(Parser)]
pub struct Clean {
    /// The name of the theme to remove from the data directory
    #[arg(short, long)]
    pub theme_name: String,
}

#[derive(Parser)]
pub struct List {
    /// Wether to list the currently installed one theme too
    #[arg(short, long, default_value = "true")]
    pub installed: bool,

    /// Wether to list featured downloadable themes
    #[arg(short, long, default_value = "true")]
    pub featured: bool,

    /// The path to the the Hyprland config directory
    #[arg(short,long,default_value="~/.config/hypr/themes",value_parser=parse_path)]
    pub config_dir: PathBuf,

    /// The path to the the Hyprtheme data directory
    #[arg(short,long,default_value="~/.config/hypr/themes",value_parser=parse_path)]
    pub data_dir: PathBuf,
}

#[derive(Parser)]
pub struct Install {
    pub theme: String,

    #[arg(short,long,default_value="~/.config/hypr/themes",value_parser=parse_path)]
    pub theme_dir: PathBuf,
}

#[derive(Parser)]
pub struct Uninstall {
    #[arg(short,long,default_value="~/.config/hypr/",value_parser=parse_path)]
    pub config_dir: PathBuf,
}

#[derive(Parser)]
pub struct Update {
    /// Optional: The path to the hyprland config directory. By default "~/.config/hypr/"
    #[arg(short,long,default_value="~/.config/hypr/",value_parser=parse_path)]
    pub theme_dir: PathBuf,
}

#[derive(Parser)]
pub struct CleanAll {
    #[arg(short,long,default_value="~/.local/share/hyprtheme/themes",value_parser=parse_path)]
    pub data_directory: PathBuf,
}

fn parse_path(path: &str) -> Result<PathBuf> {
    let path: PathBuf = shellexpand::tilde(path).as_ref().into();
    if path.exists() {
        Ok(path)
    } else {
        Err(anyhow!(format!("Path does not exist: {}", path.display())))
    }
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
