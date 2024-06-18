# Hyprtheme

works with themes installed at `~/.local/share/hyprtheme/themes`

additional themes can be installed from [hyprland-community/theme-repo](https://github.com/hyprland-community/theme-repo) or from other repositories.

<hr>

## Features

- browse themes in [hyprland-community/theme-repo](theme-repo)
- install themes from [hyprland-community/theme-repo](theme-repo)
- uninstall themes
- enable/disable themes
- passes a `$<theme-name>` variable to enabled themes that contains the path to theme directory

## Wiki

~~https://github.com/hyprland-community/theme-repo/wiki~~ may be outdated

## Dependencies

Arch:

- rust

```
sudo pacman -S rust
```

## Install

> from git

```
git clone https://github.com/hyprland-community/hyprtheme
cd hyprtheme
make all
```

## Example

https://user-images.githubusercontent.com/77581181/211601026-44109e18-b20c-4d5c-907c-5b151f9f7b85.mp4

> a waybar button that switches active theme using hyprtheme

## Todo

- [ ] install dependencies for themes
- [ ] control value of variables in theme
- [ ] control which components to enable in theme
- [ ] aur pkg
- [ ] JSON for List command
- [ ] Support old format
- [ ] hyprtheme config file
