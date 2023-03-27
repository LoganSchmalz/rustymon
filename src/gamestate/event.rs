use hecs::Entity;

use crate::vec2::Vec2;
use crate::gamestate::stray::Move;

pub enum Event {
    PlayerMoved(Vec2),
    NpcMoved(Entity),
    BattleAttack(Move),
    TransitionFull,
}
