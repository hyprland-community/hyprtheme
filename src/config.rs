use crate::theme::Theme;

#[derive(Debug,Clone)]
pub struct Component {
    name: String,
    comment: String,
    pos: (u32, u32),
    enabled: bool,
}

#[derive(Debug)]
pub struct Variable {
    name: String,
    comment: String,
    value: String,
    var_type: VarType,
    pos: (u32, u32)
}

#[derive(Debug)]
pub enum VarType {
    Slider,
    Text,
    Color,
}

pub struct Config{
    pub raw: String,
    pub theme: Theme
}

impl Config{
    pub fn 
}




