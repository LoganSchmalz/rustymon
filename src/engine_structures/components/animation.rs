use sdl2::rect::Rect;

use crate::engine_structures::{vec2::Direction, humanoid_properties::{ROTATION_TIME, WALKING_TIME_PER_TILE, RUNNING_TIME_PER_TILE}};

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
        //calculate x coordinate of sprite
        let x = match self.rotation {
            Direction::Up => 16,
            Direction::Right => 48,
            Direction::Down => 0,
            Direction::Left => 32,
        };

        //calculate y coordinate based on leg and time
        let y = if self.time.0 <= 0.5 * self.time.1 {
            match self.left_leg {
                true => 40,
                false => 20,
            }
        } else {
            0
        };

        //calculate y coordinate based on sprinting
        let y = y + if self.sprinting { 60 } else { 0 };

        Rect::new(x, y, 16, 20)
    }

    pub fn play_animation(&mut self, anim_type: HumanAnimationType, rotation: Direction) {
        //this starts the animation
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
            self.sprinting = false; //don't want to leave the animation stuck sprinting when the animation is over
        }
    }
}