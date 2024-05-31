use globset::{Glob, GlobSetBuilder};
use pathdiff::diff_paths;
use reqwest::header::Entry;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use walkdir::WalkDir;

use anyhow::{anyhow, Context, Ok, Result};
use serde::Deserialize;

use expanduser::expanduser;

use crate::consts::{DEFAULT_DOWNLOAD_PATH, DEFAULT_INSTALL_PATH};

use super::{installed, installed::InstalledTheme};

/// A theme in the data directory, thus saved to the disc.
/// ! Has to be created
/// Can be installed, updated, etc
///
/// That is _*NOT*_ an already installed theme. For that see the InstalledTheme struct
#[derive(Deserialize, Debug)]
pub struct SavedTheme {
    /// The path to the theme in the data directory
    path: PathBuf,

    /// Parsed theme config
    pub config: ParsedThemeConfig,
}

/// Parsed theme config, does not have computed properties yet
///
/// For that see the Theme struct
#[derive(Deserialize, Debug)]
struct ParsedThemeConfig {
    pub meta: ThemeMeta,
    version: String,
    hypr: HyprConfig,
    dots: Vec<DotsDirectoryConfig>,
    lifetime: LifeTimeConfig,
    extra_configs: Vec<ExtraConfig>,
    dependencies: Vec<String>,
}

// Maybe I have `Theme` and `ThemeDir`.
// I clone the repo and the installation is just adding the files to ./hypr and sourcing them
// Why do I even need a `install` method here?
// Just get the installed theme by looking at some file like .config/hypr/hyprtheme/meta.json
//But I still need the install thingy. I actually dont need `Theme` struct for
// the installed thing. It's just an installed theme which can get uninstalled.
// But what about updating? hmmm
// Ez, just look up what is installed, create the corresponding Theme struct, exec the update method,
// and then install method

impl SavedTheme {
    /// Has an optional `data_dir` argument, which by default is `~/.local/share/hyprtheme/themes`
    pub async fn install(&self, data_dir: Option<&PathBuf>) -> Result<InstalledTheme> {
        let meta = &self.config.meta;

        // TODO use default value for theme config instead of option, to simplify this
        let install_dir = data_dir
            .unwrap_or(&expanduser(&DEFAULT_INSTALL_PATH).context(format!(
                "Failed to expand home directory for the default install path: {}",
                DEFAULT_INSTALL_PATH
            ))?)
            .to_path_buf();

        let theme_dir_binding = &install_dir.join(&meta.name);
        let theme_dir = match theme_dir_binding.to_str() {
            Some(string) => Ok(string),
            None => Err(anyhow!(
                "Failed to install: Theme directory contains Non-UTF8 characters.",
            )),
        }?;

        // TODO: Check for update and if yes, prompt for update

        println!("Installing theme {} to {}\n", &meta.name, &theme_dir);

        // Now this is what I want:
        // Check if its download: Yes: Update? and install  | No: Download and install
        // But lets do that later
        // TODO: Check for updates and prompt for it

        // TODO: What to do when the install process fails mid-install?
        // Revert back? Leave it? Crash the OS?

        &self.setup_dots(&install_dir).await?;

        //
        // TODO: Where to run setup.sh in data folder or hypr config folder?
        // Set installed theme data path in $PATH variable or smth, so that setup.sh knows where it can find theme data
        let installed_theme = installed::get(Some(&install_dir))
            .context("Failed to retrieve installed theme directly after installtion")?;

        return match installed_theme {
            Some(theme) => Ok(theme),
            None => Err(anyhow!(format!(
                "Failed to install theme. hyprtheme.conf could not be located in {}. Plz open up an issue and we shall fix it!",
                install_dir.display()
            ))),
        };
    }

    /// hypr_config_dir is the absolute path
    async fn setup_dots(&self, install_dir: &PathBuf) -> Result<()> {
        // First lets copy the regular dots
        // 1. Get all the files
        //  1.1 Get them by the specified root in the hyprtheme.toml
        //  1.2 Copy them over recursivley matching the path
        // TODO 2. Install extra configs
        // TODO 3. Add the variables config

        self.copy_general_dots();
        self.setup_hyprtheme_hypr_dots(&install_dir);

        // TODO: Prompt and install extra configs

        // TODO: Variables
        // Create variables.conf if it does not exists yet in the hypr user dir
        // Append it after the Hyprtheme config, but before the rest

        Ok(())
    }

    /// Move the hypr config dir into `~/.config/hypr/hyprtheme`
    /// And then source it
    fn setup_hyprtheme_hypr_dots(&self, install_dir: &PathBuf) -> Result<()> {
        fs::copy(
            &self.path.join(&self.config.hypr.from),
            &install_dir.join("hypr/hyprtheme/"),
        )
        .context("Failed to copy over hypr config directory")?;
        // Calculate theme source string, like `source=~/.config/hypr/settings.conf`
        // TODO: replace absolute source path with ~ if possible
        // makes config more portable

        // TODO: how to handle custom hypr config locations and different install paths
        let hyprtheme_config_path = &install_dir.join("hypr/hyprtheme.conf");

        let hyprtheme_source_str = "source=".to_string()
            + &install_dir
                .join("hypr/hyprtheme/hyprtheme.conf")
                .to_string_lossy();
        // Read out hyprland.conf via std::fs::read_to_string("list.txt").unwrap();
        // Check if hyprtheme.conf is sourced in hyprland.conf, if not, source it
        let is_already_sourced = std::fs::read_to_string(&hyprtheme_config_path)
            .context("Failed to read out main hyprtheme.conf")?
            .contains(&hyprtheme_source_str);

        if !is_already_sourced {
            // Append the source line
            // TODO should be sourced after variables sourcing
            let mut config_file = OpenOptions::new()
                .write(true)
                .append(true)
                .open(&hyprtheme_config_path)?;

            config_file.write_all(&hyprtheme_source_str.as_bytes())?;
        }

        Ok(())

        // TODO remove outdated sourced config (install path changed after install / user used dots from someone who used hyprtheme and uses hyprtheme too)
    }

    /// Copy over the dot files, as specified in the theme toml.
    ///
    /// Deletes directories/files which would get overwritten to prevent clutter
    ///
    /// Not the hypr dots, as they need special treatment
    fn copy_general_dots(&self) -> Result<()> {
        for dots_config in &self.config.dots {
            let destination_dir = expanduser(dots_config.to.clone().unwrap_or_else(|| {
                let mut to_path = dots_config.from.clone().display().to_string();
                to_path.insert_str(0, "~/");
                to_path
            }))
            .with_context(|| {
                format!(
                    "Failed to get install path for dot files in: {}",
                    &dots_config.from.display()
                )
            })?;

            let root_dots_path = &self.path.join(&dots_config.from);

            let mut ignore_glob = GlobSetBuilder::new();
            for ignore_pattern in &dots_config.ignore {
                ignore_glob.add(Glob::new(&ignore_pattern).with_context(|| {
                    format!("Invalid ignore glob pattern: {}", &ignore_pattern)
                })?);
            }
            let ignore_glob = ignore_glob.build()?;

            let mut include_glob = GlobSetBuilder::new();
            for include_pattern in &dots_config.include {
                include_glob.add(Glob::new(&include_pattern).with_context(|| {
                    format!("Invalid include glob pattern: {}", &include_pattern)
                })?);
            }
            let include_glob = include_glob.build()?;

            // Get top level folders of to_copy to determine what gets deleted/backed up
            let mut top_level_paths: Vec<(PathBuf, PathBuf)> = Vec::new();
            // Filtered by the globs
            let mut from_to_paths: Vec<(PathBuf, PathBuf)> = Vec::new();
            for (index, entry) in WalkDir::new(&root_dots_path)
                .min_depth(1)
                .into_iter()
                .enumerate()
            {
                let from_path = entry?.path().to_path_buf();
                let from_path_relative = diff_paths(&from_path, &self.path).expect(
                    format!(
                        "Failed to find relative path for dot file:\n{}\n{}",
                        &from_path.display(),
                        &self.path.display()
                    )
                    .as_str(),
                );

                // Ignore `hypr_directory` files, as they get treated differently
                if from_path_relative.starts_with(&self.config.hypr.from) {
                    continue;
                }

                // File/dir is ignored and not included again, let's ignore it
                if ignore_glob.is_match(&from_path_relative)
                    && !include_glob.is_match(&from_path_relative)
                {
                    continue;
                }

                let to_path = destination_dir.join(from_path.strip_prefix(&root_dots_path)?);

                // The first iteration contains the top level dirs/files
                if index == 0 {
                    top_level_paths.push((from_path.clone(), to_path.clone()));
                }

                from_to_paths.push((from_path, to_path));
            }

            // TODO: Backup existing files
            //      - Check if hyprtheme-theme is installed. If yes, skip,
            //        as we would overwrite the backup with a theme
            //      - If no, save theme into <data>/backup/<date>/

            for (from, to) in top_level_paths {
                // Delete the file/dir which would get overwritten
                if to.is_file() {
                    fs::remove_file(&to)?;
                } else if to.is_dir() {
                    fs::remove_dir_all(&to)?;
                } else {
                    return Err(anyhow!(format!(
                        "Provided path is not a file nor dir??? {}",
                        &to.display()
                    )))?;
                }

                // Copy it over
                fs::copy(from, &to)?;
            }
        }
        Ok(())
    }

    // async fn copy_other_dots(&self) -> Result<()> {}
    // async fn setup_extra_configs(&self) -> Result<()> {}

    // TODO Create variables.conf if it does not exists yet in the hypr user dir
    // Append it after the Hyprtheme config, but before the rest
    // async fn create_variables_conf(&self) -> Result<()> {}

    /// Remove the theme from the data directory, therefore consumes self
    ///
    /// Used to clean up unused themes
    pub fn remove(self) -> Result<()> {
        println!(
            "Uninstalling theme {} from {}",
            &self.config.meta.name,
            &self
                .path
                .to_str()
                .expect("Theme path contained non-unicode characters. Should not happen.")
        );

        Ok(std::fs::remove_dir_all(&self.path)?)
    }

    /// Update the repository in the data directory
    ///
    /// Returns itself in case you want to chain methods
    pub fn update(&self) -> Result<&Self> {
        println!(
            "Updating theme {} in {}",
            &self.config.meta.name,
            &self.path.display()
        );

        // delete dir
        std::process::Command::new("sh")
            .arg("-c")
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .current_dir(&self.path)
            .arg("git pull")
            .output()?;

        return Ok(&self);
    }

    /// Check if this saved theme is also installed
    pub async fn is_installed(&self, config_dir: Option<&PathBuf>) -> Result<bool> {
        let is_installed = installed::get(config_dir)?
            .map_or(false, |theme| theme.meta.name == self.config.meta.name);

        Ok(is_installed)
    }

    // pub fn ensure_exists(&mut self) -> Result<(), anyhow::Error> {
    //     if self.path.exists() {
    //         return Ok(());
    //     }

    //     let parent = self
    //         .path
    //         .parent()
    //         .context("Failed to get parent path of the theme config.")?;
    //     if !parent.exists() {
    //         std::fs::create_dir_all(parent)?
    //     }

    //     std::fs::write(&self.path, "")
    //         .with_context(|| format!("Failed to write config to {}", self.path.display()))
    // }
}

// display
impl std::fmt::Display for SavedTheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.config.meta.name,)
    }
}

#[derive(Debug, Deserialize)]
pub struct ThemeMeta {
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: String,
    /// Git repository of the theme
    pub git: String,
    /// The path to the Hyprland config directory
    pub hypr_directory: PathBuf,
    /// Git Branch of the theme repository
    pub branch: String,
    // Todo: Do we need to know where the entry config is?
    // Entry point of the hyprtheme Hyprland config file.
}

#[derive(Debug, Deserialize)]
struct HyprConfig {
    /// Relative path to the Hyprland config directory in the theme repository
    from: PathBuf,
    /// Minimum required Hyprland version. Either a semver-string for a tagged release or 'git' for the latest git version.
    min_version: String,
}

/// Configuration on how to move dot files to their locations
#[derive(Debug, Deserialize)]
struct DotsDirectoryConfig {
    /// What to copy relative from the root of the repository
    from: PathBuf,
    /// The destination.
    ///
    /// If not specified it will be `to_copy` appended with `~/`
    to: Option<String>,
    // TODO: Parse this as Vec<Glob> and err if they are not valid globs
    /// Which dirs and files to ignore. Useful to ignore your own installer for example.
    /// By default `[ ".hyprtheme/", "./*\.[md|]", "LICENSE", ".git*" ]` is always ignored
    /// You can ignore everything with ['**/*'] and only `include` the dirs/files which you want
    ignore: Vec<String>,
    // TODO: Parse this as Vec<Glob> and err if they are not valid globs
    /// Regex strings
    include: Vec<String>,
    // TODO: How to do manual moves A->B for edge cases. And are they really nessecary
    // Define how to move files from a to b.
    // Not sure how to do it in a nice way though. Maybe a Hashmap? Record<from, to>
    //# manual_moves = []
}

/// Setup and cleanup script paths. If not set, uses default values
#[derive(Debug, Deserialize)]
struct LifeTimeConfig {
    /// Gets run after the theme got installed. Usually to restart changed apps
    /// Default path: .hyprtheme/setup.sh - If found it will run it, even if not specified
    setup: String,
    /// Gets run after the theme got uninstalled. Usually to kill started apps
    /// Default path: .hyprtheme/cleanup.sh - If found it will run it, even if not specified
    cleanup: String,
}

/// Data for an optional extra configurations, like an optional workspaces setup
/// User can install them or not
#[derive(Debug, Deserialize)]
struct ExtraConfig {
    /// The name of the extra configuration.
    ///
    /// For example: `workspaces` (theme author also provides his own workspace setup, which might interfer with the users one)
    name: String,
    /// Path to the hyprlang `<extra_config>.conf` which will, if selected by the user, sourced by hyprtheme.conf
    path: String,
    /// Gets displayed to the user. Describes what this is
    description: Option<String>,
}

/// Create a theme struct from a hyprtheme repo.
///
/// Like this a themes can be loaded into memory, queried
pub async fn from_directory(path: &PathBuf) -> Result<SavedTheme> {
    // This might need more work. Not all data can be read out from the string
    // I would like to add the theme path, too
    let config: ParsedThemeConfig = toml::from_str(&get_theme_toml_config(&path)?)?;

    // Lets prevent weirdly named themes, like "ФΞд฿Ŀ"
    if !config
        .meta
        .name
        .bytes()
        .all(|character| matches!(character, b'a'..=b'z' | b'0'..=b'9' | b'_' | b'-' | b' '))
    {
        return Err(anyhow!("Theme name contains invalid characters. Only latin letters, numbers and '_', '-' are allowed."));
    }

    return Ok(SavedTheme {
        path: path.to_owned(),
        config,
    });
}

/// Get the saved theme in the data directory by its name
///
/// - name: The name of the theme as in its theme.toml config file
/// - data_dir: Data directory of hyprtheme
pub async fn find_saved(name: &str, data_dir: Option<&PathBuf>) -> Result<Option<SavedTheme>> {
    let theme = get_all(data_dir)
        .await?
        .into_iter()
        .find(|theme| theme.config.meta.name == name);

    return Ok(theme);
}

pub async fn get_all(data_dir: Option<&PathBuf>) -> Result<Vec<SavedTheme>> {
    let data_dir: PathBuf = data_dir
        .unwrap_or(&expanduser(DEFAULT_DOWNLOAD_PATH)?)
        .to_owned();

    let themes_dir = data_dir.join("./themes/");
    let entries = fs::read_dir(themes_dir)?;

    let mut themes: Vec<SavedTheme> = vec![];
    for entry in entries {
        themes.push(from_directory(&entry?.path()).await?);
    }

    return Ok(themes);
}

/// Get the raw string from the hyprtheme theme.toml config
fn get_theme_toml_config(theme_dir: &PathBuf) -> Result<String> {
    let locations = ["./.hyprtheme/hyprtheme.toml", "./hyprtheme.toml"];
    // TODO check that hyprtheme.conf exists in the Hyprland config dir of this this theme

    // Lets keep the errors by not using a find_map in case there are others reasons why the config cannot get accessed
    let config_string = locations
        .iter()
        .map(|location| {
            let config_path = theme_dir.join(location);

            return fs::read_to_string(&config_path).map_err(|error| {
                anyhow::Error::from(error).context(format!(
                    "Failed to read out file at {}",
                    &config_path
                        .to_str()
                        .expect("Provided path has non-unicode characters ")
                ))
            });
        })
        .collect::<Result<String>>();

    config_string
}
