mod cli;
use crate::cli::commands::ThemeName;
mod consts;
mod theme;
use clap::Parser;
use cli::commands::CliCommands;
use std::process::ExitCode;
use theme::online;

#[tokio::main]
async fn main() -> ExitCode {
    match CliCommands::parse() {
        CliCommands::List(arguments) => {
            // TODO
            // Fetch all featured and saved themes
            // Display maybe like that:
            // - <Theme name> [Installed]
            // - <Theme name> [Saved]
            // - <Theme name> [Saved]
            // - <Theme name>

            // for theme in repo::fetch_themes(&list.theme_dir, None)
            //     .await
            //     .unwrap()
            //     .themes
            // {
            //     println!("{}", theme);
            // }
        }

        CliCommands::Install(arguments) => {
            struct GitUrlBranch {
                pub url: String,
                pub branch: Option<String>,
            }

            // TODO prompt for update if theme is already installed

            let git_data: GitUrlBranch = match arguments.name {
                ThemeName::Featured(theme) => {
                    let found_theme = saved::find_saved(&theme, Some(&arguments.data_dir))
                        .await
                        .expect("Failed to lookup saved themes")
                        .map(|theme| GitUrlBranch {
                            url: theme.config.meta.repo,
                            branch: None,
                        })
                        .or(online::find_featured(&theme)
                            .await
                            .expect("Failed to fetch featured theme")
                            .map(|theme| GitUrlBranch {
                                url: theme.repo,
                                branch: theme.branch,
                            }))
                        .expect("Failed to find theme by provided name.");

                    GitUrlBranch {
                        url: found_theme.url,
                        branch: arguments.branch.or(found_theme.branch),
                    }
                }

                ThemeName::Github((author, repo)) => GitUrlBranch {
                    url: "git@github.com:".to_string() + &author + "/" + &repo + ".git",
                    branch: arguments.branch,
                },

                ThemeName::Git(github_string) => GitUrlBranch {
                    url: github_string,
                    branch: arguments.branch,
                },
            };

            let saved_theme = saved::find_saved(&git_data.url, Some(&arguments.data_dir)).await;

            //  TODO
            // Check if its download: Yes: Update? and install  | No: Download and install
            //
            match saved_theme {
                Ok(saved) => {}
                Err(_) => {}
            }
            theme::online::download(
                &git_data.url,
                git_data.branch.as_ref(),
                Some(&arguments.data_dir),
            )
            .await
            .expect("Failed to download theme")
            .install(Some(&arguments.data_dir))
            .await
            .expect("Failed to install theme");

            println!("Installing theme {} to {}\n", &meta.name, &theme_dir);
        }

        CliCommands::Uninstall(arguments) => {
            installed::get(Some(&arguments.hypr_dir))
                .await
                .expect("Error retrieving installed theme")
                .expect("No installed theme found")
                .uninstall(Some(&arguments.hypr_dir))
                .expect("Failed to uninstall theme");
        }

        CliCommands::Update(arguments) => {
            installed::get(Some(&arguments.hypr_dir))
                .await
                .expect("Error retrieving installed theme")
                .expect("No installed theme found")
                .update(Some(&arguments.data_dir))
                .await
                .expect("Failed to update theme");
        }

        CliCommands::Remove(arguments) => {
            saved::find_saved(&arguments.theme_name, Some(&arguments.data_dir))
                .await
                .expect("Failed to lookup saved themes.")
                .expect("Could not find specified saved theme.")
                .remove()
                .expect("Failed to remove specified theme.")
        }

        CliCommands::Clean(arguments) => {
            let themes = saved::get_all(Some(&arguments.data_dir))
                .await
                .expect("Failed to lookup saved themes.");

            for theme in themes {
                if theme
                    .is_installed(Some(&arguments.hypr_dir))
                    .await
                    .expect("Failed to check installed theme")
                {
                    continue;
                }
                theme.remove().expect("Failed to remove specified theme.");
            }
        }

        CliCommands::UpdateAll(arguments) => {
            let themes = saved::get_all(Some(&arguments.data_dir))
                .await
                .expect("Failed to lookup saved themes.");

            for theme in themes {
                theme.update().expect("Failed to update theme.");
            }
        }
    }

    return ExitCode::SUCCESS;
}

// async fn install_theme(theme: String, theme_dir: PathBuf) -> ExitCode {
//     let theme = match repo::find_theme(&theme, &theme_dir).await {
//         Ok(theme) => theme,
//         Err(e) => {
//             eprintln!("{}", e);
//             return ExitCode::FAILURE;
//         }
//     };
//     println!("found {}", theme);

//     match theme.install(Some(theme_dir)) {
//         Ok(_) => println!("\ninstalled"),
//         Err(e) => {
//             eprintln!("{}", e);
//             return ExitCode::FAILURE;
//         }
//     }
//     ExitCode::SUCCESS
// }

// async fn uninstall_theme(theme: String, theme_dir: PathBuf) -> ExitCode {
//     let theme = match repo::find_theme(&theme, &theme_dir).await {
//         Ok(theme) => theme,
//         Err(e) => {
//             eprintln!("{}", e);
//             return ExitCode::FAILURE;
//         }
//     };
//     println!("found {}", theme);

//     match theme.uninstall(Some(theme_dir)) {
//         Ok(_) => println!("\nuninstalled"),
//         Err(e) => {
//             eprintln!("{}", e);
//             return ExitCode::FAILURE;
//         }
//     }
//     ExitCode::SUCCESS
// }

// async fn update_theme(theme: String, theme_dir: PathBuf) -> ExitCode {
//     let theme = match repo::find_theme(&theme, &theme_dir).await {
//         Ok(theme) => theme,
//         Err(e) => {
//             eprintln!("{}", e);
//             return ExitCode::FAILURE;
//         }
//     };
//     println!("found {}", theme);

//     match theme.update() {
//         Ok(_) => println!("\nupdated"),
//         Err(e) => {
//             eprintln!("{}", e);
//             return ExitCode::FAILURE;
//         }
//     }
//     ExitCode::SUCCESS
// }
