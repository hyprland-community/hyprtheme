mod cli;
use crate::cli::commands::ThemeName;
mod consts;
mod theme;
use clap::Parser;
use cli::commands::CliCommands;
use std::process::ExitCode;
use theme::{installed, online, saved};

#[tokio::main]
async fn main() -> ExitCode {
    match CliCommands::parse() {
        CliCommands::List(arguments) => {
            println!(
                "{}",
                arguments
                    .get_formatted_list()
                    .await
                    .expect("Failed to list the themes")
            );
        }

        CliCommands::Install(arguments) => {
            struct GitUrlBranch {
                pub url: String,
                pub branch: Option<String>,
            }

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

            let saved_theme = saved::find_saved(&git_data.url, Some(&arguments.data_dir))
                .await
                .unwrap_or({
                    println!("Failed to lookup saved themes! Downloading theme to be safe...");
                    None
                });

            let saved_theme = match saved_theme {
                Some(saved) => saved,
                None => {
                    let downloaded = theme::online::download(
                        &git_data.url,
                        git_data.branch.as_deref(),
                        Some(&arguments.data_dir),
                    )
                    .await
                    .expect("Failed to download theme");
                    println!("Downloaded theme.");
                    downloaded
                }
            };

            let installed_theme = saved_theme
                .install(Some(&arguments.hypr_dir))
                .await
                .expect("Failed to install theme");

            println!(
                "Installed theme {} to {}\n",
                &installed_theme.config.meta.name,
                &arguments.hypr_dir.display()
            );
        }

        CliCommands::Uninstall(arguments) => {
            let installed_theme = installed::get(Some(&arguments.hypr_dir))
                .await
                .expect("Error retrieving installed theme")
                .expect("No installed theme found");

            let name = &installed_theme.config.meta.name.clone();

            installed_theme
                .uninstall(Some(&arguments.hypr_dir))
                .await
                .expect("Failed to uninstall theme");

            println!("Succesfully uninstalled theme: {}", &name);
        }

        CliCommands::Update(arguments) => {
            let installed_theme = installed::get(Some(&arguments.hypr_dir))
                .await
                .expect("Error retrieving installed theme")
                .expect("No installed theme found")
                .update(Some(&arguments.data_dir))
                .await
                .expect("Failed to update theme");

            println!(
                "Succesfully updated installed theme: {}",
                &installed_theme.config.meta.name
            );
        }

        CliCommands::Remove(arguments) => {
            saved::find_saved(&arguments.theme_name, Some(&arguments.data_dir))
                .await
                .expect("Failed to lookup saved themes.")
                .expect("Could not find specified saved theme.")
                .remove()
                .expect("Failed to remove specified theme.");

            println!("Succesfully removed theme  {}", &arguments.theme_name);
        }

        CliCommands::Clean(arguments) => {
            let themes = saved::get_all(Some(&arguments.data_dir))
                .await
                .expect("Failed to lookup saved themes.");

            for theme in themes {
                let name = theme.config.meta.name.clone();
                if theme
                    .is_installed(Some(&arguments.hypr_dir))
                    .await
                    .expect(format!("Failed to check installed theme: {}", &name).as_str())
                {
                    continue;
                }
                theme
                    .remove()
                    .expect(format!("Failed to remove theme: {}", &name).as_str());
            }

            println!("Succesfully clean unused themes.");
        }

        CliCommands::UpdateAll(arguments) => {
            let themes = saved::get_all(Some(&arguments.data_dir))
                .await
                .expect("Failed to lookup saved themes.");

            for theme in themes {
                theme.update().expect("Failed to update theme.");
            }

            println!("Succesfully updated all themes.");
        }
    }

    return ExitCode::SUCCESS;
}
