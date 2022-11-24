use crate::render::Renderer;
use crate::menu::MenuManager;
use crate::object::TObject;

pub struct Door {
	pos: (f64, f64),
    goes_to: (usize, f64, f64)
}

impl Door {
	pub fn new(pos: (f64, f64), goes_to: (usize, f64, f64)) -> Door {
		Door {
            pos, goes_to
        }
	}
}

impl TObject for Door {
    fn pos(&self) -> (f64, f64) {
        self.pos
    }

    fn update(&self) {

    }

    fn interact(&self, renderer: &mut Renderer, _: &mut MenuManager) -> bool {
        renderer.play_fade();
        false
	}
}