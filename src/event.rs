use std::{process::ExitCode, rc::Rc, cell::RefCell};

use crate::{
    bag,
    engine_structures::coordinate::{Coordinate, Direction},
    humanoid::Humanoid,
    input, menu, object, player, render,
};

pub fn handle_event(
    input: &input::Input,
    event_pump: &mut sdl2::EventPump,
    menu_man: &mut menu::MenuManager,
    player: &mut player::Player,
    obj_man: &mut object::ObjectManager,
    renderer: &mut render::Renderer,
    bag: Rc<RefCell<bag::Bag>>,
) -> Option<ExitCode> {
    for event in input.handle_input(event_pump, menu_man) {
        use input::InputEvent::*;

        //todo: fix player moving other events happen. probably requires more redesign

        match event {
            MenuInteract(menu_input) => menu_man.interact(menu_input, bag.clone()),
            PlayerSprinting => player.set_try_sprinting(true),
            PlayerWalking => player.set_try_sprinting(false),
            PlayerMove(dir) => player.set_try_walking(dir),
            Interact => {
                let Coordinate(x, y) = player.get_pos();
                let temp_pos = match player.get_facing() {
                    Direction::Left => Coordinate(x - 1.0, y),
                    Direction::Right => Coordinate(x + 1.0, y),
                    Direction::Up => Coordinate(x, y - 1.0),
                    Direction::Down => Coordinate(x, y + 1.0),
                };

                obj_man.interact(temp_pos, player.get_pos(), renderer, menu_man, bag.clone());
            }
            ExitGame => return Some(ExitCode::from(0)),
            ToggleFullscreen => renderer.toggle_fullscreen(),
            ResizeWindow(width, height) => renderer.resize(width, height),
        }
    }
    None
}
