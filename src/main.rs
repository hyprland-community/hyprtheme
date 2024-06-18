mod cli_flags;
mod consts;
mod theme;
use anyhow::{Context, Result};
use clap::Parser;
use cli_flags::flags::CliFlags;
use expanduser::expanduser;
use std::{fs, process::ExitCode};
use theme::{
    installed,
    saved::{self},
};

#[tokio::main]
async fn main() -> ExitCode {
    ensure_default_dirs_exist().expect("Failed to create default directories");

    match CliFlags::parse() {
        CliFlags::List(arguments) => {
            println!(
                "{}",
                arguments
                    .get_formatted_list()
                    .await
                    .expect("Failed to list the themes")
            );
        }

        CliFlags::Install(arguments) => {
            let installed_theme = arguments.install().await.expect("Failed to install theme.");

            println!("Installed theme {}", installed_theme.config.meta.name);
        }

        CliFlags::Uninstall(arguments) => {
            let installed_theme = installed::get(arguments.hypr_dir.as_ref())
                .await
                .expect("Error retrieving installed theme")
                .expect("No installed theme found");

            let name = &installed_theme.config.meta.name.clone();

            installed_theme
                .uninstall(arguments.hypr_dir.as_ref())
                .await
                .expect("Failed to uninstall theme");

            println!("Succesfully uninstalled theme: {}", &name);
        }

        CliFlags::Update(arguments) => {
            let installed_theme = installed::get(arguments.hypr_dir.as_ref())
                .await
                .expect("Error retrieving installed theme")
                .expect("No installed theme found")
                .update(arguments.data_dir.as_ref())
                .await
                .expect("Failed to update theme");

            println!(
                "Succesfully updated installed theme: {}",
                &installed_theme.config.meta.name
            );
        }

        CliFlags::Remove(arguments) => arguments.remove().await.expect("Failed to remove theme"),

        CliFlags::Clean(arguments) => {
            let themes = saved::get_all(arguments.data_dir.as_ref())
                .await
                .expect("Failed to lookup saved themes.");

            for theme in themes {
                let name = theme.config.meta.name.clone();
                if theme
                    .is_installed(arguments.hypr_dir.as_ref())
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

        CliFlags::UpdateAll(arguments) => {
            let themes = saved::get_all(arguments.data_dir.as_ref())
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

fn ensure_default_dirs_exist() -> Result<()> {
    let _ = fs::create_dir_all(
        expanduser(consts::DEFAULT_DOWNLOAD_PATH)
            .context("Failed to expand default download path.")?,
    )?;
    let _ = fs::create_dir_all(
        expanduser(consts::DEFAULT_HYPR_CONFIG_PATH)
            .context("Failed to expand default hypr config path.")?,
    )?;

    Ok(())
}
