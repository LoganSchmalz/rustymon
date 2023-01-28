use sdl2::{
    event::{Event, WindowEvent},
    keyboard::{Keycode, Scancode},
    render::Canvas,
    video::Window,
};

use crate::{
    bag,
    coordinate::Direction,
    menu::{self, menu_events::MenuInput},
};
use crate::{object, player, render, tilemap};

pub struct Input {
    pub allow_input: bool,
}

pub enum InputEvent {
    MenuInteract(MenuInput),
    PlayerSprinting,
    PlayerWalking,
    PlayerMove(Option<Direction>),
    Interact,
    ExitGame,
    ToggleFullscreen,
    ResizeWindow(i32, i32),
}

impl Input {
    pub fn new() -> Input {
        Input { allow_input: true }
    }

    pub fn handle_keydown(
        &self,
        key: Keycode,
        menu_man: &mut menu::MenuManager,
    ) -> Vec<InputEvent> {
        let mut input_events = vec![];

        if menu_man.is_open() {
            match key {
                Keycode::Up => input_events.push(InputEvent::MenuInteract(MenuInput::Up)),
                Keycode::Left => input_events.push(InputEvent::MenuInteract(MenuInput::Left)),
                Keycode::Down => input_events.push(InputEvent::MenuInteract(MenuInput::Down)),
                Keycode::Right => input_events.push(InputEvent::MenuInteract(MenuInput::Right)),
                Keycode::Space | Keycode::Return => {
                    input_events.push(InputEvent::MenuInteract(MenuInput::Accept))
                }
                Keycode::X => input_events.push(InputEvent::MenuInteract(MenuInput::Reject)),
                _ => {}
            }
        } else {
            if key == Keycode::Return {
                input_events.push(InputEvent::MenuInteract(MenuInput::Start));
            }

            if key == Keycode::Space {
                input_events.push(InputEvent::Interact)
            }
        }

        input_events
    }

    pub fn handle_input(
        &self,
        event_pump: &mut sdl2::EventPump,
        menu_man: &mut menu::MenuManager,
    ) -> Vec<InputEvent> {
        let mut input_events = vec![];

        for event in event_pump.poll_iter() {
            match event {
                Event::Window {
                    win_event: WindowEvent::Resized(width, height),
                    ..
                } => input_events.push(InputEvent::ResizeWindow(width, height)),
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => input_events.push(InputEvent::ExitGame),
                Event::KeyDown {
                    keycode: Some(Keycode::F11),
                    ..
                } => input_events.push(InputEvent::ToggleFullscreen),
                Event::KeyDown {
                    keycode: Some(key),
                    repeat,
                    ..
                } => {
                    if !repeat {
                        input_events.append(&mut self.handle_keydown(key, menu_man));
                    }
                }
                _ => {}
            }
        }

        let ks = event_pump.keyboard_state();

        if !menu_man.is_open() {
            if ks.is_scancode_pressed(Scancode::LShift) {
                input_events.push(InputEvent::PlayerSprinting)
            } else {
                input_events.push(InputEvent::PlayerWalking)
            }

            if ks.is_scancode_pressed(Scancode::Left) {
                input_events.push(InputEvent::PlayerMove(Some(Direction::Left)));
            } else if ks.is_scancode_pressed(Scancode::Right) {
                input_events.push(InputEvent::PlayerMove(Some(Direction::Right)));
            } else if ks.is_scancode_pressed(Scancode::Up) {
                input_events.push(InputEvent::PlayerMove(Some(Direction::Up)));
            } else if ks.is_scancode_pressed(Scancode::Down) {
                input_events.push(InputEvent::PlayerMove(Some(Direction::Down)));
            } else {
                input_events.push(InputEvent::PlayerMove(None));
            }
        }
        input_events
    }
}
