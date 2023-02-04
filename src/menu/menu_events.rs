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

pub enum MenuEvent {
    OpenStrays,
	OpenBag,
	OpenSave,
    Close,
}
