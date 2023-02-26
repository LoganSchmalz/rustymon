use super::menu_events::MenuCommand;
use super::{MenuInput, MenuItem};

#[derive(PartialEq, Default)]
pub enum MainMenuButton {
    #[default]
    StartButton,
    LoadButton,
    SettingsButton,
}

#[derive(Default)]
pub struct MainMenu {
    pub curr_button: MainMenuButton,
}

impl MainMenu {
    pub fn new() -> MainMenu {
        MainMenu {
            curr_button: StartButton,
        }
    }
}

use MainMenuButton::*;

impl MenuItem for MainMenu {
    fn update(&mut self, action: MenuInput) -> Option<MenuCommand> {
        match action {
            MenuInput::Accept => {
                if self.curr_button == StartButton {
                    return Some(MenuCommand::Close);
                }
            }
            MenuInput::Left => {
                self.curr_button = match self.curr_button {
                    StartButton => LoadButton,
                    LoadButton => StartButton,
                    SettingsButton => LoadButton,
                }
            }
            MenuInput::Right => {
                self.curr_button = match self.curr_button {
                    StartButton => SettingsButton, //BUTTONS[2] == Button::SettingsButton
                    LoadButton => SettingsButton,  //BUTTONS[2] == Button::SettingsButton
                    SettingsButton => StartButton, //BUTTONS[0] == Button::StartButton
                }
            }
            MenuInput::Up => {
                self.curr_button = match self.curr_button {
                    StartButton => SettingsButton,
                    SettingsButton => StartButton,
                    LoadButton => StartButton,
                }
            }
            MenuInput::Down => {
                self.curr_button = match self.curr_button {
                    StartButton => LoadButton,
                    SettingsButton => StartButton,
                    LoadButton => SettingsButton,
                }
            }
            _ => {}
        }
        None
    }
}
