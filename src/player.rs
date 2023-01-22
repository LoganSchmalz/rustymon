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
    animation_time: f64,
    facing: Direction,
    current_leg: Leg,
    walking: Option<Direction>,
    rotation_timer: f64,
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
    fn get_walking(&self) -> Option<Direction> {
        self.walking
    }
    fn set_walking(&mut self, walking: Option<Direction>) {
        self.walking = walking;
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

impl Player {
    pub fn new() -> Player {
        Player {
            pos: Coordinate(1.0, 1.0),
            prev_pos: Coordinate(1.0, 1.0),
            try_sprinting: false,
            is_sprinting: false,
            moving_towards: None,
            animation_time: 0.0,
            facing: Direction::DOWN,
            current_leg: Leg::LEFT,
            walking: None,
            rotation_timer: 0.0,
        }
    }

    pub fn update(&mut self, delta_time: &f64, map: &TileMap, collision_manager: &CollisionManager) -> bool {
        self.walk(map, collision_manager);

        if self.rotation_timer > 0.0 {
            self.rotation_timer -= delta_time;
        }

        if self.animation_time > 0.0 {
            self.animation_time -= delta_time;
        }


        match self.moving_towards {
            Some(_) => {
                self.move_towards_target(delta_time);
                true
            }
            None => {
                self.animation_time = 0.0;
                false
            }
        }
    }

    pub fn get_texture(&self) -> sdl2::rect::Rect {
        let anim_time = if self.is_sprinting {
            RUNNING_TIME_PER_TILE
        } else {
            WALKING_TIME_PER_TILE
        };

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
