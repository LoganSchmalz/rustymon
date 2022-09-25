use sdl2::keyboard::Keycode;

use crate::player;

pub struct Input {
    pub allow_input: bool,
}

impl Input {
    pub fn handle_input(&self, key: Keycode, player: &mut player::Player) {
        if self.allow_input {
            match key {
                Keycode::Left => {
                    player.move_left();
                }
                Keycode::Right => {
                    player.move_right();
                }
                Keycode::Up => {
                    player.move_up();
                }
                Keycode::Down => {
                    player.move_down();
                }
                _ => {}
            }
        }
    }

	pub fn input_off(&mut self) {
		self.allow_input = false;
	}

	pub fn input_on(&mut self) {
		self.allow_input = true;
	}
}
