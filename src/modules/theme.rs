use super::{InstallStatus,ModuleType};
use super::installed::{Installed,Partial};
use super::module::Module;

pub struct Theme<I:InstallStatus> {
    pub config: String,
    pub install_status: I,
}

impl<I> ModuleType for Theme<I> where I:InstallStatus{}
impl<I> InstallStatus for Theme<I> where I:InstallStatus {}

impl<I> Module<Theme<I>> where I:InstallStatus {
    pub fn get_author(&self) -> String {
        let mut split = self.repo.split('/').collect::<Vec<&str>>();
        split.reverse();
        split[1].to_string()
    }

    pub async fn fetch_preview(&self) -> Result<Vec<u8>, String> {
        if self.images.len() == 0 {
            return Err("No preview images found".to_string());
        }
        match reqwest::get(&self.images[0]).await {
            Ok(res) => match res.bytes().await {
                Ok(bytes) => Ok(bytes.to_vec()),
                Err(e) => Err(e.to_string()),
            },
            Err(e) => Err(e.to_string()),
        }
    }
}


impl Module<Theme<Partial>> {
    pub fn install(&self) -> Result<Module<Theme<Installed>>,String> {
        println!("Installing theme: {}", self.name);

        let install_dir = &self.module_data.install_status.install_dir;

        let theme_dir = install_dir.join(&self.standardized_name);

        // check if theme is already installed
        if theme_dir.exists() {
            return Err(format!("Theme {} is already installed", &self.name));
        }

        println!("Installing theme {} to {}\n", &self.name, theme_dir.to_str().unwrap());
        // clone repo
        let clone_cmd = format!("git clone --depth 1 --branch {} {} {}", &self.branch, &self.repo, theme_dir.to_str().unwrap());
        
        match std::process::Command::new("sh")
        .arg("-c")
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .arg(&clone_cmd)
        .output() {
            Ok(_) => Ok(Module {
                standardized_name: self.standardized_name.clone(),
                name: self.name.clone(),
                repo: self.repo.clone(),
                branch: self.branch.clone(),
                desc: self.desc.clone(),
                images: self.images.clone(),
                module_data: Theme { config: self.module_data.config.clone(), install_status: Installed { install_dir: theme_dir },},
                installed: true,
            }),
            Err(e) => return Err(e.to_string()),
        }
    }
}

impl Module<Theme<Installed>> {
    pub fn uninstall(&self) -> Result<Module<Theme<Partial>>,String> {

        let theme_dir = &self.module_data.install_status.install_dir;

        // check if theme is already installed
        if !&theme_dir.exists() {
            return Err(format!("Theme {} is not installed", &self.name));
        }

        println!("Uninstalling theme {} from {}", &self.name, &theme_dir.to_str().unwrap());
        
        // delete dir
        match std::fs::remove_dir_all(&theme_dir) {
            Ok(_) => Ok(Module {
                standardized_name: self.standardized_name.clone(),
                name: self.name.clone(),
                repo: self.repo.clone(),
                branch: self.branch.clone(),
                desc: self.desc.clone(),
                images: self.images.clone(),
                module_data: Theme { config: self.module_data.config.clone(), install_status: Partial { install_dir : theme_dir.to_owned() },},
                installed: false,
            }),
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn update(&self) -> Result<(),String> {
        let install_dir = &self.module_data.install_status.install_dir;

        let theme_dir = install_dir.join(&self.standardized_name);

        // check if theme is installed
        if !theme_dir.exists() {
            return Err(format!("Theme {} is not installed", &self.name));
        }

        println!("Updating theme {} in {}", &self.name, theme_dir.to_str().unwrap());
        
        // delete dir
        match std::process::Command::new("sh")
        .arg("-c")
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .current_dir(&theme_dir)
        .arg("git pull")
        .output() {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }
}