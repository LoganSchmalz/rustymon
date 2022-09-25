use sdl2::keyboard::Keycode;

use crate::player;

pub fn handle_input(key: Keycode, player: &mut player::Player) {
	match key {
		Keycode::Left => {
			player.move_left();
		},
		Keycode::Right => {
			player.move_right();
		},
		Keycode::Up => {
			player.move_up();
		},
		Keycode::Down => {
			player.move_down();
		},
		_ => {}
	}
}