use std::path::PathBuf;

use crate::{parser::theme::Theme, util};
use clap::Parser;

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
}

#[derive(Parser)]
pub struct Install {
    pub theme: String,

    #[arg(short, long, default_value = "~/.config/hypr/themes")]
    pub path: PathBuf,
}

#[derive(Parser)]
pub struct Uninstall {
    #[arg()]
    pub theme: String,
}

fn parse_theme(theme_name: &str) -> Result<Theme, String> {
    let nest = theme_name.split(':').into_iter().collect::<Vec<&str>>();

    match util::find_theme(nest[0]) {
        Ok(theme_path) => {
            let mut t = match Theme::from_file(theme_path) {
                Ok(theme) => theme,
                Err(e) => return Err(e),
            };
            if nest.len() > 1 {
                t.default_subtheme = nest[1..].join(":");
            }
            Ok(t)
        }
        Err(e) => Err(e),
    }
}
