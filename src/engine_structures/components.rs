use sdl2::rect::Rect;

use crate::event::Command;

use super::{
    coordinate::{Coordinate, Direction},
    humanoid_properties::{ROTATION_TIME, RUNNING_TIME_PER_TILE, WALKING_TIME_PER_TILE},
};

pub struct Player;

pub struct NPC;

pub struct Position(pub Coordinate);

pub struct IsSprinting(bool);

pub struct Collision;

pub struct Interactions(pub Vec<Command>);

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum MovingState {
    Idle,
    CenterTile,
    Moving(Direction),
}

pub enum HumanAnimationType {
    Rotate,
    Walk,
    Run,
}
pub struct HumanWalkAnimation {
    pub rotation: Direction,
    pub time: (f32, f32),
    pub left_leg: bool,
    pub sprinting: bool,
}

impl HumanWalkAnimation {
    pub fn get_src(&self) -> Rect {
        let x = match self.rotation {
            Direction::Up => 16,
            Direction::Right => 48,
            Direction::Down => 0,
            Direction::Left => 32,
        };

        let y = if self.time.0 <= 0.5 * self.time.1 {
            match self.left_leg {
                true => 40,
                false => 20,
            }
        } else {
            0
        };

        let y = y + if self.sprinting { 60 } else { 0 };

        Rect::new(x, y, 16, 20)
    }

    pub fn play_animation(&mut self, anim_type: HumanAnimationType, rotation: Direction) {
        self.left_leg = !self.left_leg;
        self.time.0 = 0.0;
        (self.time.1, self.sprinting) = match anim_type {
            HumanAnimationType::Rotate => (ROTATION_TIME, false),
            HumanAnimationType::Walk => (WALKING_TIME_PER_TILE, false),
            HumanAnimationType::Run => (RUNNING_TIME_PER_TILE, true),
        };
        self.rotation = rotation;
    }

    pub fn is_playing(&self) -> bool {
        self.time.0 < self.time.1
    }

    pub fn update(&mut self, delta_time: f32) {
        if self.time.0 < self.time.1 {
            self.time.0 += delta_time;
        } else {
            self.sprinting = false;
        }
    }
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

impl Sprite {
    pub fn character(str: String) -> Self {
        Self {
            texture: str,
            src: Rect::new(0, 0, 16, 20),
            shift_x: 0,
            shift_y: -8,
        }
    }

    pub fn berry() -> Self {
        Self {
            texture: String::from("assets/tiles/objectsprites.png"),
            src: Rect::new(0, 0, 16, 20),
            shift_x: 0,
            shift_y: 0,
        }
    }
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
