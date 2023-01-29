use std::cell::RefCell;
use std::rc::Rc;

use serde::{Deserialize, Serialize};

use crate::bag;
use crate::coordinate::Coordinate;
use crate::menu::MenuManager;
use crate::render::Renderer;
use crate::updated::Updated;

use super::TObject;
#[derive(Debug, Serialize, Deserialize)]
pub struct Door {
    pos: Coordinate,
    _goes_to: (usize, Coordinate),
}

impl Door {
    pub fn new(pos: Coordinate, _goes_to: (usize, Coordinate)) -> Door {
        Door { pos, _goes_to }
    }
}

impl TObject for Door {
    fn get_pos(&self) -> Coordinate {
        self.pos
    }

    fn set_pos(&mut self, pos: Coordinate) {
        self.pos = pos;
    }

    fn interact(
        &mut self,
        renderer: &mut Renderer,
        _: &mut MenuManager,
        _player_position: Coordinate,
        bag: Rc<RefCell<bag::Bag>>
    ) -> Updated {
        renderer.play_fade();
        Updated(false)
    }
}
