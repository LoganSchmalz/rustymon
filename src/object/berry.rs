use crate::render::Renderer;
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

    fn interact(&self, _renderer: &mut Renderer, menu_man: &mut MenuManager) -> bool {
        menu_man.open_menu(Box::new(Textbox::new("Don't eat me!".to_string())));
        true
	}
}