// use clap::Parser;

mod cli;
// mod helper;
// mod parser;
mod repo;
mod util;

use clap::Parser;
use cli::parse::Hyprtheme;
use util::ansi::{red, reset,bold};

use std::process::ExitCode;
// use cli::render;
// use helper::util;
// use parser::config::Config;


#[tokio::main]
async fn main() -> ExitCode{
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
            let theme = match repo::find_theme(&install.theme,&install.theme_dir).await {
                Ok(theme) => theme,
                Err(e) => {
                    eprintln!("{}{}{}",reset() + &red(false) + &bold() ,e,reset());
                    return ExitCode::FAILURE;
                },
            };
            println!("found {}", theme);

            match theme.install(Some(install.theme_dir)) {
                Ok(_) => println!("\ninstalled"),
                Err(e) => {
                    eprintln!("{}{}{}",reset() + &red(false) + &bold() ,e,reset());
                    return ExitCode::FAILURE;
                },
            }
        },
        Hyprtheme::Uninstall(uninstall) => {
            let theme = match repo::find_theme(&uninstall.theme,&uninstall.theme_dir).await {
                Ok(theme) => theme,
                Err(e) => {
                    eprintln!("{}{}{}",reset() + &red(false) + &bold() ,e,reset());
                    return ExitCode::FAILURE;
                },
            };
            println!("found {}", theme);

            match theme.uninstall(Some(uninstall.theme_dir)) {
                Ok(_) => println!("uninstalled"),
                Err(e) => {
                    eprintln!("{}{}{}",reset() + &red(false) + &bold() ,e,reset());
                    return ExitCode::FAILURE;
                },
            }
        },
    }
    return ExitCode::SUCCESS;
}
