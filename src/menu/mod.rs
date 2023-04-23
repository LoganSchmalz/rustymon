pub mod bag_menu;
pub mod main_menu;
pub mod menu_events;
pub mod moves_menu;
pub mod pause_menu;
pub mod textbox;

use hecs::World;

use crate::font_manager::FontManager;
use crate::gamestate::event::Event;

use self::bag_menu::BagMenu;
use self::main_menu::MainMenu;
use self::menu_events::{MenuCommand, MenuInput};
use self::moves_menu::MovesMenu;
use self::pause_menu::PauseMenu;
use self::textbox::Textbox;

#[enum_delegate::register]
pub trait MenuItem {
    fn update(
        &mut self,
        action: MenuInput,
        world: &mut World,
        events: &mut Vec<Event>,
    ) -> Option<MenuCommand>;
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
    pub menu_queue: Vec<Menu>,
}

impl Default for MenuManager { 
    fn default() -> Self {
        Self {
            //menus: vec![Menu::MainMenu(MainMenu::new())],
            menus: vec![],
            menu_queue: vec![],
        }
    }
}

impl MenuManager {
    pub fn new() -> Self {
        Self {
            //menus: vec![Menu::MainMenu(MainMenu::new())],
            menus: vec![],
            menu_queue: vec![],
        }
    }

    //this function handles the logic for opening a new menu on the stack
    pub fn open_menu(&mut self, next_menu: Menu) {
        if self.menus.len() == 0 {
            //if no menu is open
            self.menus.push(next_menu); //open menu
        } else {
            //if a menu is open
            if !matches!(next_menu, Menu::Textbox(_)) {
                //if the current menu is a textbox
                self.menu_queue.push(next_menu); //add next menu to queue, do not open yet
            }
        }
    }

    //this function handles the logic for closing a menu on the stack
    pub fn close_menu(&mut self) -> bool {
        self.menus.pop();
        if self.menu_queue.len() > 0 {
            //if there are menus in the queue
            if let Some(m) = self.menu_queue.pop() {
                //remove first queued menu from queue
                self.open_menu(m); //open first queued menu
            }
        }
        false
    }

    //checks if a menu is open
    pub fn is_open(&self) -> bool {
        !self.menus.is_empty()
    }

    //processes menucommands that tell the manager to open and close menus
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

    //handles input and sends it to the correct menu
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
