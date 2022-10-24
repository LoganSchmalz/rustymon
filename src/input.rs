use sdl2::{
    event::{Event, WindowEvent},
    keyboard::{Keycode, Scancode}, render::Canvas, video::Window,
};

use crate::{player, render, tilemap, objects};
use crate::render::{DisplayScreen, Button, BUTTONS};
use crate::player::Direction;
use crate::TILE_SIZE;

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
        canvas: &mut Canvas<Window>,
        player: &mut player::Player,
        renderer: &mut render::Renderer,
        mut map: &mut tilemap::TileMap,
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
                    match renderer.display_screen {
                        DisplayScreen::MainMenu => {
                            match key {
                                Keycode::Up => {
                                    if renderer.curr_button == 0 {
                                        renderer.curr_button = 2;
                                    } else {
                                        renderer.curr_button -= 1;
                                    }
                                }
                                Keycode::Left => {
                                    match BUTTONS[renderer.curr_button] {
                                        Button::StartButton => renderer.curr_button = 1, //BUTTONS[1] == Button::LoadButton
                                        Button::LoadButton => renderer.curr_button = 0, //BUTTONS[0] == Button::StartButton
                                        Button::SettingsButton => renderer.curr_button = 1, //BUTTONS[1] == Button::LoadButton
                                    }
                                }
                                Keycode::Down => renderer.curr_button = (renderer.curr_button + 1) % 3,
                                Keycode::Right => {
                                    match BUTTONS[renderer.curr_button] {
                                        Button::StartButton => renderer.curr_button = 2, //BUTTONS[2] == Button::SettingsButton
                                        Button::LoadButton => renderer.curr_button = 2, //BUTTONS[2] == Button::SettingsButton
                                        Button::SettingsButton => renderer.curr_button = 0, //BUTTONS[0] == Button::StartButton
                                    }
                                }
                                Keycode::Space | Keycode::Return => {
                                    if BUTTONS[renderer.curr_button] == Button::StartButton {
                                        renderer.display_screen = DisplayScreen::OverWorld;
                                    }
                                }
                                _ => {}
                            }
                        }
                        DisplayScreen::OverWorld => {
                            if key == Keycode::Space || key == Keycode::Return {
                                
                                let mut temp_pos: usize = 0;

                                match player.dir {
                                    Direction::LEFT => {
                                        if (player.pos.0 / TILE_SIZE as f64) - 1.0 < 0.0 {
                                            break;
                                        }
                                        temp_pos = (player.pos.0 / TILE_SIZE as f64) as usize - 1 + (player.pos.1 / TILE_SIZE as f64) as usize * map.size_x;
                                    }
                                    Direction::RIGHT => {
                                        if (player.pos.0 / TILE_SIZE as f64) as usize + 1 >= map.size_x {
                                            break;
                                        }
                                        temp_pos = (player.pos.0 / TILE_SIZE as f64) as usize + 1 + (player.pos.1 / TILE_SIZE as f64) as usize * map.size_x;
                                    }
                                    Direction::UP => {
                                        if (player.pos.1 / TILE_SIZE as f64) - 1.0 < 0.0 {
                                            break;
                                        }
                                        temp_pos = (player.pos.0 / TILE_SIZE as f64) as usize + ((player.pos.1 / TILE_SIZE as f64) - 1.0) as usize * map.size_x;
                                    }
                                    Direction::DOWN => {
                                        if (player.pos.1 / TILE_SIZE as f64) as usize + 1 >= map.size_y {
                                            break;
                                        }
                                        temp_pos = (player.pos.0 / TILE_SIZE as f64) as usize + ((player.pos.1 / TILE_SIZE as f64) + 1.0) as usize * map.size_x;
                                    }
                                }

                                objects::object_interact(temp_pos, &mut map)
                            }
                        }
                        //_ => {}
                    }
                }
                _ => {}
            }
        }

        if self.allow_input {
            let ks = event_pump.keyboard_state();
            //maybe move the setting is_sprinting somewhere else to fix mid-tile timing?
            
            if renderer.display_screen == DisplayScreen::OverWorld {
                if ks.is_scancode_pressed(Scancode::LShift) {
                    player.sprint(true);
                } else {
                    player.sprint(false);
                }

                if ks.is_scancode_pressed(Scancode::Left) {
                    player.walk(player::Direction::LEFT, &mut map);
                } else if ks.is_scancode_pressed(Scancode::Right) {
                    player.walk(player::Direction::RIGHT, &mut map);
                } else if ks.is_scancode_pressed(Scancode::Up) {
                    player.walk(player::Direction::UP, &mut map);
                } else if ks.is_scancode_pressed(Scancode::Down) {
                    player.walk(player::Direction::DOWN, &mut map);
                } else {
                    player.stop_walk();
                }
            }
        }

        false
    }
}
