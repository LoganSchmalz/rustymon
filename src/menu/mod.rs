pub mod bag_menu;
pub mod macros;
pub mod main_menu;
pub mod menu_events;
pub mod pause_menu;
pub mod textbox;

use hecs::World;

use crate::gamestate::State;

use self::bag_menu::BagMenu;
use self::main_menu::MainMenu;
use self::menu_events::MenuInput;
use self::pause_menu::PauseMenu;
use self::textbox::Textbox;

#[enum_delegate::register]
pub trait MenuItem {
    fn update(&mut self, action: MenuInput, world: &mut World)
        -> Option<Box<dyn Fn(&mut MenuManager)>>;
}

#[enum_delegate::implement(MenuItem)]
pub enum Menu {
    MainMenu(MainMenu),
    Textbox(Textbox),
    PauseMenu(PauseMenu),
    BagMenu(BagMenu),
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

    pub fn interact(&mut self, action: MenuInput, world: &mut World) -> bool {
        if self.is_open() {
            let curr_menu = self
                .menus
                .last_mut()
                .expect("Tried to change menu with no menus open");
            /*match curr_menu.update(action) {
                Some(MenuCommand::OpenStrays) => {}
                Some(MenuCommand::OpenSave) => {}
                Some(MenuCommand::OpenBag) => self.open_menu(Menu::BagMenu(BagMenu::new(items))),
                Some(MenuCommand::Close) => {
                    return self.close_menu();
                }
                _ => {}
            }*/
            if let Some(command) = curr_menu.update(action, world) {
                command(self);
            }
        } else if action == MenuInput::Start {
            self.open_menu(Menu::PauseMenu(PauseMenu::new()));
        }
        false
    }
}
