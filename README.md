# hyprtheme

works with themes installed at `~/.config/hypr/themes`

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
conf = "path_to_hyprland_config.conf"
subthemes = ["path_to_subtheme.toml", "oh_another_one.toml"]
default_subtheme = "the theme name present in path_to_subtheme"
hyprpaper = "path_to_hyprpaper_conf.conf"
```
> note all paths will be relative to your toml file path

a variable named `$THEME_DIR` will be passed to config files that can be used to link to other files inside theme folder

## Commands
+ apply \<theme-name\>
 > theme-name can also contain `:` which represent subthemes, ie `print:dark` will represent `print` theme with subtheme `dark`

+ list [-d,--deep]
 > `--deep` also lists subthemes
