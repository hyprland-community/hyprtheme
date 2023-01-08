use std::path::{Path, PathBuf};

use clap::Parser;

mod cli;
use cli::Hyprtheme;

struct Theme{
    name: String,
    desc: String,
    path: PathBuf,
    author: String,
    git: String,
    version: String,
    subthemes: Vec<Theme>,
}

fn main() {
    let hyprtheme = Hyprtheme::parse();
    match hyprtheme {
        Hyprtheme::Apply(apply) => {
            println!("{:?}", apply.theme);
        }
    }
}
