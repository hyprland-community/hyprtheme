mod helper;
mod install;
mod list;
mod remove;

use {helper::parse_path, install::InstallArgs, list::List, remove::RemoveArgs};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, name = "hyprtheme")]
pub struct CliParser{

    /// The path to the hyprland config directory.
    #[arg(long,value_parser=parse_path,default_value="~/.config/hypr/")]
    pub hypr_dir: PathBuf,

    /// The path to the hyprtheme data directory.
    #[arg(short, long, value_parser=parse_path, default_value="~/.config/hypr/themes/")]
    pub themes_dir: PathBuf,


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

#[derive(Parser,Clone)]
pub struct UninstallArgs {
}

#[derive(Parser,Clone)]
pub struct UpdateArgs {
}

#[derive(Parser,Clone)]
pub struct CleanAllArgs {
}


