use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

use crate::texture_manager::{TextureManager};
use crate::render::{Fonts, PIXELS_X, PIXELS_Y};

use super::{Action, MenuItem};

pub struct Textbox {
    text: String,
}

impl Textbox {
    pub fn new(text: String) -> Textbox {
        Textbox { text }
    }
}

impl MenuItem for Textbox {
    fn render(&self, canvas: &mut Canvas<Window>, texture_manager: &mut TextureManager, fonts: &Fonts) {
        let box_quad = Rect::new(0, (PIXELS_Y - 41) as i32, PIXELS_X, 41 as u32);

        let surface = fonts
            .press_start_2p
            .render(self.text.as_str())
            .blended(Color::RGB(179, 145, 133))
            .unwrap();

		let text_quad = Rect::new(
			10,
			(PIXELS_Y - 41) as i32 + 10,
			surface.width(),
			surface.height(),
		);
        let creator = canvas.texture_creator();
        let texture = creator.create_texture_from_surface(&surface).unwrap();
        canvas.copy(&texture_manager.textures.text_box, None, box_quad).unwrap();
        canvas.copy(&texture, None, text_quad).unwrap();
    }

    fn update(&mut self, action: Action) -> bool {
        match action {
            Action::ACCEPT | Action::REJECT => {
                return self.advance_text();
            }
            _ => {}
        }
        false
    }
}

impl Textbox {
    fn advance_text(&mut self) -> bool {
        //todo: use this for advancing displayed string on longer messages
        true
    }
}
