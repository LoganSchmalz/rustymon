use super::{MenuItem, Menu};

#[derive(PartialEq, Debug)]
pub enum MenuInput {
	Up,
	Down,
	Left,
	Right,
	Accept,
	Reject,
	Start,
}

pub enum MenuEvent {
    Open(Menu),
    Close,
}