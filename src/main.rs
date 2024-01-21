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
            for theme in repo::fetch_themes(&list.theme_dir,None).await.unwrap().themes {
                println!("{}", theme);
            }
        },
        Hyprtheme::Install(install) => {
            let theme = repo::find_theme(&install.theme,&install.theme_dir).await.unwrap();
            println!("found {}", theme);

            match theme.install(Some(install.theme_dir)) {
                Ok(_) => println!("\ninstalled"),
                Err(e) => println!("{}", e),
            }
        },
        Hyprtheme::Uninstall(uninstall) => {
            println!("uninstall");
        },
    }
}
