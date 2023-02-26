use enum_map::{enum_map, Enum, EnumMap};
use sdl2::{
    event::{Event, WindowEvent},
    keyboard::Keycode,
};

use crate::render::Renderer;

#[derive(PartialEq, Copy, Clone)]
pub enum KeyState {
    Released,
    Pressed,
    Held,
}
pub struct Input {
    pub pressed_controls: EnumMap<Control, KeyState>,
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Enum)]
pub enum Control {
    Up,
    Down,
    Left,
    Right,
    Interact1,
    Interact2,
    Menu,
}

impl Input {
    pub fn new() -> Input {
        use Control::*;

        Input {
            pressed_controls: enum_map! {
                Up => KeyState::Released,
                Down  => KeyState::Released,
                Left  => KeyState::Released,
                Right  => KeyState::Released,
                Interact1  => KeyState::Released,
                Interact2  => KeyState::Released,
                Menu  => KeyState::Released,
            },
        }
    }

    fn get_control(&mut self, key: Keycode) -> Option<Control> {
        use Control::*;

        match key {
            Keycode::Up => Some(Up),
            Keycode::Left => Some(Left),
            Keycode::Down => Some(Down),
            Keycode::Right => Some(Right),
            Keycode::Space => Some(Interact1),
            Keycode::Z => Some(Interact2),
            Keycode::Return => Some(Menu),
            _ => None,
        }
    }

    pub fn handle_events(
        &mut self,
        event_pump: &mut sdl2::EventPump,
        renderer: &mut Renderer,
    ) -> Result<bool, String> {
        for (control, state) in self.pressed_controls {
            if state == KeyState::Pressed {
                self.pressed_controls[control] = KeyState::Held
            }
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Window {
                    win_event: WindowEvent::Resized(width, height),
                    ..
                } => renderer.resize(width, height)?,
                Event::Quit { .. } => return Ok(true),
                Event::KeyDown {
                    keycode: Some(Keycode::F11),
                    ..
                } => renderer.toggle_fullscreen()?,
                Event::KeyDown {
                    keycode: Some(key),
                    repeat: false,
                    ..
                } => match self.get_control(key) {
                    Some(control) => {
                        self.pressed_controls[control] = KeyState::Pressed;
                    }
                    None => (),
                },
                Event::KeyUp {
                    keycode: Some(key), ..
                } => match self.get_control(key) {
                    Some(control) => {
                        self.pressed_controls[control] = KeyState::Released;
                    }
                    None => (),
                },
                _ => {}
            }
        }
        Ok(false)
    }
}
