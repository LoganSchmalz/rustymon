pub const TILE_SIZE: i32 = 16;

pub const WALK_SPEED: f32 = 4.0 / 1000.0; // tiles per millisecond
pub const WALKING_TIME_PER_TILE: f32 =  1.0 / WALK_SPEED; // in ms
pub const RUN_SPEED: f32 = 8.0 / 1000.0; // tiles per millisecond
pub const RUNNING_TIME_PER_TILE: f32 = 1.0 / RUN_SPEED; // in ms
pub const _HUMAN_WIDTH: u32 = 16;
pub const _HUMAN_HEIGHT: u32 = 20;
pub const ROTATION_TIME: f32 = RUNNING_TIME_PER_TILE;