// use clap::Parser;

mod cli;
// mod helper;
// mod parser;
mod repo;
mod util;

use clap::Parser;
use cli::parse::Hyprtheme;
// use cli::render;
// use helper::util;
// use parser::config::Config;


#[tokio::main]
async fn main() {
    match Hyprtheme::parse() {
        Hyprtheme::Apply(apply) => {
            println!("apply");
        },
        Hyprtheme::List(list) => {
            for theme in repo::fetch_themes().await.unwrap().themes {
                println!("{}", theme);
            }
        },
        Hyprtheme::Install(install) => {
            println!("install");
        },
        Hyprtheme::Uninstall(uninstall) => {
            println!("uninstall");
        },
    }
}
