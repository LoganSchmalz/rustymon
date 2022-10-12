use sdl2::{
    event::{Event, WindowEvent},
    keyboard::{Keycode, Scancode},
    render::Canvas,
    video::Window,
};

use crate::{player, render};

pub struct Input {
    pub allow_input: bool,
}

impl Input {
    pub fn handle_input(
        &self,
        event_pump: &mut sdl2::EventPump,
        player: &mut player::Player,
        renderer: &mut render::Renderer,
        canvas: &mut Canvas<Window>,
    ) -> bool {
        for event in event_pump.poll_iter() {
            match event {
                Event::Window {
                    win_event: WindowEvent::Resized(width, height),
                    ..
                } => {
                    renderer.resize(canvas, width, height);
                }
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return true,
                Event::KeyDown {
                    keycode: Some(Keycode::F11),
                    ..
                } => {
                    renderer.toggle_fullscreen(canvas);
                }
                _ => {}
            }
        }

        if self.allow_input {
            let ks = event_pump.keyboard_state();
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

        false
    }
}
