use hecs::World;

use crate::{
    close_menu,
    components::{bag::Bag, Player},
    open_menu,
};

use super::{
    bag_menu::BagMenu,
    menu_events::{MenuCommand, MenuInput},
    MenuItem, MenuManager,
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
    fn update(
        &mut self,
        action: MenuInput,
        world: &mut World,
    ) -> Option<Box<dyn Fn(&mut MenuManager)>> {
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
                        return Some(open_menu!(BagMenu::new(entity)));
                    }
                }
                "Close" => return Some(close_menu!()),
                _ => {}
            },
            MenuInput::Reject => return Some(close_menu!()),
            _ => {}
        }
        None
    }
}
