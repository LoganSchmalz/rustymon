/****************************************************/
// Created by: Logan Schmalz
// Description: Menu logic for the main menu that appears on the starting screen
/****************************************************/
use super::menu_events::MenuCommand;
use super::{MenuInput, MenuItem};

use crate::gamestate::event::Event;

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
    //the menu update logic
    fn update(&mut self, action: MenuInput, _: &mut World, events: &mut Vec<Event>) -> Option<MenuCommand> {
        match action {
            MenuInput::Accept => {
                if self.curr_button == Start {
                    return Some(MenuCommand::Close);
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
                    Start => Settings, 
                    Load => Settings,  
                    Settings => Start, 
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
