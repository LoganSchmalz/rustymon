use std::{cmp::Ordering, ops};

use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Coordinate(pub f32, pub f32);

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn compute_direction(pos_from: Coordinate, pos_to: Coordinate) -> Direction {
    //compute direction (non-normalized vector)
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

impl Coordinate {
    #[inline]
    pub fn round_to_tile(&self) -> Coordinate {
        Coordinate(self.0.round(), self.1.round())
    }

    #[inline]
    pub fn to_usize(self, size_x: usize) -> usize {
        let Coordinate(x, y) = self.round_to_tile();
        x as usize + y as usize * size_x
    }

    #[inline]
    #[allow(dead_code)]
    pub fn dist2(&self, rhs: Coordinate) -> f32 {
        (self.0 - rhs.0) * (self.0 - rhs.0) + (self.1 - rhs.1) * (self.1 - rhs.1)
    }

    #[inline]
    #[allow(dead_code)]
    pub fn dist(&self, rhs: Coordinate) -> f32 {
        self.dist2(rhs).sqrt()
    }
}

impl ops::Add<Coordinate> for Coordinate {
    type Output = Coordinate;

    #[inline]
    fn add(self, rhs: Coordinate) -> Coordinate {
        Coordinate(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl ops::Sub<Coordinate> for Coordinate {
    type Output = Coordinate;

    #[inline]
    fn sub(self, rhs: Coordinate) -> Coordinate {
        Coordinate(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl From<(f32, f32)> for Coordinate {
    fn from((x, y): (f32, f32)) -> Self {
        Coordinate(x, y)
    }
}

impl From<(i32, i32)> for Coordinate {
    fn from((x, y): (i32, i32)) -> Self {
        Coordinate(x as f32, y as f32)
    }
}

impl From<(u32, u32)> for Coordinate {
    fn from((x, y): (u32, u32)) -> Self {
        Coordinate(x as f32, y as f32)
    }
}

impl PartialOrd for Coordinate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let (x, y) = (self.0.partial_cmp(&other.0), self.1.partial_cmp(&other.1));
        match (x, y) {
            (_, Some(Ordering::Less)) => Some(Ordering::Less),
            (_, Some(Ordering::Greater)) => Some(Ordering::Greater),
            (Some(Ordering::Less), Some(_)) => Some(Ordering::Less),
            (Some(Ordering::Greater), Some(_)) => Some(Ordering::Greater),
            (_, _) => Some(Ordering::Equal),
        }
    }
}
