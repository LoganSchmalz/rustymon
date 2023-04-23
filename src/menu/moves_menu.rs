/****************************************************/
// Created by: Logan Schmalz
// Description: Menu logic for move selection that occurs in battle
/****************************************************/
use hecs::World;
use crate::components::stray::Move;
use crate::gamestate::event::Event;

use super::{
    menu_events::{MenuCommand, MenuInput},
    MenuItem,
};

pub struct MovesMenu {
    pub moves: [Option<Move>; 4],
    pub selected: usize,
}

impl MovesMenu {
    pub fn new(moves: [Option<Move>; 4]) -> MovesMenu {
        //let moves = [Some(Move::wave()), Some(Move::peck()), Some(Move::slice()), Some(Move::screech())]; //currently hardcoding the moves, will change later
        MovesMenu { moves, selected: 0 } //selected will indicate the selected move as 0 (top left), 1 (top right), 2 (bottom right), or 3 (bottom left)
    }
}

impl MenuItem for MovesMenu {
    fn update(&mut self, action: MenuInput, world: &mut World, events: &mut Vec<Event>) -> Option<MenuCommand> {
        let length = 4; //currently hardcoding length (the number of possible selections) as 4, may need to change to allow for less than 4 moves
        match action{ //check the user input to decide whether to activate a move or to scroll through moves
            MenuInput::Up => { //if user activates up input
                if let Some(mv) = &self.moves[((self.selected as i8 + 2) % 4) as usize] { //making sure there's an actual move at the index
                    self.selected = ((self.selected as i8 + 2) % 4) as usize;
                }
            }
            MenuInput::Right => { //if user activates right input
                if let Some(mv) = &self.moves[((self.selected as i8 + 1) % 4) as usize] { //making sure there's an actual move at the index
                    self.selected = ((self.selected as i8 + 1) % 4) as usize;
                }
            }
            MenuInput::Down => { //if user activates down input
                if let Some(mv) = &self.moves[((self.selected as i8 - 2 + 4) % 4) as usize] { //making sure there's an actual move at the index
                    self.selected = ((self.selected as i8 - 2 + 4) % 4) as usize;
                }
            }
            MenuInput::Left => { //if user activates left input
                if let Some(mv) = &self.moves[((self.selected as i8 - 1 + 4) % 4) as usize] { //making sure there's an actual move at the index
                    self.selected = ((self.selected as i8 - 1 + 4) % 4) as usize;
                }
            }
            MenuInput::Accept => { //if user activates accept input
                //activate that move
                if let Some(selection) = &self.moves[self.selected] {
                    events.push(Event::BattleAttack(selection.clone()));
                }
            }
            _ => {}
        }
        None
    }
}