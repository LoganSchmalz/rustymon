use crate::TILE_SIZE;
use sdl2::{
    pixels::Color,
    rect::Rect,
    render::{Canvas, Texture},
    video::Window,
};

const PLAYER_WALK_SPEED: f64 = 1.0 / 16.0;
const WALKING_TIME_PER_TILE: f64 = 1.0 / (PLAYER_WALK_SPEED / TILE_SIZE as f64); // in ms b/c delta_time in ms
const PLAYER_RUN_SPEED: f64 = 2.0 / 16.0;
const RUNNING_TIME_PER_TILE: f64 = 1.0 / (1.0 * PLAYER_RUN_SPEED / TILE_SIZE as f64); // in ms b/c delta_time in ms
const PLAYER_WIDTH: u32 = 16;
const PLAYER_HEIGHT: u32 = 16;
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

pub struct Player<'a> {
    texture: Texture<'a>,
    texture_slice: sdl2::rect::Rect,
    pub pos: (f64, f64),
    pub is_sprinting: bool,
    moving_towards: Option<(i32, i32)>,
    animation_time: f64,
    pub dir: Direction,
    current_leg: Leg,
}

impl Player<'_> {
    pub fn new(texture: Texture) -> Player {
        Player {
            texture: texture,
            texture_slice: Rect::new(0, 0, 16, 16),
            pos: (0.0, 0.0),
            is_sprinting: false,
            moving_towards: None,
            animation_time: 0.0,
            dir: DOWN,
            current_leg: Leg::LEFT,
        }
    }

    pub fn update(&mut self, delta_time: &f64) {
        match self.moving_towards {
            Some((_, _)) => {
                if self.animation_time <= 0.0 {
                    self.animation_time = if self.is_sprinting {
                        RUNNING_TIME_PER_TILE
                    } else {
                        WALKING_TIME_PER_TILE
                    }
                } else {
                    self.animation_time = self.animation_time - delta_time;
                    self.move_towards_target(delta_time);
                }
            }
            None => {
                self.animation_time = 0.0;
                self.current_leg = match self.current_leg {
                    Leg::LEFT => Leg::RIGHT,
                    Leg::RIGHT => Leg::LEFT,
                };
            }
        }

        let anim_time = if self.is_sprinting {
            RUNNING_TIME_PER_TILE
        } else {
            WALKING_TIME_PER_TILE
        };

        self.texture_slice = if self.moving_towards == None
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
        };
    }

    pub fn move_towards_target(&mut self, delta_time: &f64) {
        let (tx, ty) = self.moving_towards.unwrap();
        if (self.pos.0.round() as i32, self.pos.1.round() as i32) == (tx, ty) {
            self.pos = (self.pos.0.round(), self.pos.1.round());
            self.moving_towards = None;
        } else {
            let dx = tx as f64 - self.pos.0;
            let dy = ty as f64 - self.pos.1;
            let mx = if dx != 0.0 {
                if self.is_sprinting {
                    self.pos.0 + PLAYER_RUN_SPEED * delta_time * dx.signum()
                } else {
                    self.pos.0 + PLAYER_WALK_SPEED * delta_time * dx.signum()
                }
            } else {
                self.pos.0
            };
            let my = if dy != 0.0 {
                if self.is_sprinting {
                    self.pos.1 + PLAYER_RUN_SPEED * delta_time * dy.signum()
                } else {
                    self.pos.1 + PLAYER_WALK_SPEED * delta_time * dy.signum()
                }
            } else {
                self.pos.1
            };
            self.pos = (mx, my);
        }
    }

    pub fn move_left(&mut self) {
        if self.moving_towards == None {
            self.dir = LEFT;
            self.moving_towards = Some((self.pos.0 as i32 - TILE_SIZE, self.pos.1 as i32));
        }
    }
    pub fn move_right(&mut self) {
        if self.moving_towards == None {
            self.dir = RIGHT;
            self.moving_towards = Some((self.pos.0 as i32 + TILE_SIZE, self.pos.1 as i32));
        }
    }
    pub fn move_up(&mut self) {
        if self.moving_towards == None {
            self.dir = UP;
            self.moving_towards = Some((self.pos.0 as i32, self.pos.1 as i32 - TILE_SIZE));
        }
    }
    pub fn move_down(&mut self) {
        if self.moving_towards == None {
            self.dir = DOWN;
            self.moving_towards = Some((self.pos.0 as i32, self.pos.1 as i32 + TILE_SIZE));
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        let render_quad = Rect::new(
            self.pos.0 as i32,
            self.pos.1 as i32,
            PLAYER_WIDTH,
            PLAYER_HEIGHT,
        );

        canvas
            .copy(&self.texture, self.texture_slice, render_quad)
            .unwrap();
    }
}
