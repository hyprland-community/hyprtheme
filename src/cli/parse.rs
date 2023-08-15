use std::path::PathBuf;

use crate::{parser::theme::Theme, util};
use clap::Parser;

#[derive(Parser)]
#[command(version, name = "hyprtheme")]
pub enum Hyprtheme {
    Apply(Apply),
    List(List),
    Repo(Repo),
    Util(Util),
    Init(Init),
}

#[derive(clap::Subcommand)]
pub enum RepoSubcommand {
    Install(Install),
    Remove(Remove),
    List(List),
}

#[derive(clap::Subcommand)]
pub enum UtilSubCommand {
    Kill(Kill),
}

#[derive(Parser)]
pub struct Apply {
    #[arg(value_parser=parse_theme)]
    pub theme: Theme,
}

#[derive(Parser)]
pub struct List {
    #[arg(long, short)]
    pub deep: bool,
}

#[derive(Parser)]
pub struct Install {
    #[arg()]
    pub theme: String,
}

#[derive(Parser)]
pub struct Remove {
    #[arg()]
    pub theme: String,
}

#[derive(Parser)]
pub struct Repo {
    #[command(subcommand)]
    pub subcommand: Option<RepoSubcommand>,

    #[arg()]
    pub theme: Option<String>,
}

#[derive(Parser)]
pub struct Kill {
    #[arg(short, long)]
    pub bars: bool,

    #[arg(short, long)]
    pub wallpaper: bool,

    #[arg(short, long, value_parser=parse_list)]
    pub exclude_bar: Option<Vec<String>>,

    #[arg(short, long, value_parser=parse_list)]
    pub exclude_wallpaper: Option<Vec<String>>,
}

#[derive(Parser)]
pub struct Util {
    #[command(subcommand)]
    pub subcommand: UtilSubCommand,
}

#[derive(Parser)]
pub struct Init {
    #[arg()]
    pub path: Option<PathBuf>,
}

fn parse_theme(theme_name: &str) -> Result<Theme, String> {
    let nest = theme_name.split(':').collect::<Vec<&str>>();

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

fn parse_list(list: &str) -> Result<Vec<String>, String> {
    Ok(list.split(',').map(|s| s.to_string()).collect())
}
