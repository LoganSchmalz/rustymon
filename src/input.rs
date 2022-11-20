use sdl2::{
    event::{Event, WindowEvent},
    keyboard::{Keycode, Scancode},
    render::Canvas,
    video::Window,
};

use crate::TILE_SIZE;
use crate::{menu, player::Direction};
use crate::{objects, player, render, tilemap};

pub struct Input {
    pub allow_input: bool,
}

impl Input {
    pub fn new() -> Input {
        Input { allow_input: true }
    }

    pub fn handle_keydown(
        &self,
        key: Keycode,
        player: &mut player::Player,
        renderer: &mut render::Renderer,
        mut map: &mut tilemap::TileMap,
        menu_man: &mut menu::MenuManager,
    ) -> () {
        if menu_man.is_open() {
            match key {
                Keycode::Up => {
                    menu_man.interact(menu::Action::UP);
                }
                Keycode::Left => {
                    menu_man.interact(menu::Action::LEFT);
                }
                Keycode::Down => {
                    menu_man.interact(menu::Action::DOWN);
                }
                Keycode::Right => {
                    menu_man.interact(menu::Action::RIGHT);
                }
                Keycode::Space | Keycode::Return => {
                    menu_man.interact(menu::Action::ACCEPT);
                }
                Keycode::X => {
                    menu_man.interact(menu::Action::REJECT);
                }
                _ => {}
            }
        } else {
            if key == Keycode::Space || key == Keycode::Return {
                let temp_pos: usize;

                match player.dir {
                    Direction::LEFT => {
                        if (player.pos.0 / TILE_SIZE as f64) - 1.0 < 0.0 {
                            return;
                        }
                        temp_pos = (player.pos.0 / TILE_SIZE as f64) as usize - 1
                            + (player.pos.1 / TILE_SIZE as f64) as usize * map.size_x;
                    }
                    Direction::RIGHT => {
                        if (player.pos.0 / TILE_SIZE as f64) as usize + 1 >= map.size_x {
                            return;
                        }
                        temp_pos = (player.pos.0 / TILE_SIZE as f64) as usize
                            + 1
                            + (player.pos.1 / TILE_SIZE as f64) as usize * map.size_x;
                    }
                    Direction::UP => {
                        if (player.pos.1 / TILE_SIZE as f64) - 1.0 < 0.0 {
                            return;
                        }
                        temp_pos = (player.pos.0 / TILE_SIZE as f64) as usize
                            + ((player.pos.1 / TILE_SIZE as f64) - 1.0) as usize * map.size_x;
                    }
                    Direction::DOWN => {
                        if (player.pos.1 / TILE_SIZE as f64) as usize + 1 >= map.size_y {
                            return;
                        }
                        temp_pos = (player.pos.0 / TILE_SIZE as f64) as usize
                            + ((player.pos.1 / TILE_SIZE as f64) + 1.0) as usize * map.size_x;
                    }
                }

                objects::object_interact(temp_pos, &mut map, renderer, menu_man);
            }
        }
    }

    pub fn handle_input(
        &self,
        event_pump: &mut sdl2::EventPump,
        canvas: &mut Canvas<Window>,
        player: &mut player::Player,
        renderer: &mut render::Renderer,
        mut map: &mut tilemap::TileMap,
        menu_man: &mut menu::MenuManager,
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
                } => {
                    return true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::F11),
                    ..
                } => {
                    renderer.toggle_fullscreen(canvas);
                }
                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    self.handle_keydown(key, player, renderer, map, menu_man);
                }
                _ => {}
            }
        }

        if self.allow_input {
            let ks = event_pump.keyboard_state();

            if !menu_man.is_open() {
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
