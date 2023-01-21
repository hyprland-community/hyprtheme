use clap::Parser;

mod cli;
mod helper;
mod parser;

use cli::parse::{Hyprtheme, RepoSubcommand, UtilSubCommand};
use cli::render;
use helper::util;
use parser::config::Config;


#[tokio::main()]
async fn main() {
    let hyprtheme = Hyprtheme::parse();
    match hyprtheme {
        Hyprtheme::Apply(apply) => {
            let config = Config::from_theme(apply.theme);
            render::apply(config)
        }
        Hyprtheme::List(list) => {
            if list.deep {
                render::list_themes_deep(match util::list_themes_deep() {
                    Ok(t) => t,
                    Err(e) => return render::error(e.as_str()),
                })
            } else {
                render::list_themes(util::list_themes())
            }
        }
        Hyprtheme::Repo(repo) => {
            match repo.subcommand {
                Some(subcommand) => match subcommand {
                    RepoSubcommand::Install(install) => {
                        render::install(install.theme).await;
                    }
                    RepoSubcommand::Remove(remove) => {
                        println!("removing: {}", remove.theme);
                    }
                    RepoSubcommand::List(_) => {
                        render::list_themes_deep(match util::Repo::list_themes_deep().await {
                            Ok(t) => t,
                            Err(e) => return render::error(e.as_str()),
                        })
                    }
                },
                None => render::list_themes_deep(match util::Repo::list_themes_deep().await {
                    Ok(t) => t,
                    Err(e) => return render::error(e.as_str()),
                }),
            };
        }
        Hyprtheme::Util(util) => match util.subcommand {
            UtilSubCommand::Kill(kill) => {
                if kill.bars {
                    render::warn("killing bars");
                    util::Util::kill_all_bars(kill.exclude_bar);
                } else if kill.wallpaper {
                    render::warn("killing wallpaper");
                    util::Util::kill_all_wallpapers(kill.exclude_wallpaper);
                } else {
                    render::warn("killing all");
                    util::Util::kill_all();
                }
            }
        },
        Hyprtheme::Init(init) => match util::Util::create_template(init.path) {
            Ok(path) => render::template(&path),
            Err(e) => render::error(e.as_str()),
        },
    }
}
