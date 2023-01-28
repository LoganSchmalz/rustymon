use sdl2::{render::Canvas, video::Window};

pub mod should_close;
pub mod main_menu;
pub mod textbox;
pub mod pause_menu;

use crate::font_manager::FontManager;
use crate::texture_manager::TextureManager;

use self::main_menu::MainMenu;
use self::should_close::ShouldClose;
use self::textbox::Textbox;
use self::pause_menu::PauseMenu;

#[derive(PartialEq, Debug)]
pub enum Action {
    Up,
    Down,
    Left,
    Right,
    Accept,
    Reject,
    _Start,
}

#[enum_delegate::register]
pub trait MenuItem {
    fn render(
        &mut self,
        canvas: &mut Canvas<Window>,
        textures: &mut TextureManager,
        font_man: &FontManager,
    );
    fn update(&mut self, action: Action) -> ShouldClose; // returns true if menu should close after interaction
}

#[enum_delegate::implement(MenuItem)]
pub enum Menu {
    MainMenu(MainMenu),
    Textbox(Textbox),
    PauseMenu(PauseMenu<'static>)
}

pub struct MenuManager {
    menus: Vec<Menu>, // this is a stack
    pub paused: bool,
}

impl MenuManager {
    pub fn new() -> MenuManager {
        MenuManager {
            menus: vec![Menu::MainMenu(MainMenu::new())],
            paused: true,
        }
    }

    pub fn open_menu(&mut self, next_menu: Menu) {
        self.menus.push(next_menu);
    }

    pub fn close_menu(&mut self) {
        let Some(menu) = self.menus.pop() else { return; };
        match menu {
            Menu::MainMenu(_) => {
                self.paused = false;
            }
            _ => {}
        }
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
            if curr_menu.update(action) == ShouldClose(true) {
                self.close_menu();
            }
        }
    }

    pub fn render(
        &mut self,
        canvas: &mut Canvas<Window>,
        texture_manager: &mut TextureManager,
        font_man: &FontManager,
    ) {
        for menu_item in self.menus.iter_mut() {
            menu_item.render(canvas, texture_manager, font_man);
        }
    }
}
