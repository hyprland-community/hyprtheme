mod helper;
mod install;
mod list;
mod remove;

use {helper::parse_path,helper::parse_paths, install::InstallArgs, list::List, remove::RemoveArgs};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, name = "hyprtheme")]
pub struct CliParser{

    /// The path to the hyprland config directory.
    #[arg(long,value_parser=parse_path,default_value="~/.config/hypr/")]
    pub hypr_dir: PathBuf,

    // /// The path to the hyprtheme data directory.
    // #[arg(short, long, value_parser=parse_path, default_value="~/.config/hypr/themes/")]
    // pub themes_dir: PathBuf,

    /// path where themes are stored, can be repeated to add multiple directories
    #[arg(short, long, value_parser=parse_path, default_value="~/.config/hyprtheme/themes")]
    pub theme_dirs: Vec<PathBuf>,

    /// url to raw text files containing a list of themes in json, can be repeated to add multiple urls
    #[arg(short, long, default_value="https://github.com/hyprland-community/theme-repo/blob/main/themes.json?raw=true")]
    pub theme_urls: Vec<String>,

    #[command(subcommand)]
    pub commands: CliCommands,
}

#[derive(Parser, Clone)]
pub enum CliCommands {
    /// List all saved themes
    /// and all featured on the official Hyprtheme site
    List(List),

    /// Install a theme from a repository
    ///
    /// Accepted values are:
    /// - Theme name for themes featured on "https://hyprland-community/hyprtheme/browse"
    /// - A git url
    Install(InstallArgs),

    /// Uninstall the installed theme
    Uninstall(UninstallArgs),

    /// Update the installed theme
    Update(UpdateArgs),

    // /// Remove a saved theme from the data directory
    // ///
    // /// Takes a saved theme ID. Use the list command to get the ID.
    // Remove(RemoveArgs),
}

#[derive(Parser,Clone)]
pub struct UninstallArgs {
    /// uses theme name or theme id (theme_name:branch@repo) to identify the theme to uninstall
    #[arg()]
    pub theme_id: String,
}

#[derive(Parser,Clone)]
pub struct UpdateArgs {
    #[arg()]
    pub theme_id: String,

}

#[derive(Parser,Clone)]
pub struct CleanAllArgs {
}


