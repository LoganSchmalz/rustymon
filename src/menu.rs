use sdl2::{render::Canvas, video::Window};

pub mod bag_menu;
pub mod main_menu;
pub mod menu_events;
pub mod pause_menu;
pub mod textbox;

use crate::bag::ItemList;
use crate::font_manager::FontManager;
use crate::texture_manager::TextureManager;

use self::bag_menu::BagMenu;
use self::main_menu::MainMenu;
use self::menu_events::{MenuEvent, MenuInput};
use self::pause_menu::PauseMenu;
use self::textbox::Textbox;

#[enum_delegate::register]
pub trait MenuItem {
    fn render(
        &mut self,
        canvas: &mut Canvas<Window>,
        textures: &mut TextureManager,
        font_man: &FontManager,
    );
    fn update(&mut self, action: MenuInput) -> Option<MenuEvent>;
}

#[enum_delegate::implement(MenuItem)]
pub enum Menu {
    MainMenu(MainMenu),
    Textbox(Textbox),
    PauseMenu(PauseMenu),
    BagMenu(BagMenu),
}
pub struct MenuManager {
    menus: Vec<Menu>, // this is a stack
    pub paused: bool,
}

impl<'a> MenuManager {
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

    pub fn interact(&mut self, action: MenuInput, items: ItemList) {
        if self.is_open() {
            let curr_menu = self
                .menus
                .last_mut()
                .expect("Tried to change menu with no menus open");
            match curr_menu.update(action) {
                Some(MenuEvent::OpenStrays) => {}
                Some(MenuEvent::OpenSave) => {}
                Some(MenuEvent::OpenBag) => self.open_menu(Menu::BagMenu(BagMenu::new(items))),
                Some(MenuEvent::Close) => self.close_menu(),
                _ => {}
            }
        } else if action == MenuInput::Start {
            self.open_menu(Menu::PauseMenu(PauseMenu::new()));
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
