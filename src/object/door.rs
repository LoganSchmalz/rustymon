use serde::{Deserialize, Serialize};

use crate::coordinate::Coordinate;
use crate::event::Command;

use super::TObject;
#[derive(Debug, Serialize, Deserialize)]
pub struct Door {
    pos: Coordinate,
    goes_to: (usize, Coordinate),
}

impl Door {
    pub fn new(pos: Coordinate, goes_to: (usize, Coordinate)) -> Door {
        Door { pos, goes_to }
    }
}

impl TObject for Door {
    fn get_pos(&self) -> Coordinate {
        self.pos
    }

    fn set_pos(&mut self, pos: Coordinate) {
        self.pos = pos;
    }

    fn interact(&mut self, _player_position: Coordinate) -> Vec<Command> {
        vec![
            Command::DrawTransition,
            Command::ChangeMap(self.goes_to.0, self.goes_to.1),
        ]
    }
}
