use super::{InstallStatus,ModuleType};
use super::installed::{Installed,Partial};
use super::module::Module;

pub struct Plugin<I:InstallStatus> {
    pub config: String,

    pub install_status: I,
}

impl<I> ModuleType for Plugin<I> where I:InstallStatus{}
impl<I> InstallStatus for Plugin<I> where I:InstallStatus {}

impl<I> Module<Plugin<I>> where I:InstallStatus {
    
}