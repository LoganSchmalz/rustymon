use sdl2::{
    event::{Event, WindowEvent},
    keyboard::{Keycode, Scancode},
    render::Canvas,
    video::Window,
};

use crate::{player, render};
use crate::render::DisplayScreen;

pub struct Input {
    pub allow_input: bool,
}

impl Input {
    pub fn new() -> Input {
        Input { allow_input: true }
    }

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
                Event::KeyDown {
                    keycode: Some(key),
                    ..
                } => {
                    if renderer.get_display_screen() == &DisplayScreen::MainMenu {
                        match key {
                            Keycode::Up | Keycode::Left => renderer.prev_button(),
                            Keycode::Down | Keycode::Right => renderer.next_button(),
                            Keycode::Space | Keycode::Return => renderer.select_button(),
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        if self.allow_input {
            let ks = event_pump.keyboard_state();
            //maybe move the setting is_sprinting somewhere else to fix mid-tile timing?
            
            match renderer.get_display_screen() {
                DisplayScreen::MainMenu => {
                    /*if ks.is_scancode_pressed(Scancode::Space) || ks.is_scancode_pressed(Scancode::Return) {
                        renderer.select_button();
                    }

                    if ks.is_scancode_pressed(Scancode::Left) {
                        renderer.prev_button();
                    } else if ks.is_scancode_pressed(Scancode::Right) {
                        renderer.next_button();
                    } else if ks.is_scancode_pressed(Scancode::Up) {
                        renderer.prev_button();
                    } else if ks.is_scancode_pressed(Scancode::Down) {
                        renderer.next_button();
                    }*/
                }
                DisplayScreen::OverWorld => {
                    if ks.is_scancode_pressed(Scancode::LShift) {
                        player.is_sprinting = true;
                    } else {
                        player.is_sprinting = false;
                    }

                    if ks.is_scancode_pressed(Scancode::Left) {
                        player.walk(player::Direction::LEFT);
                    } else if ks.is_scancode_pressed(Scancode::Right) {
                        player.walk(player::Direction::RIGHT);
                    } else if ks.is_scancode_pressed(Scancode::Up) {
                        player.walk(player::Direction::UP);
                    } else if ks.is_scancode_pressed(Scancode::Down) {
                        player.walk(player::Direction::DOWN);
                    } else {
                        player.stop_walk();
                    }
                }
            }
        }

        false
    }
}
