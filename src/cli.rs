use clap::Parser;
use anyhow::{anyhow, Result};
use expanduser::expanduser;
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

    // will replace with flags
    // /// Remove a saved theme from the data directory
    // ///
    // /// Takes a saved theme ID. Use the list command to get the ID.
    // Remove(RemoveArgs),
}

// #[derive(Parser,Clone)]
// pub struct RemoveArgs {
//     /// The name of the theme to remove from the data directory
//     #[arg(short, long)]
//     pub theme_name: String,

//     /// The data directory of Hyprtheme, by default in `~/.local/share/hyprtheme/`
//     #[arg(short, long, value_parser=parse_path)]
//     pub data_dir: Option<PathBuf>,
// }

#[derive(Parser,Clone)]
pub struct List {
   
    /// show installed themes
    #[arg(short,long)]
    pub installed: bool,

    /// show online themes excluding the ones already installed
    #[arg(short,long)]
    pub online: bool,

    /// whether to show already installed themes while listing online themes
    #[arg(short,long, requires="online")]
    pub show_installed: bool,

    /// show installed themes that use the legacy format
    #[arg(short,long)]
    pub legacy: bool,

    
}

#[derive(Parser,Clone)]
pub struct InstallArgs {
    /// Either:
    /// - Name of a theme featured on www.hyprland-community.org/hyprtheme/browse
    /// - Git repository: https://github.com/hyprland-community/hyprtheme
    /// - Github short-form: author/repo-name
    // #[arg(short,long,value_parser=ThemeName::parse)]
    // pub name: ThemeName,

    #[arg(short, long, group = "source")]
    pub git: Option<String>,

    #[arg(short, long, requires = "git")]
    pub branch: Option<String>,

    #[arg(group = "source")]
    pub theme_id: Option<String>,
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


/// Parse an user-inputted path as PathBuf and verify that it exists
///
/// Useful for commands like `remove` where a non-existing path would not make sense
pub fn parse_path(path: &str) -> Result<PathBuf> {
    let path: PathBuf = expanduser(path).expect("Failed to expand path");
    if path.try_exists()? {
        Ok(path)
    } else {
        Err(anyhow!(format!("Path does not exist: {}", path.display())))
    }
}