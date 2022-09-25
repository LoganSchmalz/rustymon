const TILE_SIZE: i32 = 16;

mod player;
mod input;

extern crate sdl2; 

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::sys::{SDL_GetPerformanceCounter, SDL_GetPerformanceFrequency};
use std::time::Duration;

pub fn main() {
	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();
 
	let window = video_subsystem.window("rust-sdl2 demo", 240, 160)
		.position_centered()
		.build()
		.unwrap();
 
	let mut canvas = window.into_canvas().build().unwrap();
 
	canvas.set_draw_color(Color::RGB(0, 255, 255));
	canvas.clear();
	canvas.present();
	let mut event_pump = sdl_context.event_pump().unwrap();
	let mut t: f64 = 0.0;

	let mut input = input::Input{ allow_input: true };
	let mut player : player::Player = player::Player::new();

	let mut time_last: u64 = 0;
	let mut time_now: u64 = unsafe { SDL_GetPerformanceCounter() };
	'running: loop {
		time_last = time_now;
		time_now = unsafe { SDL_GetPerformanceCounter() };
		let delta_time: f64 = ((time_now - time_last) * 1000 / unsafe { SDL_GetPerformanceFrequency() }) as f64;
		
		t = t + delta_time;
		if t >= 256.0 {
			t = t - 256.0;
		}
		println!("{:?}, {:?}", delta_time, t);
		let i = t as u8 % 255;

		canvas.set_draw_color(Color::RGB(i as u8, 64, 255 - i as u8));
		canvas.clear();
		for event in event_pump.poll_iter() {
			match event {
				Event::Quit {..} |
				Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
					break 'running
				},
				Event::KeyDown { keycode: Some(key), .. } => {
					input.handle_input(key, &mut player)
				}
				_ => {}
			}
		}
		// The rest of the game loop goes here...
		player.update(&delta_time);
		player.render(&mut canvas);

		canvas.present();
	}
}