use serde::{Deserialize, Serialize};

use crate::bag::Item;
use crate::coordinate::Coordinate;
use crate::event::Command;
use crate::menu::menu_events::MenuCommand;

use super::TObject;

#[derive(Debug, Serialize, Deserialize)]

pub struct Berry {
    pos: Coordinate,
}

impl Berry {
    pub fn new(pos: Coordinate) -> Berry {
        Berry { pos }
    }
}

impl TObject for Berry {
    fn get_pos(&self) -> Coordinate {
        self.pos
    }

    fn set_pos(&mut self, pos: Coordinate) {
        self.pos = pos;
    }

    fn interact(&mut self, _player_position: Coordinate) -> Vec<Command> {
        vec![
            Command::OpenMenu(MenuCommand::OpenTextbox("Don't eat me!".to_string())),
            Command::GiveItem(Item::Berry, 1),
            Command::DeleteObject(self.pos),
        ]
    }
}
