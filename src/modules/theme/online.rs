use super::{Theme,ThemeType,ThemeId};

#[derive(Debug,Clone)]
pub struct OnlineTheme {
    pub partial: Theme,
}

impl OnlineTheme {
    pub fn from_theme(partial: Theme) -> OnlineTheme {
        OnlineTheme {
            partial,
        }
    }
}

impl ThemeType for OnlineTheme {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_id(&self) -> ThemeId {
        ThemeId{
            repo: self.partial.repo.clone(),
            branch: self.partial.branch.clone(),
        }
    }

    fn get_name(&self) -> String {
        self.partial.name.clone()
    }

    fn get_repo(&self) -> String {
        self.partial.repo.clone()
    }

    fn get_branch(&self) -> Option<String> {
        self.partial.branch.clone()
    }

    fn get_desc(&self) -> String {
        self.partial.desc.clone()
    }

    fn get_images(&self) -> Vec<String> {
        self.partial.images.clone()
    }
    
}