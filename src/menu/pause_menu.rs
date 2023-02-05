use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

use crate::{
    font_manager::FontManager,
    render::{PIXELS_X, PIXELS_Y},
    texture_manager::TextureManager,
};

use super::{
    menu_events::{MenuEvent, MenuInput},
    MenuItem,
};

pub struct PauseMenu {
    items: Vec<String>,
    selected: usize,
}

impl PauseMenu {
    pub fn new() -> PauseMenu {
        let items = vec![
            String::from("Strays"),
            String::from("Bag"),
            String::from("Save"),
            String::from("Options"),
            String::from("Close"),
        ];
        PauseMenu { items, selected: 0 }
    }
}

impl MenuItem for PauseMenu {
    fn render(
        &mut self,
        canvas: &mut Canvas<Window>,
        texture_manager: &mut TextureManager,
        font_man: &FontManager,
    ) {
        // create new quad over the textbox texture (which is 41 px tall)
        //let box_quad = Rect::new(0, (PIXELS_Y - 41) as i32, PIXELS_X, 41 as u32);
        let box_quad = Rect::new(0, 0, PIXELS_X, PIXELS_Y);

        canvas
            .copy(&texture_manager.textures.pause_menu, None, box_quad)
            .unwrap();

        let mut text_quad = Rect::new(180, 10, 0, 0);

        for (idx, item) in self.items.iter().enumerate() {
            let surface = font_man.fonts.press_start_2p.render(item);
            let surface = if idx == self.selected {
                surface.blended(Color::RGB(0, 183, 235))
            } else {
                surface.blended(Color::RGB(40, 40, 40))
            };
            let surface = surface.unwrap();

            text_quad.set_width(surface.width());
            text_quad.set_height(surface.height());

            let creator = canvas.texture_creator();
            let texture = creator.create_texture_from_surface(&surface).unwrap();

            canvas.copy(&texture, None, text_quad).unwrap();

            text_quad.set_y(text_quad.y + surface.height() as i32 + 4);
        }
    }

    fn update(&mut self, action: MenuInput) -> Option<MenuEvent> {
        match action {
            MenuInput::Down => {
                self.selected = if self.selected < self.items.len() - 1 {
                    self.selected + 1
                } else {
                    0
                }
            }
            MenuInput::Up => {
                self.selected = if self.selected > 0 {
                    self.selected - 1
                } else {
                    self.items.len() - 1
                }
            }
            MenuInput::Accept => match self.items[self.selected].as_str() {
                "Bag" => return Some(MenuEvent::OpenBag),
                "Exit" => {
                    if self.items[self.selected] == "Exit" {
                        return Some(MenuEvent::Close);
                    } else {
                    }
                }
                _ => {}
            },
            MenuInput::Reject => return Some(MenuEvent::Close),
            _ => {}
        }
        None
    }
}
