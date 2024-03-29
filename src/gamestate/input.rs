/****************************************************/
// Created by: Logan Schmalz
// Description: General input handling for gameplay and menus
/****************************************************/
use enum_map::Enum;
use sdl2::{
    event::{Event, WindowEvent},
    keyboard::Keycode,
    EventPump,
};

use crate::{
    components::MovingState,
    font_manager::FontManager,
    gamestate::battle::BattleState,
    menu::{menu_events::MenuInput, pause_menu::PauseMenu},
    render::Renderer,
    vec2::Direction,
};

use super::{Screen, State};

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
    //this function turns control keys into what they are supposed to do in the game
    //takes in a keycode (the key pressed by the user)
    //returns the control option (the change the player is making)
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

    //this function updates the input
    //also has some special handling for window events (e.g. quitting the game)
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

    //this updates menus based on the current state of the input
    //takes in a font manager
    //returns true if the input is valid
    pub fn handle_input_menus(&mut self, font_manager: &FontManager) -> bool {
        use Control::*;
        use KeyState::*;

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

    //this updates gameplay based on the current state of the input
    //takes in a font manager
    pub fn handle_input_gameplay(&mut self, font_man: &FontManager) {
        use Control::*;
        use KeyState::*;

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

    //this updates battles based on the current state of the input
    //takes in a font manager
    pub fn handle_input_battle(&mut self, font_manager: &FontManager) {
        use Control::*;
        use KeyState::*;

        let Screen::Battle(battle) = &mut self.screen else { panic!() };

        //first check if a menu is open while battling and do those interactions
        if battle.menus.is_open() {
            let close = if self.input[Interact1] == Pressed {
                battle.menus.interact(
                    MenuInput::Accept,
                    &mut self.world,
                    font_manager,
                    &mut self.events,
                )
            } else if self.input[Interact2] == Pressed {
                battle.menus.interact(
                    MenuInput::Reject,
                    &mut self.world,
                    font_manager,
                    &mut self.events,
                )
            } else if self.input[Left] == Pressed {
                battle.menus.interact(
                    MenuInput::Left,
                    &mut self.world,
                    font_manager,
                    &mut self.events,
                )
            } else if self.input[Right] == Pressed {
                battle.menus.interact(
                    MenuInput::Right,
                    &mut self.world,
                    font_manager,
                    &mut self.events,
                )
            } else if self.input[Up] == Pressed {
                battle.menus.interact(
                    MenuInput::Up,
                    &mut self.world,
                    font_manager,
                    &mut self.events,
                )
            } else if self.input[Down] == Pressed {
                battle.menus.interact(
                    MenuInput::Down,
                    &mut self.world,
                    font_manager,
                    &mut self.events,
                )
            } else {
                false
            };

            if close {
                battle.menus.close_menu();
            }
        }

        //special handling for if the battle is in a state where the player is selecting a stray
        if matches!(
            battle.battle_state,
            BattleState::SelectingOpponentStray | BattleState::SelectingFriendlyStray
        ) {
            if self.input[Interact1] == Pressed {
                if let Some(index) = battle.selected_stray {
                    println!("attacked {:?}", &battle.selected_stray);
                    self.events
                        .push(crate::gamestate::event::Event::AttackStray(index));
                }
            } else if self.input[Left] == Pressed {
                match battle.battle_state {
                    BattleState::SelectingOpponentStray => {
                        battle.selected_stray =
                            battle.get_left_opponent_stray(battle.selected_stray)
                    }
                    BattleState::SelectingFriendlyStray => {
                        battle.selected_stray = battle.get_left_player_stray(battle.selected_stray)
                    }
                    _ => {}
                }
            } else if self.input[Right] == Pressed {
                match battle.battle_state {
                    BattleState::SelectingOpponentStray => {
                        battle.selected_stray =
                            battle.get_right_opponent_stray(battle.selected_stray)
                    }
                    BattleState::SelectingFriendlyStray => {
                        battle.selected_stray = battle.get_right_player_stray(battle.selected_stray)
                    }
                    _ => {}
                }
            }
        }
    }
}
