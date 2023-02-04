use crate::{
    bag::{Bag, Item},
    engine_structures::coordinate::{Coordinate, Direction},
    humanoid::Humanoid,
    input::{self, Control, InputEvent},
    menu::{self, menu_events::MenuInput},
    object, player, render,
};

pub enum Command {
    PlayerSprint(bool),
    PlayerMove(Option<Direction>),
    PlayerInteract,
    Menu(MenuInput),
    GiveItem(Item, u32),
    DeleteObject(Coordinate),
}

type Exit = bool;

pub struct EventManager {
    commands: Vec<Command>,
}

impl EventManager {
    pub fn new() -> EventManager {
        EventManager { commands: vec![] }
    }

    pub fn push_event(&mut self, command: Command) {
        self.commands.push(command);
    }

    pub fn handle_input_event(
        &mut self,
        input_events: Vec<InputEvent>,
        menu_man: &mut menu::MenuManager,
        renderer: &mut render::Renderer,
    ) -> Exit {
        let mut result = false;

        for input in input_events {
            use input::InputEvent::*;

            //todo: fix player moving other events happen. probably requires more redesign

            match input {
                Pressed(control) => self.handle_control_pressed(control, menu_man),
                Held(control) => self.handle_control_held(control, menu_man),
                Released(control) => self.handle_control_released(control),
                ExitGame => result = true,
                ToggleFullscreen => renderer.toggle_fullscreen(),
                ResizeWindow(width, height) => renderer.resize(width, height),
            }
        }

        result
    }

    pub fn handle_gameplay_event(
        &mut self,
        menu_man: &mut menu::MenuManager,
        player: &mut player::Player,
        obj_man: &mut object::ObjectManager,
        renderer: &mut render::Renderer,
        bag: &mut Bag,
    ) {
        use Command::*;

        while let Some(command) = self.commands.pop() {
            match command {
                Menu(menu_input) => {
                    menu_man.interact(menu_input, bag.items.clone());
                    break;
                }
                PlayerSprint(sprinting) => player.set_try_sprinting(sprinting),
                PlayerMove(dir) => player.set_try_walking(dir),
                PlayerInteract => {
                    let Coordinate(x, y) = player.get_pos();
                    let temp_pos = match player.get_facing() {
                        Direction::Left => Coordinate(x - 1.0, y),
                        Direction::Right => Coordinate(x + 1.0, y),
                        Direction::Up => Coordinate(x, y - 1.0),
                        Direction::Down => Coordinate(x, y + 1.0),
                    };

                    self.commands.append(&mut obj_man.interact(
                        temp_pos,
                        player.get_pos(),
                        renderer,
                        menu_man,
                    ));
                    break; //this definitely is not a good permanent way to do this but it works for now
                }
                GiveItem(item, amount) => {
                    bag.add_item(item, amount);
                }
                DeleteObject(pos) => {
                    obj_man.remove_obj(pos);
                }
            }
        }
    }

    pub fn handle_control_pressed(&mut self, control: Control, menu_man: &mut menu::MenuManager) {
        use input::Control::*;

        let c = if menu_man.is_open() {
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
                Interact2 => Some(Command::PlayerSprint(true)),
                Menu => Some(Command::Menu(MenuInput::Start)),
            }
        };
        if c.is_some() {
            self.push_event(c.unwrap());
        }
    }

    pub fn handle_control_held(&mut self, control: Control, menu_man: &mut menu::MenuManager) {
        use input::Control::*;

        let c = if menu_man.is_open() {
            None
        } else {
            match control {
                Up => Some(Command::PlayerMove(Some(Direction::Up))),
                Down => Some(Command::PlayerMove(Some(Direction::Down))),
                Left => Some(Command::PlayerMove(Some(Direction::Left))),
                Right => Some(Command::PlayerMove(Some(Direction::Right))),
                Interact1 => None,
                Interact2 => Some(Command::PlayerSprint(true)),
                Menu => None,
            }
        };
        if c.is_some() {
            self.push_event(c.unwrap());
        }
    }

    pub fn handle_control_released(&mut self, control: Control) {
        use input::Control::*;
        let c = match control {
            Up => Some(Command::PlayerMove(None)),
            Down => Some(Command::PlayerMove(None)),
            Left => Some(Command::PlayerMove(None)),
            Right => Some(Command::PlayerMove(None)),
            Interact1 => None,
            Interact2 => Some(Command::PlayerSprint(false)),
            Menu => None,
        };
        if c.is_some() {
            self.push_event(c.unwrap());
        }
    }
}
