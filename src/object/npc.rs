use sdl2::rect::Rect;

use crate::coordinate::{Coordinate, Direction};
use crate::humanoid::{Humanoid, Leg, ROTATION_TIME, WALKING_TIME_PER_TILE};
use crate::menu::{textbox::Textbox, MenuManager};
use crate::object::{ObjectManager, TObject};
use crate::render::Renderer;
use crate::tilemap;
use crate::{coordinate, menu};

use super::CollisionManager;

#[derive(Debug, PartialEq)]
pub enum Character {
    Dad,
    Jodo,
    Sika,
}

pub struct NPC {
    pos: Coordinate,
    prev_pos: Coordinate,
    pub character: Character,
    try_sprinting: bool,
    is_sprinting: bool,
    moving_towards: Option<Coordinate>,
    animation_time: f64,
    facing: Direction,
    current_leg: Leg,
    walking: Option<Direction>,
    is_moving: bool,
    rotation_timer: f64,
    is_talking: bool,
}

impl NPC {
    pub fn new(pos: Coordinate, character: Character, moving_towards: Option<Coordinate>) -> NPC {
        let is_moving = match moving_towards {
            Some(_) => true,
            _ => false,
        };
        let facing = match moving_towards {
            Some(next_pos) => coordinate::compute_direction(pos, next_pos),
            _ => Direction::DOWN,
        };

        NPC {
            pos,
            prev_pos: pos,
            character,
            try_sprinting: false,
            is_sprinting: false,
            moving_towards,
            animation_time: 0.0,
            facing,
            current_leg: Leg::LEFT,
            walking: None,
            is_moving,
            rotation_timer: 0.0,
            is_talking: false,
        }
    }
}

impl TObject for NPC {
    fn get_pos(&self) -> Coordinate {
        self.pos
    }

    fn get_prev_pos(&self) -> Coordinate {
        self.prev_pos
    }

    fn set_pos(&mut self, pos: Coordinate) {
        self.pos = pos;
    }

    fn update(
        &mut self,
        delta_time: &f64,
        map: &tilemap::TileMap,
        collision_manager: &CollisionManager,
    ) -> bool {
        match self.moving_towards {
            Some(_) => {
                self.animation_time = self.animation_time - delta_time;
                self.move_towards_target(delta_time, map, collision_manager);

                if self.moving_towards == None {
                    self.facing = match self.facing {
                        Direction::LEFT => Direction::RIGHT,
                        Direction::RIGHT => Direction::LEFT,
                        Direction::UP => Direction::DOWN,
                        Direction::DOWN => Direction::UP,
                    };
                    self.rotation_timer = ROTATION_TIME; //to skip rotation check for now
                    self.start_walk(self.facing, map, collision_manager);
                }
                return true;
            }
            None => {}
        }

        false
    }

    fn interact(
        &mut self,
        renderer: &mut Renderer,
        menu_man: &mut MenuManager,
        player_position: Coordinate,
    ) -> bool {
        self.set_facing(coordinate::compute_direction(self.pos, player_position));
        menu_man.open_menu(menu::Menu::Textbox(Textbox::new(
            "Hi hungry, I'm dad! Nice try, little child --> you are bad!".to_string(),
        )));
        false
    }
}

impl Humanoid for NPC {
    fn get_pos(&self) -> Coordinate {
        self.pos
    }
    fn set_pos(&mut self, pos: Coordinate) {
        self.pos = pos;
    }
    fn get_prev_pos(&self) -> Coordinate {
        self.prev_pos
    }
    fn set_prev_pos(&mut self, pos: Coordinate) {
        self.prev_pos = pos;
    }
    fn get_facing(&self) -> Direction {
        self.facing
    }
    fn set_facing(&mut self, dir: Direction) {
        self.facing = dir;
    }
    fn get_moving_towards(&self) -> Option<Coordinate> {
        self.moving_towards
    }
    fn set_moving_towards(&mut self, pos: Option<Coordinate>) {
        self.moving_towards = pos;
    }
    fn get_current_leg(&self) -> Leg {
        self.current_leg
    }
    fn set_current_leg(&mut self, leg: Leg) {
        self.current_leg = leg;
    }
    fn get_try_sprinting(&self) -> bool {
        self.try_sprinting
    }
    fn set_try_sprinting(&mut self, try_sprinting: bool) {
        self.try_sprinting = try_sprinting;
    }
    fn get_is_sprinting(&self) -> bool {
        self.is_sprinting
    }
    fn set_is_sprinting(&mut self, is_sprinting: bool) {
        self.is_sprinting = is_sprinting;
    }
    fn get_walking(&self) -> Option<Direction> {
        self.walking
    }
    fn set_walking(&mut self, walking: Option<Direction>) {
        self.walking = walking;
    }
    /*fn get_is_moving(&self) -> bool {
        self.is_moving
    }
    fn set_is_moving(&mut self, is_moving: bool) {
        self.is_moving = is_moving;
    }*/
    fn get_rotation_timer(&self) -> f64 {
        self.rotation_timer
    }
    fn set_rotation_timer(&mut self, time: f64) {
        self.rotation_timer = time;
    }

    fn get_animation_time(&self) -> f64 {
        self.animation_time
    }

    fn set_animation_time(&mut self, time: f64) {
        self.animation_time = time;
    }
}

impl NPC {
    pub fn get_texture(&self) -> sdl2::rect::Rect {
        let anim_time = WALKING_TIME_PER_TILE;

        if self.moving_towards == None
            || self.animation_time > (0.75 * anim_time)
            || self.animation_time < (0.25 * anim_time)
        {
            match self.facing {
                Direction::UP => Rect::new(16, 0, 16, 16),
                Direction::RIGHT => Rect::new(16, 16, 16, 16),
                Direction::DOWN => Rect::new(0, 0, 16, 16),
                Direction::LEFT => Rect::new(0, 16, 16, 16),
            }
        } else {
            match self.facing {
                Direction::UP => match self.current_leg {
                    Leg::LEFT => Rect::new(16, 32, 16, 16),
                    Leg::RIGHT => Rect::new(0, 32, 16, 16),
                },
                Direction::RIGHT => Rect::new(48, 16, 16, 16),
                Direction::DOWN => match self.current_leg {
                    Leg::LEFT => Rect::new(32, 32, 16, 16),
                    Leg::RIGHT => Rect::new(48, 32, 16, 16),
                },
                Direction::LEFT => Rect::new(32, 16, 16, 16),
            }
        }
    }
}
