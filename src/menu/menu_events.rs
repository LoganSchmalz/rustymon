#[derive(PartialEq, Debug, Clone, Copy)]
pub enum MenuInput {
    Up,
    Down,
    Left,
    Right,
    Accept,
    Reject,
    Start,
}

pub enum MenuCommand {
    OpenStrays,
    OpenBag,
    OpenSave,
    Close,
    OpenTextbox(String),
    OpenPauseMenu,
}
