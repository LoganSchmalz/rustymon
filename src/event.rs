use hecs::Entity;

use crate::{
    bag::{Bag, Item},
    engine_structures::{
        components::MovingState,
        coordinate::{Coordinate, Direction},
    },
    font_manager::FontManager,
    input::{self, Input, KeyState},
    menu::{
        self,
        bag_menu::BagMenu,
        menu_events::{MenuCommand, MenuInput},
        pause_menu::PauseMenu,
        textbox::Textbox,
    },
    render::{self, PIXELS_X},
};

#[derive(Clone)]
pub enum Command {
    PlayerSprint(bool),
    PlayerMove(MovingState),
    PlayerInteract,
    InputMenu(MenuInput),
    OpenMenu(MenuCommand),
    GiveItem(Item, u32),
    DeleteObject(Entity),
    ChangeMap(usize, Coordinate),
    DrawTransition,
}

type Exit = bool;

pub struct EventManager {
    pub commands: Vec<Command>,
}

impl EventManager {
    pub fn new() -> EventManager {
        EventManager { commands: vec![] }
    }

    pub fn push_event(&mut self, command: Command) {
        self.commands.push(command);
    }

    pub fn push_events(&mut self, commands: &mut Vec<Command>) {
        self.commands.append(commands);
    }

    pub fn handle_gameplay_events(
        &mut self,
        menu_man: &mut menu::MenuManager,
        renderer: &mut render::Renderer,
        font_manager: &FontManager,
        bag: &mut Bag,
    ) {
        use Command::*;

        while let Some(command) = self.commands.pop() {
            match command {
                InputMenu(action) => {
                    menu_man.interact(action, bag.items.clone());
                }
                PlayerSprint(sprinting) => if !menu_man.is_open() {},
                PlayerMove(dir) => {}
                PlayerInteract => {}
                GiveItem(item, amount) => {
                    bag.add_item(item, amount);
                }
                DeleteObject(_) => {}
                ChangeMap(id, pos) => (),
                DrawTransition => {
                    renderer.play_fade();
                }
                OpenMenu(menu_event) => {
                    match menu_event {
                        MenuCommand::OpenStrays => todo!(),
                        MenuCommand::OpenBag => {
                            menu_man.open_menu(BagMenu::new(bag.items.clone()).into())
                        }
                        MenuCommand::OpenSave => todo!(),
                        MenuCommand::Close => {
                            menu_man.close_menu();
                        }
                        MenuCommand::OpenTextbox(text_in) => {
                            menu_man.open_menu(Textbox::new(text_in, font_manager, PIXELS_X).into())
                        }
                        MenuCommand::OpenPauseMenu => menu_man.open_menu(PauseMenu::new().into()),
                    };
                    break;
                }
            }
        }
    }

    pub fn handle_input_menus(&mut self, input: &mut Input) {
        use input::Control::*;
        use input::KeyState::*;

        let input = if input.pressed_controls[Menu] == Pressed {
            Some(Command::InputMenu(MenuInput::Start))
        } else if input.pressed_controls[Interact1] == Pressed {
            Some(Command::InputMenu(MenuInput::Accept))
        } else if input.pressed_controls[Interact2] == Pressed {
            Some(Command::InputMenu(MenuInput::Reject))
        } else if input.pressed_controls[Left] == Pressed {
            Some(Command::InputMenu(MenuInput::Left))
        } else if input.pressed_controls[Right] == Pressed {
            Some(Command::InputMenu(MenuInput::Right))
        } else if input.pressed_controls[Up] == Pressed {
            Some(Command::InputMenu(MenuInput::Up))
        } else if input.pressed_controls[Down] == Pressed {
            Some(Command::InputMenu(MenuInput::Down))
        } else {
            None
        };

        if let Some(input) = input {
            self.push_event(input);
        }
    }

    pub fn handle_input_gameplay(&mut self, input: &mut Input) {
        use input::Control::*;
        use input::KeyState::*;

        if input.pressed_controls[Menu] == Pressed {
            self.push_event(Command::InputMenu(MenuInput::Start));
        }

        if input.pressed_controls[Interact1] == Pressed {
            self.push_event(Command::PlayerInteract);
        }

        self.push_event(match input.pressed_controls[Interact2] {
            Pressed | Held => Command::PlayerSprint(true),
            Released => Command::PlayerSprint(false),
        });

        if input.pressed_controls[Up] != Released && input.pressed_controls[Down] == Released {
            self.push_event(Command::PlayerMove(MovingState::Moving(Direction::Up)))
        } else if input.pressed_controls[Down] != Released && input.pressed_controls[Up] == Released
        {
            self.push_event(Command::PlayerMove(MovingState::Moving(Direction::Down)))
        } else if input.pressed_controls[Left] != Released
            && input.pressed_controls[Right] == Released
        {
            self.push_event(Command::PlayerMove(MovingState::Moving(Direction::Left)))
        } else if input.pressed_controls[Right] != Released
            && input.pressed_controls[Left] == KeyState::Released
        {
            self.push_event(Command::PlayerMove(MovingState::Moving(Direction::Right)))
        } else {
            self.push_event(Command::PlayerMove(MovingState::Idle))
        }
    }
}
