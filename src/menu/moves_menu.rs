use hecs::World;

use super::{
    menu_events::{MenuCommand, MenuInput},
    MenuItem,
};

pub struct MovesMenu {
    pub selected: usize,
}

impl MovesMenu {
    pub fn new() -> MovesMenu {
        MovesMenu { selected: 0 } //selected will indicate the selected move as 0 (top left), 1 (top right), 2 (bottom right), or 3 (bottom left)
    }
}

impl MenuItem for MovesMenu {
    fn update(&mut self, action: MenuInput, world: &mut World) -> Option<MenuCommand> {
        let length = 4; //currently hardcoding length (the number of possible selections) as 4, may need to change to allow for less than 4 moves
        match action{ //check the user input to decide whether to activate a move or to scroll through moves
            MenuInput::Up => { //if user activates up input
                self.selected = (self.selected + 2) % 4;
                println!("MOVED IN MENU");
            }
            MenuInput::Right => { //if user activates right input
                self.selected = (self.selected + 1) % 4
            }
            MenuInput::Down => { //if user activates down input
                self.selected = (self.selected - 2) % 4
            }
            MenuInput::Left => { //if user activates left input
                self.selected = (self.selected - 1) % 4
            }
            MenuInput::Accept => { //if user activates accept input
                //activate that move
            }
            _ => {}
        }
        None
    }
}