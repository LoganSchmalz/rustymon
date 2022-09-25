use sdl2::{render::Canvas, video::Window, pixels::Color, rect::Rect};

#[derive(Debug)]
pub struct Player {
	pos: (i32, i32)
}

impl Player {
	pub fn new() -> Player {
		Player {
			pos: (0,0)
		}
	}

	pub fn move_left(&mut self) {
		self.pos = (self.pos.0-1, self.pos.1);
	}
	pub fn move_right(&mut self) {
		self.pos = (self.pos.0+1, self.pos.1);
	}
	pub fn move_up(&mut self) {
		self.pos = (self.pos.0, self.pos.1-1);
	}
	pub fn move_down(&mut self) {
		self.pos = (self.pos.0, self.pos.1+1);
	}

	pub fn render(&self, canvas: &mut Canvas<Window>) {
		canvas.set_draw_color(Color::RGB(0, 0, 0));
		canvas.fill_rect(Rect::new(self.pos.0, self.pos.1, 10, 10)).unwrap();
	}
}