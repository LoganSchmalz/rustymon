use self::bag::Item;

use super::vec2::{Vec2, Direction};

pub mod animation;
pub mod bag;
pub mod sprite;

pub struct Player;

pub struct GroundItem {
    pub item: Item,
    pub amount: u32,
}

pub struct Npc {
    pub says: String,
}

#[derive(Debug)]
pub struct Position(pub Vec2);

pub struct Collision;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum MovingState {
    Idle,
    CenterTile,
    Moving(Direction),
}
pub struct MovingEntity {
    pub rotation: Direction,
    pub moving: MovingState,
    pub try_moving: MovingState,
    pub sprinting: bool,
    pub try_sprinting: bool,
    pub animation_time: f32,
    pub rotation_timer: f32,
}

impl Default for MovingEntity {
    fn default() -> Self {
        Self {
            rotation: Direction::Down,
            moving: MovingState::Idle,
            try_moving: MovingState::Idle,
            sprinting: false,
            try_sprinting: false,
            animation_time: 0.0,
            rotation_timer: 0.0,
        }
    }
}

impl MovingEntity {
    pub fn new() -> Self {
        Self { ..Self::default() }
    }
}
