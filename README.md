# Hyprtheme

works with themes installed at `~/.config/hypr/themes`

additional themes can be installed from [hyprland-community/theme-repo](https://github.com/hyprland-community/theme-repo)

## theme.toml
this file is required to be present at the root of your theme folder

> example toml
```toml
[theme]
name = "fancy theme name"
desc = "a very nice theme description"
version = "0.00001"
author = "me!"
git = "https://www.youtube.com/watch?v=dQw4w9WgXcQ"
config = "path_to_hyprland_config.conf"
subthemes = ["path_to_subtheme.toml", "oh_another_one.toml"]
default_subtheme = "the theme name present in path_to_subtheme"
```
> note all paths will be relative to your toml file path

a variable named `$THEME_DIR` will be passed to config files that can be used to link to other files inside theme folder

## Commands:
  apply \<theme\>
  list   
  repo
    + list
    + install  \<theme\>
  util
    + kill \[-b|--bars\] \[-w|--wallpaper\]
  init \[path\]
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help information
  -V, --version  Print version information

## Wiki
https://github.com/hyprland-community/theme-repo/wiki

## Install

> from git
```
git clone https://github.com/hyprland-community/hyprtheme
cd hyprtheme
sudo make all
```

## Example

https://user-images.githubusercontent.com/77581181/211601026-44109e18-b20c-4d5c-907c-5b151f9f7b85.mp4

> a waybar button that switches active theme using hyprtheme


## Todo

- [ ] better cli
- [ ] async requests
- [ ] handle dependancies for themes
- [ ] aur pkg
- [ ] control value of variables in theme
- [ ] control which components to enable in theme

