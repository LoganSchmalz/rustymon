use crate::TILE_SIZE;
use crate::tilemap;
use crate::player::{Direction, Leg, PLAYER_WALK_SPEED, WALKING_TIME_PER_TILE, RUNNING_TIME_PER_TILE};
use sdl2::rect::Rect;


pub const NPC_WIDTH: u32 = 16;
pub const NPC_HEIGHT: u32 = 16;
const ROTATION_TIME: f64 = RUNNING_TIME_PER_TILE;

use Direction::{DOWN, LEFT, RIGHT, UP};

pub struct Npc {
    pub pos: (f64, f64),
    pub is_sprinting: bool,
    moving_towards: Option<(i32, i32)>,
    animation_time: f64,
    pub dir: Direction,
    current_leg: Leg,
    is_moving: bool,
    rotation_timer: f64,
    lr: bool,
    is_talking: bool,
}

impl Npc {
    pub fn new() -> Npc {
        Npc {
            pos: (48 as f64, 64 as f64),
            is_sprinting: false,
            moving_towards: None,
            animation_time: 0.0,
            dir: DOWN,
            current_leg: Leg::LEFT,
            is_moving: false,
            rotation_timer: 0.0,
            lr: false,
            is_talking: false,
        }
    }

    pub fn update(&mut self, delta_time: &f64) {

        match self.moving_towards {
            Some((_, _)) => {
                self.animation_time = self.animation_time - delta_time;
                self.move_towards_target(delta_time);
            }
            None => {
                self.animation_time = 0.0;
                self.lr = !self.lr;
                if self.lr {
                    self.dir = Direction::LEFT;
                    self.walk(Direction::LEFT)
                } else {
                    self.dir = Direction::RIGHT;
                    self.walk(Direction::RIGHT);
                }
            }
        }

        if self.rotation_timer < ROTATION_TIME {
            self.rotation_timer += delta_time;
        }
    }

    pub fn move_towards_target(&mut self, delta_time: &f64) {
            
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
            //compute direction (non-normalized vector)
            let dx = tx as f64 - self.pos.0;
            let dy = ty as f64 - self.pos.1;
            //compute move distance (signum normalizes)
            let mx = if dx != 0.0 {
                PLAYER_WALK_SPEED * delta_time * dx.signum()
            } else {
                0.0
            };
            let my = if dy != 0.0 {
                PLAYER_WALK_SPEED * delta_time * dy.signum()
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

    pub fn talk(&mut self, direction: Direction) {
        self.dir = direction; 
    }

    pub fn walk(&mut self, direction: Direction) {//map: &mut tilemap::TileMap) {
        //TODO MAKE IT SO THEY CANT WALK INTO SHIT LMFAO
        if direction == self.dir && self.rotation_timer >= ROTATION_TIME {
            self.is_moving = true;
            if self.moving_towards == None {
                self.animation_time = WALKING_TIME_PER_TILE;
                match direction {
                    LEFT => {
                        self.moving_towards =
                            Some((self.pos.0 as i32 - TILE_SIZE, self.pos.1 as i32));
                    }
                    RIGHT => {
                        self.moving_towards =
                            Some((self.pos.0 as i32 + TILE_SIZE, self.pos.1 as i32));
                    }
                    _ => {}
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

    pub fn stop_walk(&mut self) {
        if self.moving_towards == None {
            self.is_moving = false;
        }
    }

    pub fn get_texture(&self) -> sdl2::rect::Rect {
        let anim_time = WALKING_TIME_PER_TILE;

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
