use std::collections::HashMap;

use hecs::{Entity, World};
use sdl2::{rect::Rect, video::WindowContext};

use crate::{
    humanoid_properties::{
        ROTATION_TIME, RUNNING_TIME_PER_TILE, WALKING_TIME_PER_TILE,
    },
    bag::Bag,
    engine_structures::{
        collision,
        components::*,
        coordinate::{compute_direction, Coordinate, Direction}, humanoid_properties,
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
        MenuManager,
    },
    render::{Renderer, PIXELS_X},
    resource_manager::TextureManager,
    tilemap::TileMap,
};

pub enum Screen {
    MainMenu(MainMenu),
    Gameplay,
}

pub struct State {
    pub screen: Screen,
    pub allow_input: bool,
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
            Sprite {
                texture: String::from("assets/char-sprites/augosprite.png"),
                src: Rect::new(0, 0, 16, 20),
                shift_x: 0,
                shift_y: -8,
            },
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
                ..Default::default()
            },
            Sprite {
                texture: String::from("assets/char-sprites/dadsprite.png"),
                src: Rect::new(0, 0, 16, 20),
                shift_x: 0,
                shift_y: -8,
            },
            Collision,
            Interactions(vec![Command::OpenMenu(MenuCommand::OpenTextbox(
                "Hi hungry, I'm dad! Nice try, little child --> you are bad!".to_string(),
            ))]),
        ));

        //let npc1 = world.spawn(())

        Self {
            screen: Screen::MainMenu(MainMenu {
                curr_button: MainMenuButton::StartButton,
            }),
            allow_input: true,
            world,
            events: EventManager::new(),
            bag: Bag::new(),
            map: TileMap::load(0),
            menus: MenuManager::new(),
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
        match &self.screen {
            Screen::MainMenu(menu) => renderer.render_main_menu(menu, texture_manager, font_man)?,
            Screen::Gameplay => renderer.render_world(
                texture_manager,
                font_man,
                delta_time,
                &self.world,
                map,
                &mut self.menus,
            )?,
        };
        Ok(())
    }

    pub fn update(
        &mut self,
        delta_time: f32,
        input: &mut Input,
        _map: &mut TileMap,
        font_man: &FontManager,
    ) -> Result<(), String> {
        match self.screen {
            Screen::Gameplay => {
                if self.menus.is_open() {
                    self.events.handle_input_menus(input);
                } else {
                    self.events.handle_input_gameplay(input);
                }
            }
            _ => (),
        }

        while let Some(command) = self.events.commands.pop() {
            match command {
                Command::PlayerSprint(sprinting) => self.update_player_sprinting(sprinting)?,
                Command::PlayerMove(ms) => self.update_player_moving_direction(ms)?,
                Command::PlayerInteract => self.try_player_interaction()?,
                Command::InputMenu(action) => self.menus.interact(action, self.bag.items.clone()),
                Command::OpenMenu(menu_event) => {
                    match menu_event {
                        MenuCommand::OpenStrays => todo!(),
                        MenuCommand::OpenBag => self
                            .menus
                            .open_menu(BagMenu::new(self.bag.items.clone()).into()),
                        MenuCommand::OpenSave => todo!(),
                        MenuCommand::Close => self.menus.close_menu(),
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

        self.update_moving_objects(delta_time);
        self.update_collisions();
        self.update_animations(delta_time);

        Ok(())
    }

    pub fn update_animations(&mut self, delta_time: f32) {
        let mut animation_query = self.world.query::<&mut HumanWalkAnimation>();

        for (_, anim) in animation_query.iter() {
            anim.update(delta_time);
        }
    }

    pub fn update_collisions(&mut self) {
        let mut collision_query = self.world.query::<(&mut Position, &Collision)>();

        self.collisions = HashMap::new();
        //self.collisions.reserve(collision_query.iter().len());

        for (entity, (Position(c), _)) in collision_query.iter() {
            self.collisions.insert(c.to_usize(self.map.size_x), entity);
        }
    }

    pub fn check_collision(&self, position: &Coordinate) -> bool {
        self.map.check_collision(*position) == collision::Collision(true)
            || self
                .collisions
                .contains_key(&position.to_usize(self.map.size_x))
            || position.0 < 0f32
            || position.0 >= self.map.size_x as f32
            || position.1 < 0f32
            || position.1 >= self.map.size_y as f32
    }

    pub fn update_player_moving_direction(
        &mut self,
        moving_state: MovingState,
    ) -> Result<(), String> {
        let (_, mut moving, animation) = self
            .world
            .query_one_mut::<(&Player, &mut MovingEntity, &mut HumanWalkAnimation)>(self.player)
            .or(Err("No player found"))?;

        match moving_state {
            MovingState::Moving(dir) => {
                if moving.moving == MovingState::Idle && dir != moving.rotation {
                    moving.rotation = dir;
                    animation.play_animation(ROTATION_TIME, dir);
                    moving.rotation_timer = 0.0;
                } else if moving.moving == MovingState::CenterTile {
                    moving.rotation = dir;
                }
            }
            _ => (),
        }

        moving.try_moving = moving_state;

        if moving.try_moving != MovingState::Idle
            && !animation.is_playing()
            && (moving.moving == MovingState::Idle || moving.moving == MovingState::CenterTile)
        {
            let rotation = if let MovingState::Moving(dir) = moving_state {
                dir
            } else {
                Direction::Down
            };
            match moving.sprinting {
                true => animation.play_animation(RUNNING_TIME_PER_TILE, rotation),
                false => animation.play_animation(WALKING_TIME_PER_TILE, rotation),
            }
        }

        Ok(())
    }

    pub fn update_player_sprinting(&mut self, sprinting: bool) -> Result<(), String> {
        let (_, mut moving, animation) = self
            .world
            .query_one_mut::<(&Player, &mut MovingEntity, &mut HumanWalkAnimation)>(self.player)
            .or(Err("No player found"))?;

        if moving.moving == MovingState::Idle
            || moving.moving == MovingState::CenterTile && moving.rotation_timer >= ROTATION_TIME
        {
            moving.sprinting = sprinting;
            match sprinting {
                true => animation.set_animation_time(RUNNING_TIME_PER_TILE),
                false => animation.set_animation_time(WALKING_TIME_PER_TILE),
            }
        }

        Ok(())
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

    pub fn update_moving_objects(&self, delta_time: f32) {
        let mut moving_query = self.world.query::<(&mut Position, &mut MovingEntity)>();
        for (_, (pos, moving)) in moving_query.iter() {
            if moving.rotation_timer < ROTATION_TIME {
                moving.rotation_timer += delta_time;
                continue;
            }

            if moving.moving == MovingState::CenterTile {
                moving.moving = MovingState::Idle;
            }

            if moving.moving == MovingState::Idle && moving.try_moving == MovingState::Idle {
                continue;
            }

            let &mut Position(Coordinate(x, y)) = pos;

            if moving.moving == MovingState::Idle {
                moving.moving = moving.try_moving;
            }

            let Coordinate(target_x, target_y) = match moving.moving {
                MovingState::Moving(Direction::Left) => Coordinate((x - 1.0).ceil(), y),
                MovingState::Moving(Direction::Right) => Coordinate((x + 1.0).floor(), y),
                MovingState::Moving(Direction::Up) => Coordinate(x, (y - 1.0).ceil()),
                MovingState::Moving(Direction::Down) => Coordinate(x, (y + 1.0).floor()),
                _ => panic!("Should not happen"),
            };

            if self.check_collision(&(target_x, target_y).into())
                && Coordinate(x, y) == Coordinate(x, y).round_to_tile()
            {
                moving.moving = MovingState::CenterTile;
                continue;
            }

            let speed = if moving.sprinting {
                humanoid_properties::RUN_SPEED
            } else {
                humanoid_properties::WALK_SPEED
            };

            let speed = speed * delta_time;

            let (dx, dy) = match compute_direction(Coordinate(x, y), Coordinate(target_x, target_y))
            {
                Direction::Up => (0.0, -speed),
                Direction::Down => (0.0, speed),
                Direction::Left => (-speed, 0.0),
                Direction::Right => (speed, 0.0),
            };

            //set new position
            *pos = Position(Coordinate(x + dx, y + dy));

            let &mut Position(Coordinate(x, y)) = pos;

            //check if we have passed the tile we were trying to get to
            if (x, y) == (target_x, target_y)
                || dx != 0.0 && (target_x - x).signum() != dx.signum()
                || dy != 0.0 && (target_y - y).signum() != dy.signum()
            {
                *pos = Position(Coordinate(target_x, target_y));
                moving.moving = MovingState::CenterTile;
            }
        }
    }
}
