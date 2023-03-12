use std::collections::HashMap;

use hecs::{CommandBuffer, Entity, World};
use sdl2::{rect::Rect, video::WindowContext};

use enum_map::EnumMap;

use crate::{
    components::{animation::HumanWalkAnimation, bag::Bag, sprite::Sprite, *},
    font_manager::FontManager,
    menu::{main_menu::MainMenu, textbox::Textbox, MenuManager},
    render::Renderer,
    resource_manager::TextureManager,
    tilemap::TileMap,
    vec2::{Direction, Vec2},
};

use self::input::{Control, KeyState};

mod input;
mod updates;

pub struct State {
    pub input: EnumMap<Control, KeyState>,
    pub paused: bool,
    pub world: World,
    pub cmd: CommandBuffer,
    pub map: TileMap,
    pub menus: MenuManager,
    pub player: Entity,
    pub collisions: HashMap<usize, Entity>,
}

impl Default for State {
    fn default() -> Self {
        let mut world = World::new();
        let mut cmd = CommandBuffer::new();

        //here we create a bunch of example entities for the default world
        let player = world.spawn((
            Player,
            Position(Vec2(2f32, 1f32)),
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
            Position(Vec2(4f32, 4f32)),
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
            input: EnumMap::default(),
            paused: true,
            world,
            cmd,
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
        }
        Ok(())
    }

    pub fn check_collision(&self, position: &Vec2) -> bool {
        self.map.check_collision(*position)
            || self
                .collisions
                .contains_key(&position.to_usize(self.map.size_x))
            || position.0 < 0f32
            || position.0 >= self.map.size_x as f32
            || position.1 < 0f32
            || position.1 >= self.map.size_y as f32
    }

    pub fn try_player_interaction(&mut self, font_man: &FontManager) {
        let (_, &Position(Vec2(x, y)), moving) = self
            .world
            .query_one_mut::<(&Player, &Position, &MovingEntity)>(self.player)
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
            if let Ok(Npc { says }) = npc {
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
