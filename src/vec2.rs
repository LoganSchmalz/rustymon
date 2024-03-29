/****************************************************/
// Created by: Logan Schmalz
// Description: Logic for 2 dimensional vectors for storing positions
/****************************************************/
use std::{cmp::Ordering, ops};

use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vec2(pub f32, pub f32);

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

//compute the game direction between two vec2s for the purposes of calculating the correct moving velocity
pub fn compute_direction(pos_from: Vec2, pos_to: Vec2) -> Direction {
    let dx = pos_to.0 - pos_from.0;
    let dy = pos_to.1 - pos_from.1;

    if dx.abs() > dy.abs() {
        if dx.signum() >= 1.0 {
            Direction::Right
        } else {
            Direction::Left
        }
    } else if dy.signum() >= 1.0 {
        Direction::Down
    } else {
        Direction::Up
    }
}

impl Vec2 {
    //rounds a vec2 to the nearest integer coordinates
    #[inline]
    pub fn round_to_tile(&self) -> Vec2 {
        Vec2(self.0.round(), self.1.round())
    }

    //rounds a vec2 to the nearest integer coordinates and assumes it's positive for the purposes of grabbing data from the tilemap
    #[inline]
    pub fn to_usize(self, size_x: usize) -> usize {
        let Vec2(x, y) = self.round_to_tile();
        x as usize + y as usize * size_x
    }

    //does a dist^2 calculation in case it's needed
    #[inline]
    #[allow(dead_code)]
    pub fn dist2(&self, rhs: Vec2) -> f32 {
        (self.0 - rhs.0) * (self.0 - rhs.0) + (self.1 - rhs.1) * (self.1 - rhs.1)
    }

    //does a dist calculation
    #[inline]
    #[allow(dead_code)]
    pub fn dist(&self, rhs: Vec2) -> f32 {
        self.dist2(rhs).sqrt()
    }
}

//implementing adding two vectors
impl ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    #[inline]
    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

//implement subtracting two vectors
impl ops::Sub<Vec2> for Vec2 {
    type Output = Vec2;

    #[inline]
    fn sub(self, rhs: Vec2) -> Vec2 {
        Vec2(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl From<(f32, f32)> for Vec2 {
    fn from((x, y): (f32, f32)) -> Self {
        Vec2(x, y)
    }
}

impl From<(i32, i32)> for Vec2 {
    fn from((x, y): (i32, i32)) -> Self {
        Vec2(x as f32, y as f32)
    }
}

impl From<(u32, u32)> for Vec2 {
    fn from((x, y): (u32, u32)) -> Self {
        Vec2(x as f32, y as f32)
    }
}

impl PartialOrd for Vec2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Ordering::*;

        let (x, y) = (self.0.partial_cmp(&other.0), self.1.partial_cmp(&other.1));
        match (x, y) {
            (Some(Less), _) | (_, Some(Less)) => Some(Less),
            (Some(Greater), _) | (_, Some(Greater)) => Some(Greater),
            (_, _) => Some(Equal),
        }
    }
}
