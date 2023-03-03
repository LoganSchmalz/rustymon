use crate::close_menu;

use super::menu_events::MenuCommand;
use super::{MenuInput, MenuItem, MenuManager};

#[derive(PartialEq, Default)]
pub enum MainMenuButton {
    #[default]
    Start,
    Load,
    Settings,
}

#[derive(Default)]
pub struct MainMenu {
    pub curr_button: MainMenuButton,
}

impl MainMenu {
    pub fn new() -> MainMenu {
        MainMenu { curr_button: Start }
    }
}

use hecs::World;
use MainMenuButton::*;

impl MenuItem for MainMenu {
    fn update(&mut self, action: MenuInput, _: &mut World) -> Option<Box<dyn Fn(&mut MenuManager)>> {
        match action {
            MenuInput::Accept => {
                if self.curr_button == Start {
                    return Some(close_menu!());
                }
            }
            MenuInput::Left => {
                self.curr_button = match self.curr_button {
                    Start => Load,
                    Load => Start,
                    Settings => Load,
                }
            }
            MenuInput::Right => {
                self.curr_button = match self.curr_button {
                    Start => Settings, //BUTTONS[2] == Button::SettingsButton
                    Load => Settings,  //BUTTONS[2] == Button::SettingsButton
                    Settings => Start, //BUTTONS[0] == Button::StartButton
                }
            }
            MenuInput::Up => {
                self.curr_button = match self.curr_button {
                    Start => Settings,
                    Settings => Start,
                    Load => Start,
                }
            }
            MenuInput::Down => {
                self.curr_button = match self.curr_button {
                    Start => Load,
                    Settings => Start,
                    Load => Settings,
                }
            }
            _ => {}
        }
        None
    }
}
