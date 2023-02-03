use enum_map::{enum_map, Enum, EnumMap};
use sdl2::{
    event::{Event, WindowEvent},
    keyboard::Keycode,
};

pub struct Input {
    pressed_controls: EnumMap<Control, bool>,
}

pub enum InputEvent {
    ExitGame,
    ToggleFullscreen,
    ResizeWindow(i32, i32),
    Pressed(Control),
    Held(Control),
    Released(Control),
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
                Up => false,
                Down  => false,
                Left  => false,
                Right  => false,
                Interact1  => false,
                Interact2  => false,
                Menu  => false,
            },
        }
    }

    pub fn get_control(&self, key: Keycode) -> Option<Control> {
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

    pub fn handle_input(&mut self, event_pump: &mut sdl2::EventPump) -> Vec<InputEvent> {
        use InputEvent::*;

        let mut input_events = vec![];

        for event in event_pump.poll_iter() {
            match event {
                Event::Window {
                    win_event: WindowEvent::Resized(width, height),
                    ..
                } => input_events.push(ResizeWindow(width, height)),
                Event::Quit { .. } => input_events.push(ExitGame),
                Event::KeyDown {
                    keycode: Some(Keycode::F11),
                    ..
                } => input_events.push(ToggleFullscreen),
                Event::KeyDown {
                    keycode: Some(key),
                    repeat: false,
                    ..
                } => match self.get_control(key) {
                    Some(control) => {
                        self.pressed_controls[control] = true;
                        input_events.push(Pressed(control));
                    }
                    None => (),
                },
                Event::KeyUp {
                    keycode: Some(key), ..
                } => match self.get_control(key) {
                    Some(control) => {
                        self.pressed_controls[control] = false;
                        input_events.push(Released(control));
                    }
                    None => (),
                },
                _ => {}
            }
        }

        self.pressed_controls
            .iter()
            .filter(|(_, b)| **b == true)
            .for_each(|(c, _)| input_events.push(Held(c)));

        input_events
    }
}
