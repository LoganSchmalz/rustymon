use hecs::World;
use sdl2::{rect::Rect, video::WindowContext};

use crate::{
    bag::Bag,
    engine_structures::{
        components::*,
        coordinate::{Coordinate, Direction},
    },
    event::{Command, EventManager},
    font_manager::FontManager,
    humanoid::{self, ROTATION_TIME},
    input::Input,
    menu::{
        main_menu::{MainMenu, MainMenuButton},
        MenuManager,
    },
    render::Renderer,
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
}

impl Default for State {
    fn default() -> Self {
        let mut world = World::new();
        let _player = world.spawn((
            Player,
            Position(Coordinate(2f32, 1f32)),
            MovingEntity::new(Coordinate(2f32, 1f32)),
            Sprite {
                texture: String::from("assets/char-sprites/augosprite.png"),
                src: Rect::new(0, 0, 16, 20),
                shift_x: 0,
                shift_y: -8,
            },
        ));

        let _door = world.spawn((
            Position(Coordinate(2f32, 0f32)),
            Sprite {
                texture: String::from("assets/tiles/tilesprites.png"),
                src: Rect::new(96, 0, 16, 16),
                ..Default::default()
            },
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
        }
    }
}

impl State {
    pub fn render(
        &self,
        renderer: &mut Renderer,
        //_canvas: &Canvas<Window>,
        texture_manager: &mut TextureManager<WindowContext>,
        font_man: &FontManager,
        delta_time: &f32,
        map: &mut TileMap,
        menu_man: &mut MenuManager,
    ) -> Result<(), String> {
        match &self.screen {
            Screen::MainMenu(menu) => renderer.render_main_menu(menu, texture_manager, font_man)?,
            Screen::Gameplay => renderer.render_world(
                texture_manager,
                font_man,
                delta_time,
                &self.world,
                map,
                menu_man,
            )?,
        };
        Ok(())
    }

    pub fn update(&mut self, delta_time: &f32, input: &mut Input, _map: &mut TileMap) -> Result<(), String> {
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
                Command::PlayerSprint(_) => (),
                Command::PlayerMove(moving_state) => {
                    self.update_player_moving_direction(moving_state)?
                }
                Command::PlayerInteract => todo!(),
                Command::InputMenu(_) => todo!(),
                Command::OpenMenu(_) => todo!(),
                Command::GiveItem(_, _) => todo!(),
                Command::DeleteObject(_) => todo!(),
                Command::ChangeMap(_, _) => todo!(),
                Command::DrawTransition => todo!(),
            }
        }

        self.update_moving_objects(delta_time);

        Ok(())
    }

    pub fn update_player_moving_direction(
        &mut self,
        moving_state: MovingState,
    ) -> Result<(), String> {
        let mut player_query = self
            .world
            .query::<(&Player, &Position, &mut MovingEntity)>();
        let (_, (_player, Position(Coordinate(x, y)), moving)) =
            player_query.iter().next().ok_or("No player found")?;

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

                if (x, y) == (target_x, target_y) {
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
                    *pos = Position((Coordinate(target_x, target_y)));
                    moving.moving = MovingState::Idle;
                }
            }
        }
    }
}
