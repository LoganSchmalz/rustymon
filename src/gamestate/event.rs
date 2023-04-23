/****************************************************/
// Created by: Logan Schmalz
// Description: Enum of possible events, which are handled in updates.rs
/****************************************************/
use hecs::Entity;

use crate::vec2::Vec2;
use crate::gamestate::stray::Move;

pub enum Event {
    PlayerMoved(Vec2),
    NpcMoved(Entity),
    BattleAttack(Move),
    AttackStray(usize),
    TransitionFull,
}
