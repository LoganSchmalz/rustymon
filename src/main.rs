const TILE_SIZE: i32 = 16;

mod input;
mod player;
mod render;

extern crate sdl2;

use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::sys::{SDL_GetPerformanceCounter, SDL_GetPerformanceFrequency};
use std::path::Path;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 240, 160)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .unwrap();

    let texture_creator = canvas.texture_creator();
    let player_texture = texture_creator
        .load_texture(Path::new("assets/charSprite.png"))
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let input = input::Input { allow_input: true };
    let mut player: player::Player = player::Player::new(player_texture);

    let mut time_now: u64 = unsafe { SDL_GetPerformanceCounter() };

    'running: loop {
        let time_last = time_now;
        time_now = unsafe { SDL_GetPerformanceCounter() };
        let delta_time: f64 =
            ((time_now - time_last) * 1000 / unsafe { SDL_GetPerformanceFrequency() }) as f64;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        input.handle_input(event_pump.keyboard_state(), &mut player);
        player.update(&delta_time);

        render::render(&delta_time, &mut canvas, &player);
    }
}
