use sdl2::rect::Rect;
use serde::{Deserialize, Serialize};

use crate::coordinate::{Coordinate, Direction};
use crate::event::Command;
use crate::humanoid::{Humanoid, Leg, WALKING_TIME_PER_TILE};
use crate::menu::{textbox::Textbox, MenuManager};
use crate::render::Renderer;
use crate::tilemap;
use crate::updated::Updated;
use crate::{coordinate, menu};

use super::{CollisionManager, TObject};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Character {
    Dad,
    Jodo,
    Sika,
}

#[derive(Debug, Serialize, Deserialize)]
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
    try_walking: Option<Direction>,
    rotation_timer: f64,
    path: Vec<Direction>,
    current_idx_in_path: usize,
}

impl NPC {
    pub fn new(pos: Coordinate, character: Character, moving_towards: Option<Coordinate>) -> NPC {
        let facing = match moving_towards {
            Some(next_pos) => coordinate::compute_direction(pos, next_pos),
            _ => Direction::Down,
        };

        let path = match moving_towards {
            Some(_) => vec![Direction::Left, Direction::Right],
            None => vec![],
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
            current_leg: Leg::Left,
            try_walking: None,
            rotation_timer: 0.0,
            path,
            current_idx_in_path: 0,
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
    ) -> Updated {
        if !self.path.is_empty() {
            self.walk_on_path(delta_time, map, collision_manager);
            //println!("{:?} {:?}", self.pos, self.prev_pos);
            return Updated(true);
        }
        Updated(false)
    }

    fn interact(
        &mut self,
        _renderer: &mut Renderer,
        menu_man: &mut MenuManager,
        player_position: Coordinate,
    ) -> Vec<Command> {
        self.set_facing(coordinate::compute_direction(self.pos, player_position));
        menu_man.open_menu(menu::Menu::Textbox(Textbox::new(
            "Hi hungry, I'm dad! Nice try, little child --> you are bad!".to_string(),
        )));
        vec![]
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
    fn get_try_walking(&self) -> Option<Direction> {
        self.try_walking
    }
    fn set_try_walking(&mut self, try_walking: Option<Direction>) {
        self.try_walking = try_walking;
    }
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
    fn walk_on_path(
        &mut self,
        delta_time: &f64,
        map: &tilemap::TileMap,
        collision_manager: &CollisionManager,
    ) {
        match self.moving_towards {
            Some(_) => {
                self.animation_time = self.animation_time - delta_time;
                self.move_towards_target(delta_time);

                if self.moving_towards == None {
                    self.facing = match self.facing {
                        Direction::Left => Direction::Right,
                        Direction::Right => Direction::Left,
                        Direction::Up => Direction::Down,
                        Direction::Down => Direction::Up,
                    };
                    self.try_walking = Some(self.facing);
                    self.rotation_timer = 0.0; //to skip rotation check for now
                    self.walk(delta_time, map, collision_manager);
                }
                return;
            }
            None => {}
        }
    }

    pub fn get_texture(&self) -> sdl2::rect::Rect {
        let anim_time = WALKING_TIME_PER_TILE;

        if self.moving_towards == None
            || self.animation_time > (0.75 * anim_time)
            || self.animation_time < (0.25 * anim_time)
        {
            match self.facing {
                Direction::Up => Rect::new(16, 0, 16, 20),
                Direction::Right => Rect::new(48, 0, 16, 20),
                Direction::Down => Rect::new(0, 0, 16, 20),
                Direction::Left => Rect::new(32, 0, 16, 20),
            }
        } else {
            match self.facing {
                Direction::Up => match self.current_leg {
                    Leg::Left => Rect::new(16, 40, 16, 20),
                    Leg::Right => Rect::new(16, 20, 16, 20),
                },
                Direction::Right => match self.current_leg {
                    Leg::Left => Rect::new(48, 40, 16, 20),
                    Leg::Right => Rect::new(48, 20, 16, 20),
                },
                Direction::Down => match self.current_leg {
                    Leg::Left => Rect::new(0, 40, 16, 20),
                    Leg::Right => Rect::new(0, 20, 16, 20),
                },
                Direction::Left => match self.current_leg {
                    Leg::Left => Rect::new(32, 40, 16, 20),
                    Leg::Right => Rect::new(32, 20, 16, 20),
                },
            }
        }
    }
}
