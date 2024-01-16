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
    pub installed: bool,
}

#[derive(Parser)]
pub struct Install {
    pub theme: String,

    #[arg(short, long, default_value = "~/.config/hypr/themes")]
    pub target: PathBuf,
}

#[derive(Parser)]
pub struct Uninstall {
    #[arg()]
    pub theme: String,
}

fn parse_theme(theme_name: &str) -> Result<Theme, String> {
    tokio::runtime::Runtime::new().unwrap().block_on(find_theme(theme_name))
}
