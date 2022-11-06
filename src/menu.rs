use sdl2::{render::Canvas, video::Window};

pub mod main_menu;
pub mod textbox;
use crate::render::Textures;

use self::main_menu::MainMenu;

#[derive(PartialEq, Debug)]
pub enum Action {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    SELECT,
    REJECT,
}

pub trait Menu_Item {
    fn render(&self, canvas: &mut Canvas<Window>, textures: &mut Textures);
    fn update(&mut self, action: Action);
}

pub struct MenuManager {
    menus: Vec<Box<dyn Menu_Item>>, // this is a stack
}

impl MenuManager {
    pub fn new() -> MenuManager {
        MenuManager {
            menus: vec![Box::new(MainMenu::new())],
        }
    }

    pub fn open_menu(&mut self, next_menu: Box<dyn Menu_Item>) {
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
            if action == Action::REJECT {
                self.close_menu();
            } else {
                self.menus
                    .last_mut()
                    .expect("Tried to change menu with no menus open")
                    .update(action);
            }
        }
    }

    pub fn render(&mut self, canvas: &mut Canvas<Window>, textures: &mut Textures) {
        for menu_item in self.menus.iter() {
            menu_item.render(canvas, textures);
        }
    }
}
