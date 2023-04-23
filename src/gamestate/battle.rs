/****************************************************/
// Created by: Logan Schmalz
// Description: Battle data structure for storing all battle information
// including strays, state, moves, turn order, and necessary menus
/****************************************************/
use std::collections::VecDeque;

use crate::{
    components::stray::{Move, Stray},
    menu::{Menu, MenuManager},
};

#[derive(Clone, Default, Debug)]
pub enum BattleState {
    #[default]
    SelectingMove,
    SelectingOpponentStray,
    SelectingFriendlyStray,
}

#[derive(Default)]
pub struct Battle {
    pub player_strays: [Option<Stray>; 4],
    pub opponent_strays: [Option<Stray>; 4],
    pub selected_move: Option<Move>,
    pub selected_stray: Option<usize>,
    pub battle_state: BattleState,
    pub turn_order: VecDeque<usize>,
    pub menus: MenuManager,
}

impl Battle {
    //creates a new Battle
    //takes in player_strays (a list of 4 optional Strays that will be on the player's team), and opponent_strays (a list of 4 optional Strays that will be against the player)
    pub fn new(player_strays: [Option<Stray>; 4], opponent_strays: [Option<Stray>; 4]) -> Battle {
        let mut turn_order = VecDeque::new();
        let mut strays: Vec<&Option<Stray>> = vec![];

        for stray in player_strays.iter().chain(opponent_strays.iter()) {
            strays.push(stray);
        }

        for (idx, stray) in player_strays.iter().enumerate() {
            if stray.is_some() {
                turn_order.push_back(idx);
            }
        }
        for (idx, stray) in opponent_strays.iter().enumerate() {
            if stray.is_some() {
                turn_order.push_back(idx + 4);
            }
        }

        //computes the turn order after combining all the strays into one list
        turn_order.make_contiguous().sort_by(|a, b| {
            strays[*b]
                .as_ref()
                .unwrap()
                .spd
                .cmp(&strays[*a].as_ref().unwrap().spd)
        });

        Battle {
            player_strays,
            opponent_strays,
            selected_move: None,
            battle_state: BattleState::SelectingMove,
            turn_order,
            selected_stray: None,
            menus: MenuManager::new(),
        }
    }

    //calculates the nearest stray on the left for the opponents when selecting a stray to attack
    //takes in an optional index which is the current index of the selected stray
    //returns an optional index of the leftward stray, or None of no stray is found
    pub fn get_left_opponent_stray(&mut self, index: Option<usize>) -> Option<usize> {
        //try to get next left stray
        if let Some(index) = index {
            for i in (0..index).rev() {
                if self.opponent_strays[i].is_some() {
                    return Some(i);
                };
            }
        }
        //try to get most left stray
        for i in 0..4 {
            if self.opponent_strays[i].is_some() {
                return Some(i);
            };
        }
        //default to no stray found
        None
    }

    //calculates the nearest stray on the left for the player when selecting a stray to attack
    //takes in an optional index which is the current index of the selected stray
    //returns an optional index of the leftward stray, or None of no stray is found
    pub fn get_left_player_stray(&self, index: Option<usize>) -> Option<usize> {
        //try to get next left stray
        if let Some(index) = index {
            for i in (0..index).rev() {
                if self.player_strays[i].is_some() {
                    return Some(i);
                };
            }
        }
        //try to get most left stray
        for i in 0..4 {
            if self.player_strays[i].is_some() {
                return Some(i);
            };
        }
        //default to no stray found
        None
    }

    //calculates the nearest stray on the right for the opponents when selecting a stray to attack
    //takes in an optional index which is the current index of the selected stray
    //returns an optional index of the rightward stray, or None of no stray is found
    pub fn get_right_opponent_stray(&self, index: Option<usize>) -> Option<usize> {
        //try to get next right stray
        if let Some(index) = index {
            for i in (index + 1)..4 {
                if self.opponent_strays[i].is_some() {
                    return Some(i);
                };
            }
        }
        //try to get most right stray
        for i in (0..4).rev() {
            if self.opponent_strays[i].is_some() {
                return Some(i);
            };
        }
        //default to no stray found
        None
    }

    //calculates the nearest stray on the right for the player when selecting a stray to attack
    //takes in an optional index which is the current index of the selected stray
    //returns an optional index of the rightward stray, or None of no stray is found
    pub fn get_right_player_stray(&self, index: Option<usize>) -> Option<usize> {
        //try to get next right stray
        if let Some(index) = index {
            for i in (index + 1)..4 {
                if self.player_strays[i].is_some() {
                    return Some(i);
                };
            }
        }
        //try to get most right stray
        for i in (0..4).rev() {
            if self.player_strays[i].is_some() {
                return Some(i);
            };
        }
        //default to no stray found
        None
    }
}
