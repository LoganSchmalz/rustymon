use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

use crate::{
    bag::Bag,
    font_manager::FontManager,
    render::{PIXELS_X, PIXELS_Y},
    texture_manager::TextureManager,
};

use super::{menu_events::{MenuEvent, MenuInput}, MenuItem};

pub struct BagMenu<'a> {
    bag: &'a Bag,
    selected: usize,
}

impl BagMenu<'_> {
    pub fn new(bag: &Bag) -> BagMenu {
        BagMenu { bag, selected: 0 }
    }
}

impl MenuItem for BagMenu<'_> {
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

        let mut text_quad = Rect::new(140, 10, 0, 0);

        for (idx, (item, amount)) in self.bag.items.iter().enumerate() {
			let item_str = item.to_string();

            let item_surface = font_man.fonts.press_start_2p.render(&item_str);
            let item_surface = if idx == self.selected {
                item_surface.blended(Color::RGB(0, 183, 235))
            } else {
                item_surface.blended(Color::RGB(40, 40, 40))
            };
            let item_surface = item_surface.unwrap();

			let amount_str = amount.to_string();
			let amount_surface = font_man.fonts.press_start_2p.render(&amount_str);
			let amount_surface = if idx == self.selected {
                amount_surface.blended(Color::RGB(0, 183, 235))
            } else {
                amount_surface.blended(Color::RGB(40, 40, 40))
            };
			let amount_surface = amount_surface.unwrap();

            text_quad.set_width(item_surface.width());
            text_quad.set_height(item_surface.height());

			let amount_quad = Rect::new(PIXELS_X as i32 - 10 - amount_surface.width() as i32, text_quad.y, amount_surface.width(), amount_surface.height());

            let creator = canvas.texture_creator();
            let item_texture = creator.create_texture_from_surface(&item_surface).unwrap();
			let amount_texture = creator.create_texture_from_surface(&amount_surface).unwrap();

            canvas.copy(&item_texture, None, text_quad).unwrap();
			canvas.copy(&amount_texture, None, amount_quad).unwrap();

            text_quad.set_y(text_quad.y + item_surface.height() as i32 + 4);
        }
    }

    fn update(&mut self, action: MenuInput, _bag: &Bag) -> Option<MenuEvent> {
        match action {
            MenuInput::Down => {
                self.selected = if self.selected < self.bag.items.len() - 1 {
                    self.selected + 1
                } else {
                    0
                }
            }
            MenuInput::Up => {
                self.selected = if self.selected > 0 {
                    self.selected - 1
                } else {
                    self.bag.items.len() - 1
                }
            }
            MenuInput::Accept => {},
            MenuInput::Reject => return Some(MenuEvent::Close),
            _ => {}
        }
        None
    }
}
