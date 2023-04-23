/****************************************************/
// Created by: Logan Schmalz
// Description: Menu logic for bag menu and items within it
/****************************************************/
use hecs::{Entity, World};

use crate::components::bag::Bag;
use crate::gamestate::event::Event;

use super::{
    menu_events::{MenuCommand, MenuInput},
    MenuItem,
};

pub struct BagMenu {
    pub selected: usize,
    pub entity: Entity,
}

impl BagMenu {
    pub fn new(entity: Entity) -> BagMenu {
        BagMenu {
            selected: 0,
            entity,
        }
    }
}

impl MenuItem for BagMenu {
    fn update(&mut self, action: MenuInput, world: &mut World, events: &mut Vec<Event>) -> Option<MenuCommand> {
        let length = if let Ok(bag) = world.query_one_mut::<&Bag>(self.entity) {
            bag.items.len()
        } else {
            0
        };

        match action {
            MenuInput::Down => {
                self.selected = if self.selected < length - 1 {
                    self.selected + 1
                } else {
                    0
                }
            }
            MenuInput::Up => {
                self.selected = if self.selected > 0 {
                    self.selected - 1
                } else {
                    length - 1
                }
            }
            MenuInput::Accept => {}
            MenuInput::Reject => {
                return Some(MenuCommand::Close);
            }
            _ => {}
        }
        None
    }
}
