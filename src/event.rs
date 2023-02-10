use crate::{
    bag::{Bag, Item},
    engine_structures::{
        components::MovingState,
        coordinate::{Coordinate, Direction},
    },
    font_manager::FontManager,
    humanoid::Humanoid,
    input::{self, Input, KeyState},
    menu::{
        self,
        bag_menu::BagMenu,
        menu_events::{MenuCommand, MenuInput},
        pause_menu::PauseMenu,
        textbox::Textbox,
    },
    object, player,
    render::{self, PIXELS_X},
};

pub enum Command {
    PlayerSprint(bool),
    PlayerMove(MovingState),
    PlayerInteract,
    InputMenu(MenuInput),
    OpenMenu(MenuCommand),
    GiveItem(Item, u32),
    DeleteObject(Coordinate),
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
        player: &mut player::Player,
        obj_man: &mut object::ObjectManager,
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
                PlayerSprint(sprinting) => {
                    if !menu_man.is_open() {
                        player.set_try_sprinting(sprinting)
                    }
                }
                PlayerMove(dir) => {
                    if !menu_man.is_open() {
                        //player.set_try_walking(dir)
                    } else {
                        player.set_try_walking(None);
                    }
                }
                PlayerInteract => {
                    let Coordinate(x, y) = player.get_pos();
                    let temp_pos = match player.get_facing() {
                        Direction::Left => Coordinate(x - 1.0, y),
                        Direction::Right => Coordinate(x + 1.0, y),
                        Direction::Up => Coordinate(x, y - 1.0),
                        Direction::Down => Coordinate(x, y + 1.0),
                    };

                    self.push_events(&mut obj_man.interact(temp_pos, player.get_pos()));
                    break; //this definitely is not a good permanent way to do this but it works for now
                }
                GiveItem(item, amount) => {
                    bag.add_item(item, amount);
                }
                DeleteObject(pos) => {
                    obj_man.remove_obj(pos);
                }
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
                        MenuCommand::Close => menu_man.close_menu(),
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

        let input = if input.pressed_controls[Menu] == KeyState::Pressed {
            Some(Command::InputMenu(MenuInput::Start))
        } else if input.pressed_controls[Interact1] == KeyState::Pressed {
            Some(Command::InputMenu(MenuInput::Accept))
        } else if input.pressed_controls[Interact2] == KeyState::Pressed {
            Some(Command::InputMenu(MenuInput::Reject))
        } else if input.pressed_controls[Left] == KeyState::Pressed {
            Some(Command::InputMenu(MenuInput::Left))
        } else if input.pressed_controls[Right] == KeyState::Pressed {
            Some(Command::InputMenu(MenuInput::Right))
        } else if input.pressed_controls[Up] == KeyState::Pressed {
            Some(Command::InputMenu(MenuInput::Up))
        } else if input.pressed_controls[Down] == KeyState::Pressed {
            Some(Command::InputMenu(MenuInput::Down))
        } else {
            None
        };

        if input.is_some() {
            self.push_event(input.unwrap());
        }
    }

    pub fn handle_input_gameplay(&mut self, input: &mut Input) {
        use input::Control::*;

        if input.pressed_controls[Menu] == KeyState::Pressed {
            self.push_event(Command::InputMenu(MenuInput::Start));
        }

        if input.pressed_controls[Interact1] == KeyState::Pressed {
            self.push_event(Command::PlayerInteract);
        }

        self.push_event(match input.pressed_controls[Interact2] {
            KeyState::Pressed | KeyState::Held => Command::PlayerSprint(true),
            KeyState::Released => Command::PlayerSprint(false),
        });

        if input.pressed_controls[Up] != KeyState::Released
            && input.pressed_controls[Down] == KeyState::Released
        {
            self.push_event(Command::PlayerMove(MovingState::Moving(Direction::Up)))
        } else if input.pressed_controls[Down] != KeyState::Released
            && input.pressed_controls[Up] == KeyState::Released
        {
            self.push_event(Command::PlayerMove(MovingState::Moving(Direction::Down)))
        } else if input.pressed_controls[Left] != KeyState::Released
            && input.pressed_controls[Right] == KeyState::Released
        {
            self.push_event(Command::PlayerMove(MovingState::Moving(Direction::Left)))
        } else if input.pressed_controls[Right] != KeyState::Released
            && input.pressed_controls[Left] == KeyState::Released
        {
            self.push_event(Command::PlayerMove(MovingState::Moving(Direction::Right)))
        } else {
            self.push_event(Command::PlayerMove(MovingState::Idle))
        }
    }
}
