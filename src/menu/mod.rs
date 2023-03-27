pub mod bag_menu;
pub mod main_menu;
pub mod menu_events;
pub mod pause_menu;
pub mod textbox;
pub mod moves_menu;

use hecs::World;

use crate::font_manager::FontManager;
use crate::gamestate::event::Event;

use self::bag_menu::BagMenu;
use self::main_menu::MainMenu;
use self::menu_events::{MenuCommand, MenuInput};
use self::pause_menu::PauseMenu;
use self::textbox::Textbox;
use self::moves_menu::MovesMenu;

#[enum_delegate::register]
pub trait MenuItem {
    fn update(&mut self, action: MenuInput, world: &mut World, events: &mut Vec<Event>) -> Option<MenuCommand>;
}

#[enum_delegate::implement(MenuItem)]
pub enum Menu {
    MainMenu(MainMenu),
    Textbox(Textbox),
    PauseMenu(PauseMenu),
    BagMenu(BagMenu),
    MovesMenu(MovesMenu),
}

pub struct MenuManager {
    pub menus: Vec<Menu>, // this is a stack
}

impl MenuManager {
    pub fn new() -> MenuManager {
        MenuManager {
            //menus: vec![Menu::MainMenu(MainMenu::new())],
            menus: vec![],
        }
    }

    pub fn open_menu(&mut self, next_menu: Menu) {
        self.menus.push(next_menu);
    }

    pub fn close_menu(&mut self) -> bool {
        self.menus.pop();
        false
    }

    pub fn is_open(&self) -> bool {
        !self.menus.is_empty()
    }

    fn process_command(
        &mut self,
        command: MenuCommand,
        _world: &mut World,
        font_manager: &FontManager,
    ) -> bool {
        match command {
            MenuCommand::OpenStrays => {}
            MenuCommand::OpenSave => {}
            MenuCommand::OpenBag(entity) => self.open_menu(BagMenu::new(entity).into()),
            MenuCommand::Close => {
                return self.close_menu();
            }
            MenuCommand::OpenTextbox(str) => {
                self.open_menu(Textbox::new(&str, font_manager).into())
            }
            MenuCommand::OpenPauseMenu => self.open_menu(PauseMenu::new().into()),
        }
        false
    }

    pub fn interact(
        &mut self,
        action: MenuInput,
        world: &mut World,
        font_manager: &FontManager,
        events: &mut Vec<Event>,
    ) -> bool {
        if self.is_open() {
            let curr_menu = self
                .menus
                .last_mut()
                .expect("Tried to change menu with no menus open");

            if let Some(command) = curr_menu.update(action, world, events) {
                self.process_command(command, world, font_manager);
            }
        } else if action == MenuInput::Start {
            self.open_menu(Menu::PauseMenu(PauseMenu::new()));
        }
        false
    }
}
