use crate::TILE_SIZE;
use sdl2::{
    pixels::Color,
    rect::Rect,
    render::{Canvas, Texture},
    video::Window,
};

const PLAYER_SPEED: f64 = 1.0 / 16.0;
const PLAYER_RUN_SPEED: f64 = 2.0 / 16.0;
const PLAYER_WIDTH: u32 = 16;
const PLAYER_HEIGHT: u32 = 16;
const UP: i8 = 0;
const RIGHT: i8 = 1;
const DOWN: i8 = 2;
const LEFT: i8 = 3;

pub struct Player<'a> {
    texture: sdl2::render::Texture<'a>,
    pub pos: (f64, f64),
    pub is_sprinting: bool,
    moving_towards: Option<(i32, i32)>,
    animation_time: f64,
    pub dir: i8,
}

impl Player<'_> {
    pub fn new(texture: sdl2::render::Texture) -> Player {
        Player {
            texture: texture,
            pos: (0.0, 0.0),
            is_sprinting: false,
            moving_towards: None,
            animation_time: 0.0,
            dir: DOWN,
        }
    }

    pub fn update(&mut self, delta_time: &f64) {
        match self.moving_towards {
            Some((x, y)) => {
                if self.animation_time == 0.0 {
                    self.animation_time = 4.0;
                } else {
                    self.animation_time = self.animation_time - delta_time;
                    self.move_towards_target(delta_time);
                    //println!("{:?}", self.moving_towards);
                }
            }
            None => {}
        }
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
                    self.pos.0 + PLAYER_SPEED * delta_time * dx.signum()
                }
            } else {
                self.pos.0
            };
            let my = if dy != 0.0 {
                if self.is_sprinting {
                    self.pos.1 + PLAYER_RUN_SPEED * delta_time * dy.signum()
                } else {
                    self.pos.1 + PLAYER_SPEED * delta_time * dy.signum()
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
        
        let mut stand_texture_quad = Rect::new(16, 0, 16, 16);
        if self.moving_towards == None {
            match self.dir {
                UP=>stand_texture_quad = Rect::new(16, 0, 16, 16),
                RIGHT=>stand_texture_quad = Rect::new(16, 16, 16, 16),
                DOWN=>stand_texture_quad = Rect::new(0, 0, 16, 16),
                LEFT=>stand_texture_quad = Rect::new(0, 16, 16, 16),
                _=>println!("bad"),
            }
        } else {
            match self.dir {
                UP=>stand_texture_quad = Rect::new(0, 32, 16, 16),
                RIGHT=>stand_texture_quad = Rect::new(48, 16, 16, 16),
                DOWN=>stand_texture_quad = Rect::new(32, 32, 16, 16),
                LEFT=>stand_texture_quad = Rect::new(32, 16, 16, 16),
                _=>println!("bad"),
            }
        }

        match canvas.copy(&self.texture, stand_texture_quad, render_quad) {
            Ok(_) => {}
            Err(_) => {
                println!("bad")
            }
        };
    }
}
