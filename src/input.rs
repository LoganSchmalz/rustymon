use sdl2::{
    keyboard::{Keycode, Scancode},
    sys::SDL_GetKeyboardState,
};

use crate::player;

pub struct Input {
    pub allow_input: bool,
}

impl Input {
    pub fn handle_input(&self, ks: sdl2::keyboard::KeyboardState, player: &mut player::Player) {
        if self.allow_input {
            //maybe move the setting is_sprinting somewhere else to fix mid-tile timing?
            if ks.is_scancode_pressed(Scancode::LShift) {
                player.is_sprinting = true;
            } else {
                player.is_sprinting = false;
            }

            if ks.is_scancode_pressed(Scancode::Left) {
                player.move_left();
            } else if ks.is_scancode_pressed(Scancode::Right) {
                player.move_right();
            } else if ks.is_scancode_pressed(Scancode::Up) {
                player.move_up();
            } else if ks.is_scancode_pressed(Scancode::Down) {
                player.move_down();
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
