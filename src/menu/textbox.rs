use hecs::World;

use crate::close_menu;
use crate::font_manager::FontManager;

use super::menu_events::{MenuCommand, MenuInput};
use super::{MenuItem, MenuManager};

pub struct Textbox {
    pub text_v: Vec<String>,
}

impl Textbox {
    pub fn new(text_in: &str, font_man: &FontManager, width_pixels: u32) -> Textbox {
        let mut text_v = font_man.break_string(text_in, width_pixels);
        if (text_v.len() % 2) == 1 {
            text_v.push(" ".to_string());
        }

        Textbox { text_v }
    }
}

impl MenuItem for Textbox {
    fn update(&mut self, action: MenuInput, _: &mut World) -> Option<Box<dyn Fn(&mut MenuManager)>> {
        match action {
            MenuInput::Accept | MenuInput::Reject => {
                if self.advance_text() {
                    return Some(close_menu!());
                }
            }
            _ => {}
        }
        None
    }
}

impl Textbox {
    fn advance_text(&mut self) -> bool {
        self.text_v.drain(0..2);
        self.text_v.is_empty()
    }
}
