use sdl2::rect::Rect;

use super::coordinate::{Coordinate, Direction};

pub struct Player;

pub struct NPC;

pub struct Position(pub Coordinate);

pub struct IsSprinting(bool);

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum MovingState {
    Idle,
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
    pub fn new(pos: Coordinate) -> Self {
        Self { ..Self::default() }
    }
}

pub struct Sprite {
    pub texture: String,
    pub src: Rect,
    pub shift_x: i32,
    pub shift_y: i32,
}

impl Default for Sprite {
    fn default() -> Self {
        Self {
            texture: String::from("assets/char-sprites/augosprite.png"),
            src: Rect::new(0, 0, 16, 20),
            shift_x: 0,
            shift_y: 0,
        }
    }
}
