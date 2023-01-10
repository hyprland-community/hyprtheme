use clap::Parser;

mod cli;
mod config;
mod theme;
mod hypr;

use cli::Hyprtheme;
use theme::Theme;

fn main() {
    let hyprtheme = Hyprtheme::parse();
    match hyprtheme {
        Hyprtheme::Apply(apply) => {
            let t = apply.theme;
            println!("applying...");
            hypr::apply(t);
        }
    }
}
