use clap::Parser;

mod cli;
mod config;
mod theme;
mod hypr;
mod util;

use cli::Hyprtheme;

fn main() {
    let hyprtheme = Hyprtheme::parse();
    match hyprtheme {
        Hyprtheme::Apply(apply) => {
            let config = config::Config::from_theme(apply.theme);
            println!("applying...");
            hypr::apply(config);
        },
        Hyprtheme::List(list) => {
            if list.deep {
                for theme in util::list_themes_deep(){
                    println!("{}",theme.display());
                };
            } else{
                util::list_themes().iter().for_each(|t| println!("{}",t));
            }
        }
    }
}
