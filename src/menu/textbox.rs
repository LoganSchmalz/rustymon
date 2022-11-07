use sdl2::{render::Canvas, video::Window};

use crate::render::Textures;

use super::{MenuItem, Action, MenuManager};

pub struct Textbox {
}

impl Textbox {
	pub fn new() -> Textbox {
		Textbox {
		}
	}
}

impl MenuItem for Textbox {
	fn render(&self, canvas: &mut Canvas<Window>, textures: &mut Textures,) {
	}

	fn update(&mut self, action: Action) -> bool {
		true
	}
}