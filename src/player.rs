use crate::TILE_SIZE;
use crate::tilemap;
use sdl2::rect::Rect;

const PLAYER_WALK_SPEED: f64 = 1.0 / 16.0;
const WALKING_TIME_PER_TILE: f64 = 1.0 / (PLAYER_WALK_SPEED / TILE_SIZE as f64); // in ms b/c delta_time in ms
const PLAYER_RUN_SPEED: f64 = 2.0 / 16.0;
pub const RUNNING_TIME_PER_TILE: f64 = 1.0 / (1.0 * PLAYER_RUN_SPEED / TILE_SIZE as f64); // in ms b/c delta_time in ms
pub const PLAYER_WIDTH: u32 = 16;
pub const PLAYER_HEIGHT: u32 = 16;
const ROTATION_TIME: f64 = RUNNING_TIME_PER_TILE;

#[derive(PartialEq, Clone, Copy)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

pub enum Leg {
    LEFT,
    RIGHT,
}

use Direction::{DOWN, LEFT, RIGHT, UP};
pub struct Player {
    pub pos: (f64, f64),
    pub is_sprinting: bool,
    moving_towards: Option<(i32, i32)>,
    animation_time: f64,
    pub dir: Direction,
    current_leg: Leg,
    is_moving: bool,
    rotation_timer: f64,
}

impl Player {
    pub fn new() -> Player {
        Player {
            pos: (0.0, 0.0),
            is_sprinting: false,
            moving_towards: None,
            animation_time: 0.0,
            dir: DOWN,
            current_leg: Leg::LEFT,
            is_moving: false,
            rotation_timer: 0.0,
        }
    }

    pub fn update(&mut self, delta_time: &f64) {
        match self.moving_towards {
            Some((_, _)) => {
                /*if self.animation_time < 0.0 {
                    self.animation_time = if self.is_sprinting {
                        RUNNING_TIME_PER_TILE
                    } else {
                        WALKING_TIME_PER_TILE
                    }
                } else {*/
                self.animation_time = self.animation_time - delta_time;
                self.move_towards_target(delta_time);
                //}
            }
            None => {
                self.animation_time = 0.0;
            }
        }

        if self.rotation_timer < ROTATION_TIME {
            self.rotation_timer += delta_time;
        }
    }

    pub fn move_towards_target(&mut self, delta_time: &f64) {
        //concept from https://gamedev.stackexchange.com/questions/31410/keeping-player-aligned-to-grid-in-pacman

        let (tx, ty) = self.moving_towards.unwrap();

        //if we are on tile
        if (self.pos.0, self.pos.1) == (tx as f64, ty as f64) {
            //self.animation_time = 0.0;
            self.moving_towards = None;
            self.current_leg = match self.current_leg {
                Leg::LEFT => {
                    Leg::RIGHT
                }
                Leg::RIGHT => {
                    Leg::LEFT
                }
            };
        } else {
            let speed = if self.is_sprinting {
                PLAYER_RUN_SPEED
            } else {
                PLAYER_WALK_SPEED
            };

            //compute direction (non-normalized vector)
            let dx = tx as f64 - self.pos.0;
            let dy = ty as f64 - self.pos.1;
            //compute move distance (signum normalizes)
            let mx = if dx != 0.0 {
                speed * delta_time * dx.signum()
            } else {
                0.0
            };
            let my = if dy != 0.0 {
                speed * delta_time * dy.signum()
            } else {
                0.0
            };
            //set new position
            self.pos = (self.pos.0 + mx, self.pos.1 + my);

            //check if we have passed the tile we were trying to get to
            if dx != 0.0 && (tx as f64 - self.pos.0).signum() != dx.signum()
                || dy != 0.0 && (ty as f64 - self.pos.1).signum() != dy.signum()
            {
                self.pos = (tx as f64, ty as f64);
            }
        }
    }

    pub fn walk(&mut self, direction: Direction, map: &mut tilemap::TileMap) {
        if direction == self.dir && self.rotation_timer >= ROTATION_TIME {
            self.is_moving = true;
            if self.moving_towards == None {
                self.animation_time = if self.is_sprinting {
                    RUNNING_TIME_PER_TILE
                } else {
                    WALKING_TIME_PER_TILE
                };
                match direction {
                    LEFT => {
                        if (self.pos.0 / TILE_SIZE as f64) - 1.0 < 0.0 {
                            return;
                        }
                        match map.collision.get((self.pos.0 / TILE_SIZE as f64) as usize - 1 + (self.pos.1 / TILE_SIZE as f64) as usize * map.size_x) {
                            Some(tilemap::CollisionTile::NONE) => {
                                self.moving_towards =
                                    Some((self.pos.0 as i32 - TILE_SIZE, self.pos.1 as i32))
                            }
                            _ => {}
                        }
                    }
                    RIGHT => {
                        if (self.pos.0 / TILE_SIZE as f64) as usize + 1 >= map.size_x {
                            return;
                        }
                        match map.collision.get((self.pos.0 / TILE_SIZE as f64) as usize + 1 + (self.pos.1 / TILE_SIZE as f64) as usize * map.size_x) {
                            Some(tilemap::CollisionTile::NONE) => {
                                self.moving_towards =
                                    Some((self.pos.0 as i32 + TILE_SIZE, self.pos.1 as i32))
                                }
                            _ => {}
                        }
                    }
                    UP => {
                        if (self.pos.1 / TILE_SIZE as f64) - 1.0 < 0.0 {
                            return;
                        }
                        match map.collision.get((self.pos.0 / TILE_SIZE as f64) as usize + ((self.pos.1 / TILE_SIZE as f64) - 1.0) as usize * map.size_x) {
                            Some(tilemap::CollisionTile::NONE) => {
                                self.moving_towards =
                                    Some((self.pos.0 as i32, self.pos.1 as i32 - TILE_SIZE))
                            }
                            _ => {}
                        }
                    }
                    DOWN => {
                        match map.collision.get((self.pos.0 / TILE_SIZE as f64) as usize + ((self.pos.1 / TILE_SIZE as f64) + 1.0) as usize * map.size_x) {
                            Some(tilemap::CollisionTile::NONE) => {
                                self.moving_towards =
                                    Some((self.pos.0 as i32, self.pos.1 as i32 + TILE_SIZE))
                            }
                            _ => {}
                        }
                    }
                }
            }
        } else if direction != self.dir {
            if self.is_moving && self.moving_towards == None {
                self.dir = direction;
                self.rotation_timer = ROTATION_TIME;
            } else if !self.is_moving {
                self.dir = direction;
                self.rotation_timer = 0.0;
            }
        }
    }

    pub fn sprint(&mut self, set_sprinting: bool) {
        if self.moving_towards == None {
            self.is_sprinting = set_sprinting;
        }
    }

    pub fn stop_walk(&mut self) {
        if self.moving_towards == None {
            self.is_moving = false;
        }
    }

    pub fn get_texture(&self) -> sdl2::rect::Rect {
        let anim_time = if self.is_sprinting {
            RUNNING_TIME_PER_TILE
        } else {
            WALKING_TIME_PER_TILE
        };

        if self.moving_towards == None
            || self.animation_time > (0.75 * anim_time)
            || self.animation_time < (0.25 * anim_time)
        {
            match self.dir {
                UP => Rect::new(16, 0, 16, 16),
                RIGHT => Rect::new(16, 16, 16, 16),
                DOWN => Rect::new(0, 0, 16, 16),
                LEFT => Rect::new(0, 16, 16, 16),
            }
        } else {
            match self.dir {
                UP => match self.current_leg {
                    Leg::LEFT => Rect::new(16, 32, 16, 16),
                    Leg::RIGHT => Rect::new(0, 32, 16, 16),
                },
                RIGHT => Rect::new(48, 16, 16, 16),
                DOWN => match self.current_leg {
                    Leg::LEFT => Rect::new(32, 32, 16, 16),
                    Leg::RIGHT => Rect::new(48, 32, 16, 16),
                },
                LEFT => Rect::new(32, 16, 16, 16),
            }
        }
    }
}
