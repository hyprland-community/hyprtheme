use std::path::{self, PathBuf};

use expanduser::expanduser;

use super::{
    repo,
    theme::{self, Theme},
};

pub struct Module {
    pub name: String,
    pub theme: Option<Theme>,
    pub path: PathBuf,
}

impl Module {
    pub fn new(theme: Option<Theme>, path: PathBuf) -> Module {
        Module {
            name: path.file_name().unwrap().to_str().unwrap().to_string(),
            theme,
            path,
        }
    }

    pub fn from_theme(theme: Theme) -> Module {
        let path = expanduser("~/.config/hypr/themes/")
            .unwrap()
            .join(&theme.name.to_lowercase().replace(" ", "_"));
        Module {
            name: path.file_name().unwrap().to_str().unwrap().to_string(),
            theme: Some(theme),
            path,
        }
    }
}

pub struct Config {
    pub modules: Vec<Module>,
    pub path: PathBuf,
}

impl Config {
    pub fn new() -> Config {
        Config {
            modules: Vec::new(),
            path: PathBuf::new(),
        }
    }

    pub fn ensure_exists(&mut self) -> Result<(), String> {
        if !self.path.exists() {
            let parent = self.path.parent().unwrap();
            if !parent.exists() {
                std::fs::create_dir_all(parent).unwrap();
            }

            match std::fs::write(&self.path, "") {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("Failed to write to {}: {}", self.path.display(), e)),
            }
        } else {
            Ok(())
        }
    }

    pub async fn from(path: PathBuf) -> Config {
        let mut config = Config::new();
        config.path = path.to_owned();

        let parent_path = path.parent().unwrap();

        config.ensure_exists().unwrap();

        // read file at path
        let file = std::fs::read_to_string(&config.path).unwrap();

        // parse file
        let mut lines = file.lines();

        for line in &mut lines {
            if line.starts_with("# modules:") {
                let modules = line.strip_prefix("# modules:").unwrap().split(",");
                for module in modules {
                    let module = module.trim();
                    if module.len() > 0 {
                        let module_path = parent_path.join(module);
                        if module_path.exists() {
                            let module = Module::new(None, module_path);
                            config.add_module(module);
                        }
                    }
                }
            }
        }

        config
    }

    pub fn add_module(&mut self, module: Module) -> Result<(), String> {
        for m in &self.modules {
            if m.name == module.name {
                return Err(format!("Module {} already exists", module.name));
            }
        }
        self.modules.push(module);
        Ok(())
    }

    pub fn remove_module(&mut self, module: Module) -> Result<(), String> {
        self.modules.retain(|m| module.name != m.name);
        Ok(())
    }

    pub fn cleanup(&mut self) -> Result<(), String> {
        for module in &self.modules {
            let cleanup_path = module.path.join("cleanup.sh");
            println!("cleanup_path: {}", &cleanup_path.display());
            if cleanup_path.exists() {
                match std::process::Command::new("chmod")
                    .arg("+x")
                    .arg(&cleanup_path)
                    .output()
                {
                    Ok(_) => println!("chmod cleanup script of {}", module.name),
                    Err(e) => return Err(format!("Failed to chmod cleanup script: {}", e)),
                }
                match std::process::Command::new(cleanup_path).output() {
                    Ok(_) => println!("cleanup of {} successful", module.name),
                    Err(e) => return Err(format!("Failed to run cleanup script: {}", e)),
                }
            }
        }
        Ok(())
    }

    pub fn build(&mut self) -> String {
        // modules comment
        let mut config = String::from("# modules:");
        for module in &self.modules {
            let theme_name = match &module.theme {
                Some(theme) => theme.name.to_lowercase().replace(" ", "_"),
                None => module
                    .path
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
            };
            config.push_str(format!("{},", theme_name).as_str());
        }
        config.push('\n');

        // variables
        config.push_str("\n# variables\n");
        for module in &self.modules {
            let theme_name = match &module.theme {
                Some(theme) => theme.name.to_lowercase().replace(" ", "_"),
                None => module
                    .path
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
            };
            config.push_str(
                format!(
                    "${}={}\n",
                    theme_name.to_lowercase().replace(" ", "_"),
                    module.path.display()
                )
                .as_str(),
            );
        }
        config.push_str("\n# variables end\n");

        // import
        config.push_str("\n# import\n");
        for module in &self.modules {
            config.push_str(format!("source={}/theme.conf\n", module.path.display()).as_str());
        }
        config.push_str("\n# import end\n");

        config
    }

    pub fn apply(&mut self) -> Result<(), String> {
        // apply config
        let config = self.build();

        // let to_kill = [
        //     "eww",
        //     "ags",
        //     "swww",
        //     "waybar"
        // ];

        // for process in to_kill.iter() {
        //     match std::process::Command::new("pkill").arg(process).output() {
        //         Ok(_) => (),
        //         Err(e) => return Err(format!("Failed to kill {}: {}", process, e)),
        //     }
        // }

        match std::fs::write(&self.path, config) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to write to {}: {}", self.path.display(), e)),
        }
    }
}
