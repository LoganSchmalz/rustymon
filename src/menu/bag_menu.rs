use crate::bag::ItemList;

use super::{
    menu_events::{MenuCommand, MenuInput},
    MenuItem,
};

#[derive(Default)]
pub struct BagMenu {
    pub items: ItemList,
    pub selected: usize,
}

impl BagMenu {
    pub fn new(items: ItemList) -> BagMenu {
        BagMenu { items, selected: 0 }
    }
}

impl MenuItem for BagMenu {
    fn update(&mut self, action: MenuInput) -> Option<MenuCommand> {
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
            MenuInput::Accept => {}
            MenuInput::Reject => return Some(MenuCommand::Close),
            _ => {}
        }
        None
    }
}
