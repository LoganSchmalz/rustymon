/****************************************************/
// Created by: Logan Schmalz
// Description: Data structures and logic required for other components including items, NPCs and logic for moving NPCs
/****************************************************/

//use std::sync::Arc;

use self::bag::Item;

use super::vec2::{Direction, Vec2};

pub mod animation;
pub mod bag;
pub mod sprite;
pub mod stray;

pub struct Player;

pub struct GroundItem {
    pub item: Item,
    pub amount: u32,
}

pub struct Npc {
    pub says: String,
    pub path: Option<WalkingPath>,
}

pub struct WalkingPath {
    pub path: Vec<Direction>,
    pub index: usize,
}

impl WalkingPath {
    //returns the direction the NPC should be at the current point on their path
    pub fn direction(&self) -> Direction {
        self.path[self.index]
    }

    //this function goes to the next direction for the npc walking path or loops around
    pub fn advance(&mut self) {
        self.index += 1;
        if self.index >= self.path.len() {
            self.index = 0;
        }
    }
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
    //sets default values for a MovingEntity
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
    //creates a new MovingEntity
    pub fn new() -> Self {
        Self { ..Self::default() }
    }
}
