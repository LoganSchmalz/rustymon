use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};
use crate::TILE_SIZE;

#[derive(Debug)]
pub struct Player {
    pub pos: (i32, i32),
    moving_towards: Option<(i32, i32)>,
    animation_time: u32,
}

impl Player {
    pub fn new() -> Player {
        Player {
            pos: (0, 0),
            moving_towards: None,
            animation_time: 0,
        }
    }

    pub fn update(&mut self, delta_time: &f64) {
		match self.moving_towards {
			Some((x,y)) => {
				if self.animation_time == 0 {
					self.animation_time = 4;
				} else {
					self.animation_time = self.animation_time - 1;
				}
				self.pos = (x,y);
				self.moving_towards = None;
			}
			None => {}
		}
	}

    pub fn move_left(&mut self) {
        if self.moving_towards == None {
            self.moving_towards = Some((self.pos.0 - TILE_SIZE, self.pos.1));
        }
    }
    pub fn move_right(&mut self) {
        if self.moving_towards == None {
            self.moving_towards = Some((self.pos.0 + TILE_SIZE, self.pos.1));
        }
    }
    pub fn move_up(&mut self) {
        if self.moving_towards == None {
            self.moving_towards = Some((self.pos.0, self.pos.1 - TILE_SIZE));
        }
    }
    pub fn move_down(&mut self) {
        if self.moving_towards == None {
            self.moving_towards = Some((self.pos.0, self.pos.1 + TILE_SIZE));
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas
            .fill_rect(Rect::new(self.pos.0 + 2, self.pos.1 + 2, 12, 12))
            .unwrap();
    }
}
