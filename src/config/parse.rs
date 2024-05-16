#[derive(Debug, Deserialize)]
#[derive(Debug, Deserialize)]
struct Config {
    version: String,
    theme: ThemeMeta,
    files: ConfigFilesMeta,
    lifetime: LifeTimeConfig,
    extra_configs: Vec<ExtraConfig>,
    dependencies: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct ThemeMeta {
    name: String,
    description: String,
    version: String,
    author: String,
    git: String,
    /// Entry point of the hyprtheme Hyprland config file.
    /// By default `./hypr/hyprtheme.config` or `./.hyprtheme/hyprtheme.config`
    entry: String,
}

/// Configuration how to move dot files to their locations
#[derive(Debug, Deserialize)]
struct ConfigFilesMeta {
    /// The default is `~/.config` and will be used if not set.
    root: String,
    /// Which dirs and files to ignore. Useful to ignore your own installer for example.
    /// By default `[ ".hyprtheme/", "./*\.[md|]", "LICENSE", ".git*" ]` is always ignored
    /// You can ignore everything with ['**/*'] and only `include` the dirs/files which you want
    ignore: Vec<String>,
    include: Vec<String>,
    //
    // Define how to move files from a to b.
    // Not sure how to do it in a nice way though
    //# manual_moves = []
}

#[derive(Debug, Deserialize)]
struct LifeTimeConfig {
    /// Gets run after the theme got installed. Usually to restart changed apps
    /// Default path: .hyprtheme/setup.sh - If found it will run it, even if not specified
    setup: String,
    /// Gets run after the theme got uninstalled. Usually to kill started apps
    /// Default path: .hyprtheme/cleanup.sh - If found it will run it, even if not specified
    cleanup: String,
}

/// Data for an optional extra configuration file
/// Examples: ASUS Rog Keybinds, personal Workspaces setup
#[derive(Debug, Deserialize)]
struct ExtraConfig {
    name: String,
    path: String,
    description: Option<String>,
}
