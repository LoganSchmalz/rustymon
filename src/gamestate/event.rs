use hecs::Entity;

use crate::vec2::Vec2;

pub enum Event {
    PlayerMoved(Vec2),
    NpcMoved(Entity),
}
