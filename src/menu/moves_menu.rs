use hecs::World;
use crate::components::stray::Move;

use super::{
    menu_events::{MenuCommand, MenuInput},
    MenuItem,
};

pub struct MovesMenu {
    pub moves: [Option<Move>; 4],
    pub selected: usize,
}

impl MovesMenu {
    pub fn new() -> MovesMenu {
        let moves = [Some(Move::wave()), Some(Move::peck()), None, None]; //currently hardcoding the moves, will change later
        MovesMenu { moves, selected: 0 } //selected will indicate the selected move as 0 (top left), 1 (top right), 2 (bottom right), or 3 (bottom left)
    }
}

impl MenuItem for MovesMenu {
    fn update(&mut self, action: MenuInput, world: &mut World) -> Option<MenuCommand> {
        let length = 4; //currently hardcoding length (the number of possible selections) as 4, may need to change to allow for less than 4 moves
        match action{ //check the user input to decide whether to activate a move or to scroll through moves
            MenuInput::Up => { //if user activates up input
                self.selected = ((self.selected as i8 + 2) % 4) as usize;
                println!("MOVED UP IN MENU");
            }
            MenuInput::Right => { //if user activates right input
                self.selected = ((self.selected as i8 + 1) % 4) as usize;
                println!("MOVED RIGHT IN MENU");
            }
            MenuInput::Down => { //if user activates down input
                self.selected = ((self.selected as i8 - 2) as i8 % 4) as usize;
                println!("MOVED DOWN IN MENU");
            }
            MenuInput::Left => { //if user activates left input
                self.selected = ((self.selected as i8 - 1) % 4) as usize;
                println!("MOVED LEFT IN MENU");
            }
            MenuInput::Accept => { //if user activates accept input
                //activate that move
            }
            _ => {}
        }
        None
    }
}