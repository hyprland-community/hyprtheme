use clap::Parser;
use crate::{theme::Theme,util};

#[derive(Parser)]
#[command(version, name = "hyprtheme")]
pub enum Hyprtheme {
    Apply(Apply),
    List(List)
}

#[derive(Parser)]
pub struct Apply {
    #[arg(value_parser=parse_theme)]
    pub theme: Theme,
}

#[derive(Parser)]
pub struct List {
    #[arg(long,short)]
    pub deep: bool,
}

fn parse_theme(theme_name: &str) -> Result<Theme, String> {
    let nest = theme_name.split(":").into_iter().collect::<Vec<&str>>();

    match util::find_theme(nest[0]){
        Ok(theme_path) => {
            let mut t= Theme::from_file(theme_path);
            if nest.len() > 1 {
                t.default_subtheme = nest[1..].join(":");
            }
            Ok(t)
        },
        Err(e) => return Err(e),
    }
}




