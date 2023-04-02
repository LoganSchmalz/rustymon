use enum_map::Enum;
use sdl2::{
    event::{Event, WindowEvent},
    keyboard::Keycode,
    EventPump,
};

use crate::{
    components::MovingState,
    font_manager::FontManager,
    menu::{menu_events::MenuInput, pause_menu::PauseMenu},
    render::Renderer,
    vec2::Direction,
};

use super::State;

#[derive(PartialEq, Copy, Clone, Default)]
pub enum KeyState {
    #[default]
    Released,
    Pressed,
    Held,
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

impl State {
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

    pub fn update_input(
        &mut self,
        event_pump: &mut EventPump,
        renderer: &mut Renderer,
    ) -> Result<bool, String> {
        for (control, state) in self.input {
            if self.allow_input {
                if state == KeyState::Pressed {
                    self.input[control] = KeyState::Held;
                }
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
                } => {
                    if let Some(control) = self.get_control(key) {
                        self.input[control] = KeyState::Pressed;
                    }
                }
                Event::KeyUp {
                    keycode: Some(key), ..
                } => {
                    if let Some(control) = self.get_control(key) {
                        self.input[control] = KeyState::Released;
                    }
                }
                _ => {}
            }
        }

        Ok(false)
    }

    pub fn handle_input_menus(&mut self, font_manager: &FontManager) -> bool {
        use Control::*;
        use KeyState::*;

        if !self.allow_input {
            return false;
        }

        if self.input[Menu] == Pressed {
            self.menus.interact(
                MenuInput::Start,
                &mut self.world,
                font_manager,
                &mut self.events,
            )
        } else if self.input[Interact1] == Pressed {
            self.menus.interact(
                MenuInput::Accept,
                &mut self.world,
                font_manager,
                &mut self.events,
            )
        } else if self.input[Interact2] == Pressed {
            self.menus.interact(
                MenuInput::Reject,
                &mut self.world,
                font_manager,
                &mut self.events,
            )
        } else if self.input[Left] == Pressed {
            self.menus.interact(
                MenuInput::Left,
                &mut self.world,
                font_manager,
                &mut self.events,
            )
        } else if self.input[Right] == Pressed {
            self.menus.interact(
                MenuInput::Right,
                &mut self.world,
                font_manager,
                &mut self.events,
            )
        } else if self.input[Up] == Pressed {
            self.menus.interact(
                MenuInput::Up,
                &mut self.world,
                font_manager,
                &mut self.events,
            )
        } else if self.input[Down] == Pressed {
            self.menus.interact(
                MenuInput::Down,
                &mut self.world,
                font_manager,
                &mut self.events,
            )
        } else {
            false
        }
    }

    pub fn handle_input_gameplay(&mut self, font_man: &FontManager) {
        use Control::*;
        use KeyState::*;

        if !self.allow_input {
            self.update_player_moving(MovingState::Idle);
            return;
        }

        if self.input[Menu] == Pressed {
            self.menus.open_menu(PauseMenu::new().into());
        }

        if self.input[Interact1] == Pressed {
            self.try_player_interaction(font_man);
        }

        self.update_player_sprinting(matches!(self.input[Interact2], Pressed | Held));

        if self.input[Up] != Released && self.input[Down] == Released {
            self.update_player_moving(MovingState::Moving(Direction::Up));
        } else if self.input[Down] != Released && self.input[Up] == Released {
            self.update_player_moving(MovingState::Moving(Direction::Down));
        } else if self.input[Left] != Released && self.input[Right] == Released {
            self.update_player_moving(MovingState::Moving(Direction::Left));
        } else if self.input[Right] != Released && self.input[Left] == KeyState::Released {
            self.update_player_moving(MovingState::Moving(Direction::Right));
        } else {
            self.update_player_moving(MovingState::Idle);
        }
    }
}
