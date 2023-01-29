use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

use crate::bag::Bag;
use crate::font_manager::FontManager;
use crate::render::{PIXELS_X, PIXELS_Y};
use crate::texture_manager::TextureManager;

use super::menu_events::{MenuEvent, MenuInput};
use super::MenuItem;

pub struct Textbox {
    text_in: String,
    text_v: Vec<String>,
    init: bool,
}

impl Textbox {
    pub fn new(text_in: String) -> Textbox {
        Textbox {
            text_in,
            text_v: vec![],
            init: false,
        }
    }
}

impl MenuItem for Textbox {
    fn render(
        &mut self,
        canvas: &mut Canvas<Window>,
        texture_manager: &mut TextureManager,
        font_man: &FontManager,
    ) {
        // create new quad over the textbox texture (which is 41 px tall)
        let box_quad = Rect::new(0, (PIXELS_Y - 41) as i32, PIXELS_X, 41 as u32);
        if !self.init {
            self.init_vec(true, font_man.break_string(&self.text_in, PIXELS_X as u32));
        }

        let surface_top = font_man
            .fonts
            .press_start_2p
            .render(self.text_v[0].as_str())
            .blended(Color::RGB(40, 40, 40))
            .unwrap();

        let surface_bot = font_man
            .fonts
            .press_start_2p
            .render(self.text_v[1].as_str())
            .blended(Color::RGB(40, 40, 40))
            .unwrap();

        let text_quad_top = Rect::new(
            10,
            (PIXELS_Y - 41) as i32 + 10,
            surface_top.width(),
            surface_top.height(),
        );

        let text_quad_bot = Rect::new(
            10,
            (PIXELS_Y - 41) as i32 + 10 + surface_top.height() as i32 + 4,
            surface_bot.width(),
            surface_bot.height(),
        );

        let creator = canvas.texture_creator();
        let texture_top = creator.create_texture_from_surface(&surface_top).unwrap();
        let texture_bot = creator.create_texture_from_surface(&surface_bot).unwrap();
        canvas
            .copy(&texture_manager.textures.text_box, None, box_quad)
            .unwrap();
        canvas.copy(&texture_top, None, text_quad_top).unwrap();
        canvas.copy(&texture_bot, None, text_quad_bot).unwrap();
    }

    fn update(&mut self, action: MenuInput) -> Option<MenuEvent> {
        match action {
            MenuInput::Accept | MenuInput::Reject => {
                return self.advance_text();
            }
            _ => {}
        }
        None
    }
}

impl Textbox {
    fn advance_text(&mut self) -> Option<MenuEvent> {
        self.text_v.drain(0..2);
        match self.text_v.len() {
            0 => Some(MenuEvent::Close),
            _ => None,
        }
    }

    fn init_vec(&mut self, b: bool, v: Vec<String>) {
        self.init = b;
        self.text_v = v;
        if (self.text_v.len() % 2) == 1 {
            self.text_v.push(" ".to_string());
        }
    }
}
