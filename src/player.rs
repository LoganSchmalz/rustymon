 use crate::coordinate::{Coordinate, Direction};
use crate::humanoid::Humanoid;
use crate::humanoid::{Leg, WALKING_TIME_PER_TILE, RUNNING_TIME_PER_TILE};
use crate::object::CollisionManager;
use crate::tilemap::TileMap;
use sdl2::rect::Rect;

pub struct Player {
    pos: Coordinate,
    prev_pos: Coordinate,
    try_sprinting: bool,
    is_sprinting: bool,
    moving_towards: Option<Coordinate>,
    animation_time: f32,
    facing: Direction,
    current_leg: Leg,
    try_walking: Option<Direction>, 
    rotation_timer: f32,
}

impl Humanoid for Player {
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
    fn get_rotation_timer(&self) -> f32 {
        self.rotation_timer
    }
    fn set_rotation_timer(&mut self, time: f32) {
        self.rotation_timer = time;
    }

    fn get_animation_time(&self) -> f32 {
        self.animation_time
    }

    fn set_animation_time(&mut self, time: f32) {
        self.animation_time = time;
    }
}

impl Player {
    pub fn new() -> Player {
        Player {
            pos: Coordinate(1.0, 1.0),
            prev_pos: Coordinate(1.0, 1.0),
            try_sprinting: false,
            is_sprinting: false,
            moving_towards: None,
            animation_time: 0.0,
            facing: Direction::Down,
            current_leg: Leg::Left,
            try_walking: None,
            rotation_timer: 0.0,
        }
    }

    pub fn update(&mut self, delta_time: &f32, map: &TileMap, collision_manager: &CollisionManager) {
        if self.rotation_timer > 0.0 {
            self.rotation_timer -= delta_time;
        }

        if self.animation_time > 0.0 {
            self.animation_time -= delta_time;
        }

        self.walk(delta_time, map, collision_manager);
    }

    pub fn get_texture(&self) -> sdl2::rect::Rect {
        let anim_time = if self.is_sprinting {
            RUNNING_TIME_PER_TILE
        } else {
            WALKING_TIME_PER_TILE
        };

        if self.moving_towards == None
            || self.animation_time < (0.5 * anim_time)
        {
            match self.facing {
                Direction::Up => Rect::new(16, 0 + 60*(self.try_sprinting as i32), 16, 20),
                Direction::Right => Rect::new(48, 0 + 60*(self.try_sprinting as i32), 16, 20),
                Direction::Down => Rect::new(0, 0 + 60*(self.try_sprinting as i32), 16, 20),
                Direction::Left => Rect::new(32, 0 + 60*(self.try_sprinting as i32), 16, 20),
            }
        } else {
            match self.facing {
                Direction::Up => match self.current_leg {
                    Leg::Left => Rect::new(16, 40 + 60*(self.is_sprinting as i32), 16, 20),
                    Leg::Right => Rect::new(16, 20 + 60*(self.is_sprinting as i32), 16, 20),
                },
                Direction::Right => match self.current_leg {
                    Leg::Left => Rect::new(48, 40 + 60*(self.is_sprinting as i32), 16, 20),
                    Leg::Right => Rect::new(48, 20 + 60*(self.is_sprinting as i32), 16, 20),
                },
                Direction::Down => match self.current_leg {
                    Leg::Left => Rect::new(0, 40 + 60*(self.is_sprinting as i32), 16, 20),
                    Leg::Right => Rect::new(0, 20 + 60*(self.is_sprinting as i32), 16, 20),
                },
                Direction::Left => match self.current_leg {
                    Leg::Left => Rect::new(32, 40 + 60*(self.is_sprinting as i32), 16, 20),
                    Leg::Right => Rect::new(32, 20 + 60*(self.is_sprinting as i32), 16, 20),
                },
            }
        }
    }
}
