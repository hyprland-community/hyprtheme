// use clap::Parser;

// mod cli;
// mod helper;
// mod parser;
mod repo;
mod util;

// use cli::parse::Hyprtheme;
// use cli::render;
// use helper::util;
// use parser::config::Config;


#[tokio::main]
async fn main() {
    let themes = repo::fetch_themes(None).await.unwrap();
    println!("{:?}", themes);
}
//     let hyprtheme = Hyprtheme::parse();
//     match hyprtheme {
//         Hyprtheme::Apply(apply) => {
//             let config = Config::from_theme(apply.theme);
//             render::apply(config)
//         }
//         Hyprtheme::List(list) => {
//         }
//         Hyprtheme::Install(install) => {
//             render::install(install.theme).await;
//         }
//         Hyprtheme::Uninstall(remove) => {
//             println!("removing: {}", remove.theme);
//         }
//     }
// }
