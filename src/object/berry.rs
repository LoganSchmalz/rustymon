use sdl2::{video::Window, render::Canvas, rect::Rect};

use crate::render::{Renderer, Textures, Fonts, PIXELS_X, PIXELS_Y};
use crate::menu::{MenuManager, textbox::Textbox};
use crate::object::TObject;

pub struct Berry {
	pos: (f64, f64),
}

impl Berry {
	pub fn new(pos: (f64, f64)) -> Berry {
		Berry {
            pos
        }
	}
}

impl TObject for Berry {
    fn pos(&self) -> (f64, f64) {
        self.pos
    }

    fn update(&self) {

    }

    fn interact(&self, renderer: &mut Renderer, menu_man: &mut MenuManager) -> bool {
        //pos = self.pos.0 + self.pos.1 as usize * map.size_x;
        //map.objects[pos] = ObjectTile::NONE;
        //map.collision[pos] = CollisionTile::NONE;
        menu_man.open_menu(Box::new(Textbox::new("Don't eat me!".to_string())));
        true
	}
}