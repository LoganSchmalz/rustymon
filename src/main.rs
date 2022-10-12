const TILE_SIZE: i32 = 16;

mod input;
mod player;
mod render;

extern crate sdl2;

use sdl2::image::LoadTexture;
use std::path::Path;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Rustymon", render::PIXELS_X, render::PIXELS_Y)
        .position_centered()
        .resizable()
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

    let mut time_now: u64 = sdl_context.timer().unwrap().performance_counter();

    let mut renderer = render::Renderer::new();

    'running: loop {
        let time_last = time_now;
        time_now = sdl_context.timer().unwrap().performance_counter();
        let delta_time: f64 = ((time_now - time_last) * 1000
            / sdl_context.timer().unwrap().performance_frequency())
            as f64;

        match input.handle_input(&mut event_pump, &mut player, &mut renderer, &mut canvas) {
            true => break 'running,
            false => {}
        };

        player.update(&delta_time);
        renderer.render(&delta_time, &mut canvas, &player);
    }
}
