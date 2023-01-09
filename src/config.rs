#[derive(Debug)]
pub struct Component {
    name: String,
    comment: String,
    pos: (u32, u32),
    enabled: bool,
}
