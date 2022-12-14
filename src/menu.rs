use sdl2::{render::Canvas, video::Window};

pub mod main_menu;
pub mod textbox;

use crate::texture_manager::{TextureManager};
use crate::font_manager::{FontManager};

use self::main_menu::MainMenu;

#[derive(PartialEq, Debug)]
pub enum Action {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    ACCEPT,
    REJECT,
    _START
}

pub trait MenuItem {
    fn render(&mut self, canvas: &mut Canvas<Window>, textures: &mut TextureManager, font_man: &FontManager);
    fn update(&mut self, action: Action) -> bool; // returns true if menu should close after interaction
}

pub struct MenuManager {
    menus: Vec<Box<dyn MenuItem>>, // this is a stack
}

impl MenuManager {
    pub fn new() -> MenuManager {
        MenuManager {
            menus: vec![Box::new(MainMenu::new())],
        }
    }

    pub fn open_menu(&mut self, next_menu: Box<dyn MenuItem>) {
        self.menus.push(next_menu);
    }

    pub fn close_menu(&mut self) {
        self.menus.pop();
    }

    pub fn is_open(&self) -> bool {
        !self.menus.is_empty()
    }

    pub fn interact(&mut self, action: Action) {
        if self.is_open() {
            let curr_menu = self
                .menus
                .last_mut()
                .expect("Tried to change menu with no menus open");
            let should_close = curr_menu.update(action);
            if should_close {
                self.close_menu();
            }
        }
    }

    pub fn render(&mut self, canvas: &mut Canvas<Window>, texture_manager: &mut TextureManager, font_man: &FontManager) {
        for menu_item in self.menus.iter_mut() {
            menu_item.render(canvas, texture_manager, font_man);
        }
    }
}
