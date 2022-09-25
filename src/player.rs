use crate::TILE_SIZE;
use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

#[derive(Debug)]
pub struct Player {
    pub pos: (f64, f64),
    pub is_sprinting: bool,
    moving_towards: Option<(i32, i32)>,
    animation_time: f64,
}

impl Player {
    pub fn new() -> Player {
        Player {
            pos: (0.0, 0.0),
            is_sprinting: false,
            moving_towards: None,
            animation_time: 0.0,
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
                    self.pos.0 + (2.0 / 16.0) * delta_time * dx.signum()
                } else {
                    self.pos.0 + (1.0 / 16.0) * delta_time * dx.signum()
                }
            } else {
                self.pos.0
            };
            let my = if dy != 0.0 {
                if self.is_sprinting {
                    self.pos.1 + (2.0 / 16.0) * delta_time * dy.signum()
                } else {
                    self.pos.1 + (1.0 / 16.0) * delta_time * dy.signum()
                }
            } else {
                self.pos.1
            };
            self.pos = (mx, my);
        }
    }

    pub fn move_left(&mut self) {
        if self.moving_towards == None {
            self.moving_towards = Some((self.pos.0 as i32 - TILE_SIZE, self.pos.1 as i32));
        }
    }
    pub fn move_right(&mut self) {
        if self.moving_towards == None {
            self.moving_towards = Some((self.pos.0 as i32 + TILE_SIZE, self.pos.1 as i32));
        }
    }
    pub fn move_up(&mut self) {
        if self.moving_towards == None {
            self.moving_towards = Some((self.pos.0 as i32, self.pos.1 as i32 - TILE_SIZE));
        }
    }
    pub fn move_down(&mut self) {
        if self.moving_towards == None {
            self.moving_towards = Some((self.pos.0 as i32, self.pos.1 as i32 + TILE_SIZE));
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas
            .fill_rect(Rect::new(
                self.pos.0 as i32 + 2,
                self.pos.1 as i32 + 2,
                12,
                12,
            ))
            .unwrap();
    }
}
