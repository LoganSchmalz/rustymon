use serde::{Deserialize, Serialize};

use crate::{
    coordinate::{Coordinate, Direction},
    object::CollisionManager,
    tilemap, TILE_SIZE, engine_structures::collision::Collision,
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Leg {
    Left,
    Right,
}

pub const WALK_SPEED: f64 = 1.0 / 16.0;
pub const WALKING_TIME_PER_TILE: f64 = 1.0 / (WALK_SPEED / crate::TILE_SIZE as f64); // in ms b/c delta_time in ms
pub const RUN_SPEED: f64 = 2.0 / 16.0;
pub const RUNNING_TIME_PER_TILE: f64 = 1.0 / (1.0 * RUN_SPEED / crate::TILE_SIZE as f64); // in ms b/c delta_time in ms
pub const WIDTH: u32 = 16;
pub const HEIGHT: u32 = 16;
pub const ROTATION_TIME: f64 = RUNNING_TIME_PER_TILE;

pub trait Humanoid {
    fn get_pos(&self) -> Coordinate;
    fn set_pos(&mut self, pos: Coordinate);
    fn get_prev_pos(&self) -> Coordinate;
    fn set_prev_pos(&mut self, pos: Coordinate);
    fn get_facing(&self) -> Direction;
    fn set_facing(&mut self, dir: Direction);
    fn get_moving_towards(&self) -> Option<Coordinate>;
    fn set_moving_towards(&mut self, pos: Option<Coordinate>);
    fn get_current_leg(&self) -> Leg;
    fn set_current_leg(&mut self, leg: Leg);
    fn get_try_sprinting(&self) -> bool;
    fn set_try_sprinting(&mut self, is_sprinting: bool);
    fn get_is_sprinting(&self) -> bool;
    fn set_is_sprinting(&mut self, is_sprinting: bool);
    fn get_try_walking(&self) -> Option<Direction>;
    fn set_try_walking(&mut self, walking: Option<Direction>);
    fn get_rotation_timer(&self) -> f64;
    fn set_rotation_timer(&mut self, time: f64);

    fn get_animation_time(&self) -> f64;
    fn set_animation_time(&mut self, time: f64);

    fn move_towards_target(&mut self, delta_time: &f64) {
        /*if self.get_moving_towards() == None {
            return;
        }*/

        let Coordinate(x, y) = self.get_pos();
        let Coordinate(target_x, target_y) = self.get_moving_towards().unwrap();

        let speed = if self.get_is_sprinting() {
            RUN_SPEED
        } else {
            WALK_SPEED
        };

        //compute direction (non-normalized vector)
        let dx = target_x - x;
        let dy = target_y - y;
        //compute move distance (signum normalizes)
        let mx = if dx != 0.0 {
            speed * delta_time * dx.signum() / TILE_SIZE as f64
        } else {
            0.0
        };
        let my = if dy != 0.0 {
            speed * delta_time * dy.signum() / TILE_SIZE as f64
        } else {
            0.0
        };
        //set new position
        self.set_pos(Coordinate(x + mx, y + my));
        let Coordinate(x, y) = self.get_pos();
        //check if we have passed the tile we were trying to get to
        if (x, y) == (target_x, target_y)
            || dx != 0.0 && (target_x - x).signum() != dx.signum()
            || dy != 0.0 && (target_y - y).signum() != dy.signum()
        {
            self.set_pos(Coordinate(target_x, target_y));
            self.set_moving_towards(None);
            match self.get_try_walking() {
                Some(dir) => {
                    self.set_facing(dir);
                    self.set_try_walking(Some(dir));
                }
                None => {}
            }
        }
    }

    fn next_pos(&self, direction: Direction) -> Coordinate {
        let Coordinate(x, y) = self.get_pos();
        let x = x.round();
        let y = y.round();
        match direction {
            Direction::Left => Coordinate(x - 1.0, y),
            Direction::Right => Coordinate(x + 1.0, y),
            Direction::Up => Coordinate(x, y - 1.0),
            Direction::Down => Coordinate(x, y + 1.0),
        }
    }

    fn check_collision(
        &self,
        pos: Coordinate,
        map: &tilemap::TileMap,
        collision_manager: &CollisionManager,
    ) -> Collision {
        let Coordinate(next_x, next_y) = pos;

        if next_x < 0.0
            || next_x >= map.size_x as f64
            || next_y < 0.0
            || next_y >= map.size_y as f64
        {
            return Collision::Collision;
        }

        if map.check_collision(pos) == Collision::NoCollision {
            return collision_manager.check_collision(pos, self.get_prev_pos(), map.size_x)
        }
        Collision::Collision
    }

    fn walk(
        &mut self,
        delta_time: &f64,
        map: &tilemap::TileMap,
        collision_manager: &CollisionManager,
    ) {
        // if player is already moving towards a tile, just do that and don't do any other updates
        if self.get_moving_towards() != None {
            self.move_towards_target(delta_time);
            return;
        }

        // don't update any variables if not trying to walk
        if self.get_try_walking() == None {
            return;
        }

        // otherwise, player is stopped and not moving
        self.set_animation_time(0.0);
        let walk_direction = self.get_try_walking().unwrap();

        // set up rotation when player rotates
        if walk_direction != self.get_facing() && self.get_moving_towards() == None {
            self.set_facing(walk_direction);
            self.set_rotation_timer(ROTATION_TIME);
            return;
        }

        // wait for rotation time to complete
        if self.get_rotation_timer() > 0.0 {
            return;
        }

        // swap leg animations so movement is natural
        self.set_current_leg(match self.get_current_leg() {
            Leg::Left => Leg::Right,
            Leg::Right => Leg::Left,
        });

        self.set_is_sprinting(self.get_try_sprinting());

        self.set_animation_time(if self.get_is_sprinting() {
            RUNNING_TIME_PER_TILE
        } else {
            WALKING_TIME_PER_TILE
        });

        // move player in requested direction if possible
        let next_pos = self.next_pos(walk_direction);

        if self.check_collision(next_pos, map, collision_manager) == Collision::NoCollision {
            self.set_prev_pos(self.get_pos());
            self.set_moving_towards(Some(next_pos));
            self.move_towards_target(delta_time);
            return;
        } else {
            self.set_moving_towards(None);
        }
    }
}
