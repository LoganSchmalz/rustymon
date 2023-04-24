/****************************************************/
// Created by: Logan Schmalz
// Description: Constant values needed by other game files
/****************************************************/
pub const TILE_SIZE: i32 = 16; //pixel width and height of tiles

pub const RANDOM_ENCOUNTER_CHANCE: f32 = 0.1; //chance of a random stray encounter when walking in possible stray tiles (e.g. tall grass)

pub const WALK_SPEED: f32 = 4.0 / 1000.0; // tiles per millisecond
pub const WALKING_TIME_PER_TILE: f32 = 1.0 / WALK_SPEED; // in ms
pub const RUN_SPEED: f32 = 8.0 / 1000.0; // tiles per millisecond
pub const RUNNING_TIME_PER_TILE: f32 = 1.0 / RUN_SPEED; // in ms
pub const _HUMAN_WIDTH: u32 = 16; //in pixels
pub const _HUMAN_HEIGHT: u32 = 20; //in pixels
pub const ROTATION_TIME: f32 = RUNNING_TIME_PER_TILE; //time it takes to rotate player in ms

pub const FADE_FRAMES: i32 = 14; //number of frames in fade animation spritesheet
pub const FADE_TIME: f32 = FADE_FRAMES as f32 * 64.0; //time in ms of fade animation