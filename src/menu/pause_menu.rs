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
            MenuInput::Accept => match self.items[self.selected].as_str() {
                "Bag" => return Some(MenuCommand::OpenBag),
                "Close" => {
                    if self.items[self.selected] == "Close" {
                        return Some(MenuCommand::Close);
                    } else {
                    }
                }
                _ => {}
            },
            MenuInput::Reject => return Some(MenuCommand::Close),
            _ => {}
        }
        None
    }
}
