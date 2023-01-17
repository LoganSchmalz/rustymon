use crate::coordinate::Coordinate;
use crate::menu::{textbox::Textbox, MenuManager};
use crate::render::Renderer;
use crate::{menu, tilemap};

use super::{TObject, CollisionManager};

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
        _renderer: &mut Renderer,
        menu_man: &mut MenuManager,
        _player_position: Coordinate,
    ) -> bool {
        menu_man.open_menu(menu::Menu::Textbox(Textbox::new(
            "Don't eat me!".to_string(),
        )));
        true
    }
}
