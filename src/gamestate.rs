use std::collections::HashMap;

use hecs::{Entity, World};
use sdl2::{rect::Rect, video::WindowContext};

use crate::{
    bag::Bag,
    engine_structures::{
        components::*,
        coordinate::{Coordinate, Direction},
    },
    event::{Command, EventManager},
    font_manager::FontManager,
    input::Input,
    menu::{
        bag_menu::BagMenu,
        main_menu::{MainMenu, MainMenuButton},
        menu_events::MenuCommand,
        pause_menu::PauseMenu,
        textbox::Textbox,
        MenuItem, MenuManager,
    },
    render::{Renderer, PIXELS_X},
    resource_manager::TextureManager,
    tilemap::TileMap,
};

mod updates;

pub struct State {
    pub allow_input: bool,
    pub paused: bool,
    pub world: World,
    pub events: EventManager,
    pub bag: Bag,
    pub map: TileMap,
    pub menus: MenuManager,
    pub player: Entity,
    pub collisions: HashMap<usize, Entity>,
}

impl Default for State {
    fn default() -> Self {
        let mut world = World::new();
        let player = world.spawn((
            Player,
            Position(Coordinate(2f32, 1f32)),
            MovingEntity::new(Coordinate(2f32, 1f32)),
            Sprite::character(String::from("assets/char-sprites/augosprite.png")),
            Collision,
            HumanWalkAnimation {
                rotation: Direction::Down,
                time: (1.0, 0.0),
                left_leg: true,
            },
        ));

        let _door = world.spawn((
            Position(Coordinate(2f32, 0f32)),
            Sprite {
                texture: String::from("assets/tiles/tilesprites.png"),
                src: Rect::new(96, 0, 16, 16),
                ..Default::default()
            },
            Collision,
        ));

        let _npc = world.spawn((
            Position(Coordinate(4f32, 4f32)),
            MovingEntity {
                moving: MovingState::Moving(Direction::Left),
                try_moving: MovingState::Moving(Direction::Left),
                rotation: Direction::Left,
                ..Default::default()
            },
            Sprite::character(String::from("assets/char-sprites/dadsprite.png")),
            Collision,
            Interactions(vec![Command::OpenMenu(MenuCommand::OpenTextbox(
                "Hi hungry, I'm dad! Nice try, little child --> you are bad!".to_string(),
            ))]),
            HumanWalkAnimation {
                rotation: Direction::Left,
                time: (1.0, 0.0),
                left_leg: true,
            },
        ));

        let _berry = world.spawn((
            Position(Coordinate(10f32, 8f32)),
            Sprite::berry(),
            Collision,
        ));

        let mut menus = MenuManager::new();
        menus.open_menu(MainMenu::new().into());

        Self {
            allow_input: true,
            paused: true,
            world,
            events: EventManager::new(),
            bag: Bag::new(),
            map: TileMap::load(0),
            menus,
            player,
            collisions: HashMap::new(),
        }
    }
}

impl State {
    pub fn render(
        &mut self,
        renderer: &mut Renderer,
        //_canvas: &Canvas<Window>,
        texture_manager: &mut TextureManager<WindowContext>,
        font_man: &FontManager,
        delta_time: f32,
        map: &mut TileMap,
    ) -> Result<(), String> {
        renderer.render(
            texture_manager,
            font_man,
            delta_time,
            &self.world,
            map,
            &mut self.menus,
        )?;
        Ok(())
    }

    pub fn update(
        &mut self,
        delta_time: f32,
        input: &mut Input,
        _map: &mut TileMap,
        font_man: &FontManager,
    ) -> Result<(), String> {
        if self.menus.is_open() {
            self.events.handle_input_menus(input);
        } else {
            self.events.handle_input_gameplay(input);
        }

        while let Some(command) = self.events.commands.pop() {
            match command {
                Command::PlayerSprint(sprinting) => self.update_player_sprinting(sprinting)?,
                Command::PlayerMove(ms) => self.update_player_moving_direction(ms)?,
                Command::PlayerInteract => self.try_player_interaction()?,
                Command::InputMenu(action) => self.paused = self.menus.interact(action, self.bag.items.clone()),
                Command::OpenMenu(menu_event) => {
                    match menu_event {
                        MenuCommand::OpenStrays => todo!(),
                        MenuCommand::OpenBag => self
                            .menus
                            .open_menu(BagMenu::new(self.bag.items.clone()).into()),
                        MenuCommand::OpenSave => todo!(),
                        MenuCommand::Close => self.paused = self.menus.close_menu(),
                        MenuCommand::OpenTextbox(text_in) => self
                            .menus
                            .open_menu(Textbox::new(text_in, font_man, PIXELS_X).into()),
                        MenuCommand::OpenPauseMenu => {
                            self.menus.open_menu(PauseMenu::new().into());
                        }
                    };
                }
                Command::GiveItem(item, amount) => self.bag.add_item(item, amount),
                Command::DeleteObject(id) => self
                    .world
                    .despawn(id)
                    .or(Err("Tried to delete nonexistent entity"))?,
                Command::ChangeMap(_, _) => todo!(),
                Command::DrawTransition => todo!(),
            }
        }

        if !self.paused {
            self.update_moving_objects(delta_time);
            self.update_collisions();
            self.update_animations(delta_time);
        }
        Ok(())
    }

    pub fn check_collision(&self, position: &Coordinate) -> bool {
        self.map.check_collision(*position)
            || self
                .collisions
                .contains_key(&position.to_usize(self.map.size_x))
            || position.0 < 0f32
            || position.0 >= self.map.size_x as f32
            || position.1 < 0f32
            || position.1 >= self.map.size_y as f32
    }

    pub fn try_player_interaction(&mut self) -> Result<(), String> {
        let (_, &Position(Coordinate(x, y)), moving) = self
            .world
            .query_one_mut::<(&Player, &Position, &MovingEntity)>(self.player)
            .or(Err("No player found"))?;

        if moving.moving != MovingState::CenterTile && moving.moving != MovingState::Idle {
            return Ok(());
        }

        let temp_pos = match moving.rotation {
            Direction::Left => Coordinate(x - 1.0, y),
            Direction::Right => Coordinate(x + 1.0, y),
            Direction::Up => Coordinate(x, y - 1.0),
            Direction::Down => Coordinate(x, y + 1.0),
        };

        let interact_entity = self
            .collisions
            .get_key_value(&temp_pos.to_usize(self.map.size_x));

        match interact_entity {
            Some((_, &entity)) => {
                let interactions = self.world.query_one_mut::<&Interactions>(entity);
                match interactions {
                    Ok(Interactions(list)) => {
                        self.events.push_events(&mut list.clone());
                    }
                    _ => (),
                }
            }
            None => (),
        }

        Ok(())
    }
}
