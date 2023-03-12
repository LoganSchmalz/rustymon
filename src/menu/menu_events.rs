use hecs::Entity;

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

#[allow(dead_code)]
#[derive(Clone)]
pub enum MenuCommand {
    OpenStrays,
    OpenBag(Entity),
    OpenSave,
    Close,
    OpenTextbox(String),
    OpenPauseMenu,
}
