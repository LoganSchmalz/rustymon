use crate::coordinate::Coordinate;
use crate::menu::MenuManager;
use crate::render::Renderer;
use crate::tilemap;

use super::{TObject, CollisionManager};

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

    fn get_prev_pos(&self) -> Coordinate {
        self.pos
    }

    fn set_pos(&mut self, pos: Coordinate) {
        self.pos = pos;
    }

    fn update(&mut self, _delta_time: &f64, _map: &tilemap::TileMap, _: &CollisionManager) -> bool {
        false
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
