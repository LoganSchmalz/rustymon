use sdl2::{
    event::{Event, WindowEvent},
    keyboard::{Keycode, Scancode},
    render::Canvas,
    video::Window,
};

use crate::{menu, humanoid::{Humanoid}, coordinate::{Coordinate, Direction}};
use crate::{object, player, render, tilemap};

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
        menu_man: &mut menu::MenuManager,
        obj_man: &mut object::ObjectManager,
        map: &mut tilemap::TileMap,
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
                let Coordinate(x,y) = player.get_pos();
                let temp_pos = match player.get_facing() {
                    Direction::LEFT => {
                       Coordinate(x - 1.0, y)
                    }
                    Direction::RIGHT => {
                        Coordinate(x + 1.0, y)
                    }
                    Direction::UP => {
                        Coordinate(x, y - 1.0)
                    }
                    Direction::DOWN => {
                        Coordinate(x, y + 1.0)
                    }
                };

                obj_man.interact(temp_pos, player.get_pos(), renderer, menu_man, map);
            }
        }
    }

    pub fn handle_input(
        &self,
        event_pump: &mut sdl2::EventPump,
        canvas: &mut Canvas<Window>,
        player: &mut player::Player,
        renderer: &mut render::Renderer,
        map: &mut tilemap::TileMap,
        menu_man: &mut menu::MenuManager,
        obj_man: &mut object::ObjectManager
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
                    keycode: Some(key), repeat, ..
                } => {
                    if !repeat {
                        self.handle_keydown(key, player, renderer, menu_man, obj_man, map);
                    }
                }
                _ => {}
            }
        }

        if self.allow_input {
            let ks = event_pump.keyboard_state();

            if !menu_man.is_open() {
                if ks.is_scancode_pressed(Scancode::LShift) {
                    player.set_try_sprinting(true);
                } else {
                    player.set_try_sprinting(false);
                }

                if ks.is_scancode_pressed(Scancode::Left) {
                    player.set_walking(Some(Direction::LEFT));
                } else if ks.is_scancode_pressed(Scancode::Right) {
                    player.set_walking(Some(Direction::RIGHT));
                } else if ks.is_scancode_pressed(Scancode::Up) {
                    player.set_walking(Some(Direction::UP));
                } else if ks.is_scancode_pressed(Scancode::Down) {
                    player.set_walking(Some(Direction::DOWN));
                } else {
                    player.set_walking(None);
                }
            } else {
                player.set_walking(None);
            }
        }

        false
    }
}
