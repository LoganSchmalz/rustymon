/****************************************************/
// Created by: Logan Schmalz
// Description: Menu logic for pause menu that occurs in game when the player presses the pause key
/****************************************************/
use hecs::World;

use crate::components::{bag::Bag, Player};
use crate::gamestate::event::Event;

use super::{
    menu_events::{MenuCommand, MenuInput},
    MenuItem,
};

#[derive(Default)]
pub struct PauseMenu {
    pub items: Vec<String>,
    pub selected: usize,
}

impl PauseMenu {
    pub fn new() -> PauseMenu {
        let items = vec![
            String::from("Strays"),
            String::from("Bag"),
            String::from("Save"),
            String::from("Options"),
            String::from("Close"),
        ];
        PauseMenu { items, selected: 0 }
    }
}

impl MenuItem for PauseMenu {
    fn update(&mut self, action: MenuInput, world: &mut World, events: &mut Vec<Event>) -> Option<MenuCommand> {
        match action {
            MenuInput::Down => {
                self.selected = if self.selected < self.items.len() - 1 {
                    self.selected + 1
                } else {
                    0
                }
            }
            MenuInput::Up => {
                self.selected = if self.selected > 0 {
                    self.selected - 1
                } else {
                    self.items.len() - 1
                }
            }
            MenuInput::Accept => match self.items[self.selected].as_str() {
                "Bag" => {
                    if let Some((entity, (_, _))) =
                        world.query_mut::<(&Player, &Bag)>().into_iter().next()
                    {
                        return Some(MenuCommand::OpenBag(entity));
                    }
                }
                "Close" => return Some(MenuCommand::Close),
                _ => {}
            },
            MenuInput::Reject => return Some(MenuCommand::Close),
            _ => {}
        }
        None
    }
}
