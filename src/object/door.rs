use crate::coordinate::Coordinate;
use crate::menu::MenuManager;
use crate::render::Renderer;

use super::TObject;
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
    ) -> bool {
        renderer.play_fade();
        false
    }
}
