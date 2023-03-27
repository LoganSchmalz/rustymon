use std::{clone::Clone, collections::HashMap};

use hecs::{CommandBuffer, Entity, World};
use rand::{distributions::Uniform, rngs::ThreadRng, Rng};
use sdl2::{rect::Rect, video::WindowContext};

use enum_map::EnumMap;

use crate::{
    components::{animation::HumanWalkAnimation, bag::Bag, sprite::Sprite, stray::*, *},
    constants::RANDOM_ENCOUNTER_CHANCE,
    font_manager::FontManager,
    menu::{main_menu::MainMenu, moves_menu::MovesMenu, textbox::Textbox, MenuManager},
    render::Renderer,
    resource_manager::TextureManager,
    tilemap::TileMap,
    vec2::{Direction, Vec2},
};

use self::{
    event::Event,
    input::{Control, KeyState},
};

pub mod event;
mod input;
mod updates;

#[derive(Clone)]
pub enum Screen {
    MainMenu,
    Overworld,
    Battle(Battle),
}

#[derive(Default, Clone)]
pub struct Battle {
    pub player_strays: [Option<Stray>; 4],
    pub opponent_strays: [Option<Stray>; 4],
}

pub struct State {
    pub screen: Screen,
    pub next_screen: Screen,
    pub input: EnumMap<Control, KeyState>,
    pub paused: bool,
    pub world: World,
    pub cmd: CommandBuffer,
    pub events: Vec<Event>,
    pub map: TileMap,
    pub menus: MenuManager,
    pub player: Entity,
    pub collisions: HashMap<usize, Entity>,
    pub rng: ThreadRng,
    pub transitioning: bool
}

impl Default for State {
    fn default() -> Self {
        let mut world = World::new();
        let mut cmd = CommandBuffer::new();

        //here we create a bunch of example entities for the default world
        let player = world.spawn((
            Player,
            Position(Vec2(14f32, 15f32)),
            MovingEntity::new(),
            Sprite::character(String::from("assets/char-sprites/augosprite.png")),
            Collision,
            HumanWalkAnimation {
                rotation: Direction::Down,
                time: (1.0, 0.0),
                left_leg: true,
                sprinting: false,
            },
            Bag::new(),
        ));

        let _door = world.spawn((
            Position(Vec2(2f32, 0f32)),
            Sprite {
                texture: String::from("assets/tiles/tilesprites.png"),
                src: Rect::new(96, 0, 16, 16),
                ..Default::default()
            },
            Collision,
        ));

        let _npc = world.spawn((
            Position(Vec2(16f32, 16f32)),
            MovingEntity {
                moving: MovingState::Moving(Direction::Left),
                try_moving: MovingState::Moving(Direction::Left),
                rotation: Direction::Left,
                ..Default::default()
            },
            Sprite::character(String::from("assets/char-sprites/dadsprite.png")),
            Collision,
            Npc {
                says: "Hi hungry, I'm dad! Nice try, little child --> you are bad!".to_string(),
                path: Some(WalkingPath {
                    path: vec![
                        Direction::Left,
                        Direction::Up,
                        Direction::Right,
                        Direction::Down,
                    ],
                    index: 0,
                }),
            },
            HumanWalkAnimation {
                rotation: Direction::Left,
                time: (1.0, 0.0),
                left_leg: true,
                sprinting: false,
            },
        ));

        let _berry = world.spawn((
            Position(Vec2(10f32, 8f32)),
            Sprite::berry(),
            Collision,
            GroundItem {
                item: bag::Item::Berry,
                amount: 1,
            },
        ));

        let mut menus = MenuManager::new();
        menus.open_menu(MainMenu::new().into());

        Self {
            screen: Screen::Overworld,
            next_screen: Screen::Overworld,
            //screen: Screen::Battle(TEST_BATTLE),
            input: EnumMap::default(),
            paused: true,
            world,
            cmd,
            events: vec![],
            map: TileMap::load(0),
            menus,
            player,
            collisions: HashMap::new(),
            rng: rand::thread_rng(),
            transitioning: false
        }
    }
}

impl State {
    pub fn render(
        &mut self,
        renderer: &mut Renderer,
        //_canvas: &Canvas<Window>,
        texture_manager: &mut TextureManager<WindowContext>,
        font_manager: &FontManager,
        delta_time: f32,
        map: &mut TileMap,
    ) -> Result<(), String> {
        renderer.is_fading = self.transitioning;

        match &self.screen {
            Screen::MainMenu => {}
            Screen::Overworld => {
                let transition_done = renderer.render_overworld(
                    texture_manager,
                    font_manager,
                    delta_time,
                    &self.world,
                    map,
                    &mut self.menus,
                )?;
                if transition_done {
                    self.events.push(Event::TransitionFull);
                }
            }
            Screen::Battle(battle) => renderer.render_battle(
                texture_manager,
                font_manager,
                delta_time,
                battle,
                &mut self.menus,
                &self.world,
            )?,
        }

        self.transitioning = renderer.is_fading;

        Ok(())
    }

    pub fn update(
        &mut self,
        delta_time: f32,
        _map: &mut TileMap,
        font_manager: &FontManager,
    ) -> Result<(), String> {
        //determine correct input handler
        if self.menus.is_open() {
            self.update_player_moving(MovingState::Idle);
            self.paused = self.handle_input_menus(font_manager);
        } else {
            self.handle_input_gameplay(font_manager);
        }

        //do any physics/animation updates
        if !self.paused {
            self.update_moving_objects(delta_time);
            self.update_collisions();
            self.update_animations(delta_time);
            self.process_events(font_manager);
        }
        Ok(())
    }

    pub fn process_events(&mut self, font_man: &FontManager) {
        while let Some(event) = self.events.pop() {
            match event {
                Event::PlayerMoved(pos) => {
                    if self.map.check_encounter(pos)
                        && self.rng.gen::<f32>() <= RANDOM_ENCOUNTER_CHANCE
                    {
                        self.next_screen = Screen::Battle(Battle {
                            player_strays: [
                                Some(Stray::palliub()),
                                Some(Stray::cespae()),
                                None,
                                Some(Stray::catis()),
                            ],
                            opponent_strays: [
                                Some(Stray::carerus()),
                                None,
                                Some(Stray::rubridum()),
                                Some(Stray::omikae()),
                            ],
                        });
                        self.transitioning = true;
                    }
                }
                Event::TransitionFull => {
                    std::mem::swap(&mut self.screen, &mut self.next_screen);
                    self.menus.open_menu(MovesMenu::new().into());
                }
                Event::NpcMoved(entity) => {
                    let (moving, npc) = self
                        .world
                        .query_one_mut::<(&mut MovingEntity, &mut Npc)>(entity)
                        .unwrap();
                    if let Some(path) = &mut npc.path {
                        path.advance();
                        moving.rotation = path.direction();
                        moving.try_moving = MovingState::Moving(path.direction());
                    }
                }
                Event::BattleAttack(selection) => {
                    println!("{:?}", selection);
                }           
            }
        }
    }

    pub fn check_collision(&self, position: &Vec2) -> bool {
        self.map.check_collision(*position)
            || self
                .collisions
                .contains_key(&position.to_usize(self.map.size_x))
    }

    pub fn try_player_interaction(&mut self, font_man: &FontManager) {
        let (&Position(Vec2(x, y)), moving) = self
            .world
            .query_one_mut::<(&Position, &MovingEntity)>(self.player)
            .unwrap();

        if moving.moving != MovingState::CenterTile && moving.moving != MovingState::Idle {
            return;
        }

        let temp_pos = match moving.rotation {
            Direction::Left => Vec2(x - 1.0, y),
            Direction::Right => Vec2(x + 1.0, y),
            Direction::Up => Vec2(x, y - 1.0),
            Direction::Down => Vec2(x, y + 1.0),
        };

        //check for an entity based on player position in the collisions map
        let interact_entity = self
            .collisions
            .get_key_value(&temp_pos.to_usize(self.map.size_x));

        //if it matches, run the list of interactions it comes with
        if let Some((_, &entity)) = interact_entity {
            let npc = self.world.query_one_mut::<&Npc>(entity);
            if let Ok(Npc { says, .. }) = npc {
                self.menus.open_menu(Textbox::new(says, font_man).into());
                return;
            }

            if let Ok(&GroundItem { item, amount }) =
                self.world.query_one_mut::<&GroundItem>(entity)
            {
                if let Ok(bag) = self.world.query_one_mut::<&mut Bag>(self.player) {
                    if bag.add_item(item, amount) {
                        self.menus.open_menu(
                            Textbox::new(
                                &format!("You picked up {} (x{}).", item, amount),
                                font_man,
                            )
                            .into(),
                        );
                        self.world.despawn(entity).unwrap();
                    }
                };
            }
        }
    }
}
