use crate::font_manager::FontManager;

use super::menu_events::{MenuCommand, MenuInput};
use super::MenuItem;

pub struct Textbox {
    pub text_v: Vec<String>,
}

impl Textbox {
    pub fn new(text_in: &str, font_man: &FontManager, width_pixels: u32) -> Textbox {
        let mut text_v = font_man.break_string(&text_in, width_pixels);
        if (text_v.len() % 2) == 1 {
            text_v.push(" ".to_string());
        }

        Textbox { text_v }
    }
}

impl MenuItem for Textbox {
    fn update(&mut self, action: MenuInput) -> Option<MenuCommand> {
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
    fn advance_text(&mut self) -> Option<MenuCommand> {
        self.text_v.drain(0..2);
        match self.text_v.len() {
            0 => Some(MenuCommand::Close),
            _ => None,
        }
    }
}
