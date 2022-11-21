use sdl2::{render::Canvas, video::Window};

pub mod main_menu;
pub mod textbox;
use crate::render::{Textures, Fonts};

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
    fn render(&self, canvas: &mut Canvas<Window>, textures: &mut Textures, fonts: &Fonts);
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

    pub fn render(&mut self, canvas: &mut Canvas<Window>, textures: &mut Textures, fonts: &Fonts) {
        for menu_item in self.menus.iter() {
            menu_item.render(canvas, textures, fonts);
        }
    }
}
