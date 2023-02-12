use std::{collections::HashSet, hash::Hash};

use hecs::{Entity, World};
use sdl2::{rect::Rect, video::WindowContext};

use crate::{
    bag::Bag,
    engine_structures::{
        collision::{self, Collision},
        components::*,
        coordinate::{Coordinate, Direction},
    },
    event::{Command, EventManager},
    font_manager::FontManager,
    humanoid::{self, ROTATION_TIME},
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
    TILE_SIZE,
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
    pub collisions: HashSet<usize>,
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

        /*let _npc = world.spawn((
            Position(Coordinate(4f32, 4f32)),
            MovingEntity {
                moving: MovingState::Moving(Direction::Left),
                ..Default::default()
            },
            Sprite {
                texture: String::from("assets/char-sprites/sikasprite.png"),
                src: Rect::new(0, 0, 16, 20),
                shift_x: 0,
                shift_y: -8,
            },
            Collision,
        ));*/

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
            collisions: HashSet::new(),
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
        delta_time: &f32,
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
        delta_time: &f32,
        input: &mut Input,
        _map: &mut TileMap,
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
                        MenuCommand::OpenTextbox(text_in) =>
                        /*self
                        .menus
                        .open_menu(Textbox::new(text_in, font_manager, PIXELS_X).into())*/
                        {
                            ()
                        }
                        MenuCommand::OpenPauseMenu => {
                            self.menus.open_menu(PauseMenu::new().into());
                        }
                    };
                }
                Command::GiveItem(item, amount) => self.bag.add_item(item, amount),
                Command::DeleteObject(_) => todo!(),
                Command::ChangeMap(_, _) => todo!(),
                Command::DrawTransition => todo!(),
            }
        }

        self.update_moving_objects(delta_time);
        self.update_collisions();

        Ok(())
    }

    pub fn update_collisions(&mut self) {
        let mut collision_query = self.world.query::<(&mut Position, &Collision)>();

        self.collisions = HashSet::new();
        self.collisions.reserve(collision_query.iter().len());

        for (_, (Position(c), _)) in collision_query.iter() {
            self.collisions.insert(c.to_usize(self.map.size_x));
        }
    }

    pub fn check_collision(&self, position: &Coordinate) -> bool {
        self.map.check_collision(*position) == collision::Collision(true)
            || self
                .collisions
                .contains(&position.to_usize(self.map.size_x))
            || position.0 < 0f32
            || position.0 > self.map.size_x as f32
            || position.1 < 0f32
            || position.0 > self.map.size_y as f32
    }

    pub fn update_player_moving_direction(
        &mut self,
        moving_state: MovingState,
    ) -> Result<(), String> {
        let (_, mut moving) = self
            .world
            .query_one_mut::<(&Player, &mut MovingEntity)>(self.player)
            .expect("No player found");

        if moving.moving == MovingState::Idle {
            if let MovingState::Moving(dir) = moving_state {
                if moving_state != moving.try_moving && moving.rotation != dir {
                    moving.rotation = dir;
                    moving.rotation_timer = ROTATION_TIME;
                }
            }
        }
        moving.try_moving = moving_state;

        Ok(())
    }

    pub fn update_player_sprinting(&mut self, sprinting: bool) -> Result<(), String> {
        let (_, mut moving) = self
            .world
            .query_one_mut::<(&Player, &mut MovingEntity)>(self.player)
            .expect("No player found");

        if moving.moving == MovingState::Idle {
            moving.sprinting = sprinting;
        }

        Ok(())
    }

    pub fn try_player_interaction(&mut self) -> Result<(), String> {
        let (_, &Position(Coordinate(x, y)), moving) = self
            .world
            .query_one_mut::<(&Player, &Position, &MovingEntity)>(self.player)
            .expect("No player found");

        let temp_pos = match moving.rotation {
            Direction::Left => Coordinate(x - 1.0, y),
            Direction::Right => Coordinate(x + 1.0, y),
            Direction::Up => Coordinate(x, y - 1.0),
            Direction::Down => Coordinate(x, y + 1.0),
        };

        let mut interact_query = self.world.query::<(&Position,)>();

        let interact_entity = interact_query
            .iter()
            .filter(|(_, (&Position(c),))| c == temp_pos)
            .next();

        match interact_entity {
            Some(_) => (),
            None => (),
        }

        //self.events
        //    .push_events(&mut obj_man.interact(temp_pos, player.get_pos()));

        Ok(())
    }

    pub fn update_moving_objects(&self, delta_time: &f32) {
        let mut moving_query = self.world.query::<(&mut Position, &mut MovingEntity)>();
        for (_, (pos, moving)) in moving_query.iter() {
            if moving.rotation_timer >= 0.0 {
                moving.rotation_timer -= delta_time;
            }

            if moving.moving != MovingState::Idle
                || moving.try_moving != MovingState::Idle && moving.rotation_timer <= 0.0
            {
                let &mut Position(Coordinate(x, y)) = pos;

                if moving.moving == MovingState::Idle {
                    moving.moving = moving.try_moving;
                }

                let Coordinate(target_x, target_y) = match moving.moving {
                    MovingState::Moving(Direction::Left) => Coordinate((x - 1.0).ceil(), y),
                    MovingState::Moving(Direction::Right) => Coordinate((x + 1.0).floor(), y),
                    MovingState::Moving(Direction::Up) => Coordinate(x, (y - 1.0).ceil()),
                    MovingState::Moving(Direction::Down) => Coordinate(x, (y + 1.0).floor()),
                    _ => Coordinate(x, y),
                };

                //hopefully this condition works on moving objects but unsure
                if (x, y) == (target_x, target_y)
                    || self.check_collision(&(target_x, target_y).into())
                        && Coordinate(x, y) == Coordinate(x, y).round_to_tile()
                {
                    moving.moving = MovingState::Idle;
                    continue;
                }

                let speed = if moving.sprinting {
                    humanoid::RUN_SPEED
                } else {
                    humanoid::WALK_SPEED
                };

                //compute direction (non-normalized vector)
                let dx = target_x - x;
                let dy = target_y - y;
                //compute move distance (signum normalizes)
                let mx = if dx != 0.0 {
                    speed * delta_time * dx.signum() / TILE_SIZE as f32
                } else {
                    0.0
                };
                let my = if dy != 0.0 {
                    speed * delta_time * dy.signum() / TILE_SIZE as f32
                } else {
                    0.0
                };
                //set new position
                *pos = Position(Coordinate(x + mx, y + my));

                let &mut Position(Coordinate(x, y)) = pos;

                //check if we have passed the tile we were trying to get to
                if (x, y) == (target_x, target_y)
                    || dx != 0.0 && (target_x - x).signum() != dx.signum()
                    || dy != 0.0 && (target_y - y).signum() != dy.signum()
                {
                    *pos = Position(Coordinate(target_x, target_y));
                    moving.moving = MovingState::Idle;
                }
            }
        }
    }
}
