use serde::{Deserialize, Serialize};

use crate::coordinate::Coordinate;
use crate::menu::{textbox::Textbox, MenuManager};
use crate::render::Renderer;
use crate::menu;
use crate::updated::Updated;

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

    fn interact(
        &mut self,
        _renderer: &mut Renderer,
        menu_man: &mut MenuManager,
        _player_position: Coordinate,
    ) -> Updated {
        menu_man.open_menu(menu::Menu::Textbox(Textbox::new(
            "Don't eat me!".to_string(),
        )));
        Updated(true)
    }
}
