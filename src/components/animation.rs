/****************************************************/
// Created by: Logan Schmalz
// Description: Logic for character sprite animations
/****************************************************/

use sdl2::rect::Rect;

use crate::{
    constants::{ROTATION_TIME, RUNNING_TIME_PER_TILE, WALKING_TIME_PER_TILE},
    vec2::Direction,
};

pub enum HumanAnimationType {
    Rotate,
    Walk,
    Run,
}
pub struct HumanWalkAnimation {
    pub rotation: Direction, //the current direction the player is facing
    pub time: (f32, f32), //contains current time and max time as a tuple
    pub left_leg: bool, //whether or not the left leg is active (switches each step)
    pub sprinting: bool, //whether or not the player is sprinting
}

impl HumanWalkAnimation {
    //method that returns a rectangle in the position and size of the current sprite of the player
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

    //starts changing the member variables of the struct to play the animation
    //takes in anim_type (the type of the animation, depends on the players movement), and rotation (which way the player is facing)
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

    //returns whether or not the animation is playing
    pub fn is_playing(&self) -> bool {
        self.time.0 < self.time.1
    }

    //updates the animation to either stop sprinting or increment the time, depending on whether the animation should be complete or not
    //takes in delta_time (the amount we will increment the time by if the animation is not complete)
    pub fn update(&mut self, delta_time: f32) {
        if self.time.0 < self.time.1 {
            self.time.0 += delta_time;
        } else {
            self.sprinting = false; //don't want to leave the animation stuck sprinting when the animation is over
        }
    }
}
