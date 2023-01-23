use std::ops;

use serde::{Serialize, Deserialize};

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Coordinate(pub f64, pub f64);

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

pub fn compute_direction(pos_from: Coordinate, pos_to: Coordinate) -> Direction {
    //compute direction (non-normalized vector)
    let dx = pos_to.0 - pos_from.0;
    let dy = pos_to.1 - pos_from.1;

    if dx.abs() > dy.abs() {
        if dx.signum() >= 1.0 {
            Direction::RIGHT
        } else {
            Direction::LEFT
        }
    } else {
        if dy.signum() >= 1.0 {
            Direction::DOWN
        } else {
            Direction::UP
        }
    }
}

impl Coordinate {
    pub fn round_to_tile(&self) -> Coordinate {
        Coordinate(self.0.round(), self.1.round())
    }

    pub fn to_usize(&self, size_x: usize) -> usize {
        let Coordinate(x,y) = self.round_to_tile();
        x as usize + y as usize * size_x
    }

    pub fn dist2(&self, rhs: Coordinate) -> f64 {
        (self.0 - rhs.0)*(self.0 - rhs.0) + (self.1 - rhs.1)*(self.1 - rhs.1)
    }

    pub fn dist(&self, rhs: Coordinate) -> f64 {
        self.dist2(rhs).sqrt()
    }
}

impl ops::Add<Coordinate> for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Coordinate) -> Coordinate {
        Coordinate(self.0 + rhs.0, self.1 + rhs.1)
    }
}