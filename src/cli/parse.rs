use std::path::PathBuf;
use clap::Parser;

use crate::repo::find_theme;
use crate::util::theme::Theme;

#[derive(Parser)]
#[command(version, name = "hyprtheme")]
pub enum Hyprtheme {
    Apply(Apply),
    List(List),
    Install(Install),
    Uninstall(Uninstall),
}

#[derive(Parser)]
pub struct Apply {
    #[arg(value_parser=parse_theme)]
    pub theme: Theme,
}

#[derive(Parser)]
pub struct List {

    #[arg(short,long,default_value = "false")]
    pub installed: bool,

    #[arg(short,long,default_value="~/.config/hypr/themes",value_parser=parse_path)]
    pub theme_dir: PathBuf,
}

#[derive(Parser)]
pub struct Install {
    pub theme: String,

    #[arg(short,long,default_value="~/.config/hypr/themes",value_parser=parse_path)]
    pub theme_dir: PathBuf,
}

#[derive(Parser)]
pub struct Uninstall {
    pub theme: String,
}

fn parse_path(path: &str) -> Result<PathBuf, String> {
    // expand ~
    let path = shellexpand::tilde(path);
    let path = path.as_ref();
    let path = PathBuf::from(path);
    if path.exists() {
        Ok(path)
    } else {
        Err(format!("Path does not exist: {}", path.display()))
    }
}

fn parse_theme(theme_name: &str) -> Result<Theme, String> {
    tokio::runtime::Runtime::new().unwrap().block_on(find_theme(theme_name,PathBuf::new()))
}
