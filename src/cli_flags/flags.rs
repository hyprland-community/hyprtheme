use crate::consts;

use super::{helper::parse_path, install::InstallArgs, list, remove::RemoveArgs};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, name = "hyprtheme")]
pub enum CliFlags {
    /// List all saved themes
    /// and all featured on the official Hyprtheme site
    List(list::List),

    /// Install a theme from a repository
    ///
    /// Accepted values are:
    /// - Theme name for themes featured on "https://hyprland-community/hyprtheme/browse"
    /// - A git url
    /// - For Github: author/reponame
    Install(InstallArgs),

    /// Uninstall the installed theme
    Uninstall(UninstallArgs),

    /// Update the installed theme
    Update(UpdateArgs),

    /// Update all saved themes, including the currently installed one
    UpdateAll(UpdateArgs),

    /// Remove a saved theme from the data directory
    ///
    /// Takes a saved theme ID. Use the list command to get the ID.
    Remove(RemoveArgs),

    /// Removes all saved themes, excluding the currently installed one
    Clean(CleanAllArgs),
}

#[derive(Parser)]
pub struct UninstallArgs {
    #[arg(short,long,default_value=consts::DEFAULT_HYPR_CONFIG_PATH,value_parser=parse_path)]
    pub hypr_dir: PathBuf,
}

#[derive(Parser)]
pub struct UpdateArgs {
    /// Optional: The path to the hyprland config directory. By default "~/.config/hypr/"
    #[arg(long,default_value=consts::DEFAULT_HYPR_CONFIG_PATH,value_parser=parse_path)]
    pub hypr_dir: PathBuf,

    /// Optional: The path to the hyprtheme data directory. By default "~/.local/share/hyprtheme/"
    #[arg(short, long, default_value=consts::DEFAULT_DOWNLOAD_PATH)]
    pub data_dir: PathBuf,
}

#[derive(Parser)]
pub struct CleanAllArgs {
    /// Optional: The path to the hyprtheme data directory. By default "~/.local/share/hyprtheme/"
    #[arg(short,long,default_value=consts::DEFAULT_DOWNLOAD_PATH,value_parser=parse_path)]
    pub data_dir: PathBuf,

    /// Optional: The path to the hyprland config directory. By default "~/.config/hypr/"
    #[arg(long,default_value=consts::DEFAULT_HYPR_CONFIG_PATH,value_parser=parse_path)]
    pub hypr_dir: PathBuf,
}
