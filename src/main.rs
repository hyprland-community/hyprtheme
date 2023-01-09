use clap::Parser;

mod cli;
mod config;
mod theme;

use cli::Hyprtheme;
use theme::Theme;

fn main() {
    let hyprtheme = Hyprtheme::parse();
    match hyprtheme {
        Hyprtheme::Apply(apply) => {
            let t = Theme::from_file(apply.theme);
            println!("{:#?}", t);
        }
    }
}
