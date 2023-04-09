use std::collections::VecDeque;

use crate::components::stray::{Move, Stray};

#[derive(Default, Clone)]
pub struct Battle {
    pub player_strays: [Option<Stray>; 4],
    pub opponent_strays: [Option<Stray>; 4],
    pub selected_move: Option<Move>,
    pub battle_state: BattleState,
    pub turn_order: VecDeque<usize>,
}

impl Battle {
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
        }
    }
}

#[derive(Clone, Default)]
pub enum BattleState {
    #[default]
    SelectingMove,
    SelectingOpponentStray,
    SelectingFriendlyStray,
}