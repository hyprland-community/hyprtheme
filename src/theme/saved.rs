use globset::{Glob, GlobSetBuilder};
use pathdiff::diff_paths;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use walkdir::WalkDir;

use anyhow::{anyhow, Context, Ok, Result};
use serde::Deserialize;

use expanduser::expanduser;

use crate::consts::{DEFAULT_DOWNLOAD_PATH, DEFAULT_HYPR_CONFIG_PATH};

use super::helper::{
    create_hyrptheme_source_string, create_source_string, create_theme_id, is_theme_installed,
    prepend_to_file, ThemeId,
};
use super::{installed, installed::InstalledTheme};

/// A theme in the data directory, thus saved to the disc.
/// Can be installed, updated, etc
///
/// That is _*NOT*_ an already installed theme. For that see the InstalledTheme struct
#[derive(Deserialize, Debug)]
pub struct SavedTheme {
    /// The path to the theme in the data directory
    path: PathBuf,

    /// The path to the theme config toml file
    config_path: PathBuf,

    /// Parsed theme config
    pub config: ParsedThemeConfig,
}

/// Parsed theme config, does not have computed properties yet
///
/// For that see the Theme struct
#[derive(Deserialize, Debug)]
pub struct ParsedThemeConfig {
    pub meta: ThemeMeta,
    hypr: HyprConfig,
    dots: Vec<DotsDirectoryConfig>,
    pub lifetime: LifeTimeConfig,
    // TODO install extra configs
    extra_configs: Vec<ExtraConfig>,
    // TODO install dependencies
    dependencies: Vec<String>,
}

impl SavedTheme {
    /// Has an optional `data_dir` argument, which by default is `~/.local/share/hyprtheme/themes`
    pub async fn install(&self, hypr_dir: Option<&PathBuf>) -> Result<InstalledTheme> {
        let meta = &self.config.meta;

        let hypr_dir = hypr_dir
            .unwrap_or(&expanduser(&DEFAULT_HYPR_CONFIG_PATH).context(format!(
                "Failed to expand home directory for the default install path: {}",
                DEFAULT_HYPR_CONFIG_PATH
            ))?)
            .to_path_buf();

        let install_dir = &hypr_dir.join("./hyprtheme/").join(&meta.name);

        let _ = &self.setup_dots(install_dir).await?;
        let _ = &self.run_setup_script(install_dir, &hypr_dir).await?;
        let _ = setup_theme_variables(&hypr_dir)?;

        // Copy over the hyprtheme.toml, which gets used to determine which theme is currently installed
        fs::copy(&self.config_path, &install_dir.join("hyprtheme.toml"))?;

        let installed_theme = installed::get(Some(&hypr_dir))
            .await
            .context("Failed to retrieve installed theme directly after installtion")?;

        return match installed_theme {
            Some(theme) => Ok(theme),
            None => Err(anyhow!(format!(
                "Failed to install theme. hyprtheme.conf could not be located in {}. Please open up an issue and we shall fix it!",
                hypr_dir.display()
            ))),
        };
    }

    /// hypr_config_dir is the absolute path
    async fn setup_dots(&self, install_dir: &PathBuf) -> Result<()> {
        let _ = self.copy_general_dots()?;
        let _ = self.setup_hyprtheme_hypr_dots(&install_dir)?;

        // TODO  Prompt and install extra configs

        Ok(())
    }

    async fn run_setup_script(&self, install_dir: &PathBuf, hypr_dir: &PathBuf) -> Result<()> {
        let setup_script_path = &self.path.join(&self.config.lifetime.setup);

        if !setup_script_path.try_exists()? {
            print!("No setup script found. Skipping...");
            return Ok(());
        }

        std::process::Command::new("bash")
            .env("THEME_DIR", &self.path)
            .env("HYPR_INSTALL_DIR", &install_dir)
            .env("HYPR_CONFIG_DIR", &hypr_dir)
            .arg(&setup_script_path)
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .current_dir(&self.path)
            .output()?;

        Ok(())
    }

    /// Move the hypr config dir into `~/.config/hypr/hyprtheme`
    /// And then source it
    fn setup_hyprtheme_hypr_dots(&self, hypr_dir: &PathBuf) -> Result<()> {
        fs::copy(
            &self.path.join(&self.config.hypr.location),
            &hypr_dir.join("hypr/hyprtheme/"),
        )
        .context("Failed to copy over hypr config directory")?;
        // Calculate theme source string, like `source=~/.config/hypr/settings.conf`
        // TODO: replace absolute source path with ~ if possible
        // makes config more portable

        let hyprland_config_path = &hypr_dir.join("hyprland.conf");
        let hyprtheme_source_str = create_hyrptheme_source_string(hypr_dir);
        // Read out hyprland.conf via std::fs::read_to_string("list.txt").unwrap();
        // Check if hyprtheme.conf is sourced in hyprland.conf, if not, source it
        let is_already_sourced = std::fs::read_to_string(&hyprland_config_path)
            .context("Failed to read out main hyprtheme.conf")?
            .contains(&hyprtheme_source_str);

        if !is_already_sourced {
            // Append the source line
            // TODO should be sourced after variables sourcing
            let mut config_file = OpenOptions::new()
                .write(true)
                .append(true)
                .open(&hyprland_config_path)?;

            config_file.write_all(&hyprtheme_source_str.as_bytes())?;
        }

        Ok(())
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
                if from_path_relative.starts_with(&self.config.hypr.location) {
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
        is_theme_installed(&self.config.meta.repo, config_dir).await
    }
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
    /// The version of the theme
    pub version: String,
    pub author: String,
    /// Git repository of the theme
    pub repo: String,
    /// The path to the Hyprland config directory
    pub hypr_directory: PathBuf,
    /// Git Branch of the theme repository
    pub branch: Option<String>,
}

impl ThemeMeta {
    pub fn get_id(&self) -> ThemeId {
        create_theme_id(&self.repo, self.branch.as_deref())
    }
}

#[derive(Debug, Deserialize)]
struct HyprConfig {
    /// Relative path to the Hyprland config directory in the theme repository
    location: PathBuf,
    /// Minimum required Hyprland version. Either a semver-string for a tagged release or 'git' for the latest git version.
    // TODO implement version check
    minimum_hyprland_version: String,
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
}

/// Setup and cleanup script paths. If not set, uses default values
#[derive(Debug, Deserialize)]
pub struct LifeTimeConfig {
    /// Gets run after the theme got installed. Usually to restart changed apps
    /// Default path: .hyprtheme/setup.sh - If found it will run it, even if not specified
    pub setup: String,

    /// Gets run after the theme got uninstalled. Usually to kill started apps
    /// Default path: .hyprtheme/cleanup.sh - If found it will run it, even if not specified
    pub cleanup: String,
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
    let config_path = get_theme_toml_path(path)?;
    let config: ParsedThemeConfig = toml::from_str(fs::read_to_string(&config_path)?.as_str())?;

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
        config_path,
        config,
    });
}

/// Get the saved theme in the data directory by its name
///
/// - name: The name of the theme as in its theme.toml config file
/// - data_dir: Data directory of hyprtheme
pub async fn find_saved(repo_url: &str, data_dir: Option<&PathBuf>) -> Result<Option<SavedTheme>> {
    let theme = get_all(data_dir)
        .await?
        .into_iter()
        .find(|theme| theme.config.meta.repo == repo_url);

    return Ok(theme);
}

/// Gets all themes saved in the data directory
pub async fn get_all(data_dir: Option<&PathBuf>) -> Result<Vec<SavedTheme>> {
    let data_dir: PathBuf = data_dir
        .unwrap_or(&expanduser(DEFAULT_DOWNLOAD_PATH)?)
        .to_owned();

    let themes_dir = data_dir.join("/themes/");
    let entries = fs::read_dir(themes_dir)?;

    let mut themes: Vec<SavedTheme> = vec![];
    for entry in entries {
        themes.push(from_directory(&entry?.path()).await?);
    }

    return Ok(themes);
}

/// Get the toml path from the theme directory.
///
/// Nessecary as there are multiple possible locations for it
fn get_theme_toml_path(theme_dir: &PathBuf) -> Result<PathBuf> {
    let locations = ["./.hyprtheme/hyprtheme.toml", "./hyprtheme.toml"];

    for location in &locations {
        let path = theme_dir.join(location);
        match path.try_exists() {
            Result::Ok(true) => return Ok(path),
            Result::Ok(false) => continue, // File does not exist, try the next location
            Err(e) => return Err(e.into()), // Propagate any errors that occurred during the check
        }
    }

    Err(anyhow!(
        "hyprtheme.toml not found in any of the default locations"
    ))
}

fn setup_theme_variables(hypr_dir: &PathBuf) -> Result<()> {
    let hyprland_config_path = &hypr_dir.join("hyprland.conf");

    let filename = "variables.conf";
    let file_path = hypr_dir.join(filename);

    if !file_path.try_exists()? {
        let default_variables = { include_str!("default_variables.conf") };

        fs::write(&file_path, default_variables)?;
    }

    let variables_source_str = create_source_string(&file_path, hypr_dir);

    let is_already_sourced = std::fs::read_to_string(&hyprland_config_path)
        .context("Failed to read out main hyprtheme.conf")?
        .contains(&variables_source_str);

    if !is_already_sourced {
        // Append the source line
        prepend_to_file(&file_path, &variables_source_str)?;
    }

    Ok(())
}
