use clap::Parser;

mod cli;
mod config;
mod hypr;
mod theme;
mod util;
mod consts;

use cli::Hyprtheme;

fn main() {
    let hyprtheme = Hyprtheme::parse();
    match hyprtheme {
        Hyprtheme::Apply(apply) => {
            let config = config::Config::from_theme(apply.theme);
            println!("applying...");
            hypr::apply(config);
        }
        Hyprtheme::List(list) => {
            if list.deep {
                for theme in util::list_themes_deep() {
                    println!("{}", theme.display());
                }
            } else {
                util::list_themes().iter().for_each(|t| println!("{}", t));
            }
        }
        Hyprtheme::Repo(repo) => {
            match repo.theme {
                Some(theme) => {
                    println!("theme: {}", theme);
                }
                None => {
                    match repo.subcommand {
                        Some(subcommand ) =>{
                            match subcommand {
                                cli::RepoSubcommand::Install(install) => {
                                    println!("installing: {}", install.theme);
                                    match util::Repo::install_theme(install.theme.as_str()){
                                        Ok(_) => println!("installed: {}", install.theme),
                                        Err(e) => println!("error: {}", e),
                                    };
                                }
                                cli::RepoSubcommand::Remove(remove) => {
                                    println!("removing: {}", remove.theme);
                                }
                                cli::RepoSubcommand::List(_) => {
                                    for theme in util::Repo::list_themes_deep() {
                                        println!("{}",theme.display());
                                    }
                                }
                            }
                        }
                        None => {
                            for theme in util::Repo::list_themes_deep() {
                                println!("{}",theme.display());
                            }
                        }
                    };
                }
            };
        },
        Hyprtheme::Util(util) => {
            match util.subcommand {
                cli::UtilSubCommand::Kill(kill) => {
                    if kill.bars {
                        println!("killing bars");
                        util::Util::kill_all_bars();
                    }
                }
            }
        }
            
    }
}
