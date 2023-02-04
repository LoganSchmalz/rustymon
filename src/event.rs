use std::{cell::RefCell, process::ExitCode, rc::Rc};

use crate::{
    bag::{ItemList},
    engine_structures::coordinate::{Coordinate, Direction},
    humanoid::Humanoid,
    input::{self, Control, InputEvent},
    menu::{
        self,
        menu_events::{MenuInput},
    },
    object, player, render,
};

pub enum Command {
    PlayerSprinting(bool),
    PlayerMove(Option<Direction>),
    PlayerInteract,
    Menu(MenuInput),
}

type Exit = bool;

pub fn handle_input_event(
    events: Vec<InputEvent>,
    menu_man: &mut menu::MenuManager,
    renderer: &mut render::Renderer,
) -> (Vec<Command>, Exit) {
    let mut result = (vec![], false);

    for event in events {
        use input::InputEvent::*;

        //todo: fix player moving other events happen. probably requires more redesign

        match event {
            Pressed(control) => match handle_control_pressed(control, menu_man) {
                Some(e) => result.0.push(e),
                None => (),
            },
            Held(control) => match handle_control_held(control, menu_man) {
                Some(e) => result.0.push(e),
                None => (),
            },
            Released(control) => match handle_control_released(control) {
                Some(e) => result.0.push(e),
                None => (),
            },
            ExitGame => result.1 = true,
            ToggleFullscreen => renderer.toggle_fullscreen(),
            ResizeWindow(width, height) => renderer.resize(width, height),
        }
    }

    result
}

pub fn handle_gameplay_event(
    events: Vec<Command>,
    menu_man: &mut menu::MenuManager,
    player: &mut player::Player,
    obj_man: &mut object::ObjectManager,
    renderer: &mut render::Renderer,
    items: ItemList,
) {
    use Command::*;

    for event in events {
        match event {
            Menu(menu_input) => {
                menu_man.interact(menu_input, items);
                break;
            }
            PlayerSprinting(sprinting) => player.set_try_sprinting(sprinting),
            PlayerMove(dir) => player.set_try_walking(dir),
            PlayerInteract => {
                let Coordinate(x, y) = player.get_pos();
                let temp_pos = match player.get_facing() {
                    Direction::Left => Coordinate(x - 1.0, y),
                    Direction::Right => Coordinate(x + 1.0, y),
                    Direction::Up => Coordinate(x, y - 1.0),
                    Direction::Down => Coordinate(x, y + 1.0),
                };

                //if obj_man.interact(temp_pos, player.get_pos(), renderer, menu_man, bag.clone()) {
                //    break;
                //}
            }
        }
    }
}

pub fn handle_control_pressed(
    control: Control,
    menu_man: &mut menu::MenuManager,
) -> Option<Command> {
    use input::Control::*;

    if menu_man.is_open() {
        match control {
            Up => Some(Command::Menu(MenuInput::Up)),
            Down => Some(Command::Menu(MenuInput::Down)),
            Left => Some(Command::Menu(MenuInput::Left)),
            Right => Some(Command::Menu(MenuInput::Right)),
            Interact1 => Some(Command::Menu(MenuInput::Accept)),
            Interact2 => Some(Command::Menu(MenuInput::Reject)),
            Menu => Some(Command::Menu(MenuInput::Start)),
        }
    } else {
        match control {
            Up => Some(Command::PlayerMove(Some(Direction::Up))),
            Down => Some(Command::PlayerMove(Some(Direction::Down))),
            Left => Some(Command::PlayerMove(Some(Direction::Left))),
            Right => Some(Command::PlayerMove(Some(Direction::Right))),
            Interact1 => Some(Command::PlayerInteract),
            Interact2 => Some(Command::PlayerSprinting(true)),
            Menu => Some(Command::Menu(MenuInput::Start)),
        }
    }
}

pub fn handle_control_held(
    control: Control,
    menu_man: &mut menu::MenuManager,
) -> Option<Command> {
    use input::Control::*;

    if menu_man.is_open() {
        None
    } else {
        match control {
            Up => Some(Command::PlayerMove(Some(Direction::Up))),
            Down => Some(Command::PlayerMove(Some(Direction::Down))),
            Left => Some(Command::PlayerMove(Some(Direction::Left))),
            Right => Some(Command::PlayerMove(Some(Direction::Right))),
            Interact1 => None,
            Interact2 => None,
            Menu => None,
        }
    }
}

pub fn handle_control_released(control: Control) -> Option<Command> {
    use input::Control::*;
    match control {
        Up => Some(Command::PlayerMove(None)),
        Down => Some(Command::PlayerMove(None)),
        Left => Some(Command::PlayerMove(None)),
        Right => Some(Command::PlayerMove(None)),
        Interact1 => None,
        Interact2 => Some(Command::PlayerSprinting(false)),
        Menu => None,
    }
}
