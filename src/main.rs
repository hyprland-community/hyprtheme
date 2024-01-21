// use clap::Parser;

mod cli;
// mod helper;
// mod parser;
mod repo;
mod util;

use clap::Parser;
use cli::parse::Hyprtheme;
use util::{ansi::{red, reset,bold}, theme::Theme};
use expanduser::expanduser;

use std::{path::PathBuf, process::ExitCode};
// use cli::render;
// use helper::util;
// use parser::config::Config;


async fn install_theme(theme: String, theme_dir: PathBuf) -> ExitCode {
    let theme = match repo::find_theme(&theme,&theme_dir).await {
        Ok(theme) => theme,
        Err(e) => {
            eprintln!("{}{}{}",reset() + &red(false) + &bold() ,e,reset());
            return ExitCode::FAILURE;
        },
    };
    println!("found {}", theme);

    match theme.install(Some(theme_dir)) {
        Ok(_) => println!("\ninstalled"),
        Err(e) => {
            eprintln!("{}{}{}",reset() + &red(false) + &bold() ,e,reset());
            return ExitCode::FAILURE;
        },
    }
    ExitCode::SUCCESS
}

async fn uninstall_theme(theme: String, theme_dir: PathBuf) -> ExitCode {
    let theme = match repo::find_theme(&theme,&theme_dir).await {
        Ok(theme) => theme,
        Err(e) => {
            eprintln!("{}{}{}",reset() + &red(false) + &bold() ,e,reset());
            return ExitCode::FAILURE;
        },
    };
    println!("found {}", theme);

    match theme.uninstall(Some(theme_dir)) {
        Ok(_) => println!("\nuninstalled"),
        Err(e) => {
            eprintln!("{}{}{}",reset() + &red(false) + &bold() ,e,reset());
            return ExitCode::FAILURE;
        },
    }
    ExitCode::SUCCESS
}


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
            return install_theme(install.theme, install.theme_dir).await
        },
        Hyprtheme::Uninstall(uninstall) => {
           return uninstall_theme(uninstall.theme, uninstall.theme_dir).await
        },
        Hyprtheme::Uri(uri) => {
            match uri.uri.strip_prefix("hyprtheme://") {
                Some(uri) => {
                    let uri = uri.split('+').collect::<Vec<&str>>();

                    if uri.len() < 2 {
                        eprintln!("{}{}{}",reset() + &red(false) + &bold() ,"Invalid uri",reset());
                        return ExitCode::FAILURE;
                    }
                    
                    let command = uri[0];
                    let theme = uri[1];
                    
                    let theme_dir = if uri.len() > 2 {
                        PathBuf::from(uri[2])
                    } else {
                        expanduser("~/.config/hypr/themes").unwrap()
                    };

                    match command.to_lowercase().as_str() {
                        "install" => {
                            install_theme(String::from(theme), theme_dir).await;
                        },
                        "uninstall" => {
                            uninstall_theme(String::from(theme), theme_dir).await;
                        },
                        _ => {
                            eprintln!("{}{}{}",reset() + &red(false) + &bold() ,"Invalid command",reset());
                            return ExitCode::FAILURE;
                        },
                    }
                },
                None => {
                    eprintln!("{}{}{}",reset() + &red(false) + &bold() ,"Invalid uri",reset());
                    return ExitCode::FAILURE;
                },
            }
        },
    }
    return ExitCode::SUCCESS;
}
