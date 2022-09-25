const TILE_SIZE: i32 = 16;

mod player;
mod input;

extern crate sdl2; 

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, self};
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::sys::{SDL_GetPerformanceCounter, SDL_GetPerformanceFrequency, SDL_Delay};
use std::path::Path;
use std::time::Duration;

pub fn main() {
	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();
 
	let window = video_subsystem.window("rust-sdl2 demo", 240, 160)
		.position_centered()
		.build()
		.unwrap();
 
	let mut canvas = window.into_canvas().accelerated().present_vsync().build().unwrap();
	let texture_creator = canvas.texture_creator();
	let texture = texture_creator.load_texture(Path::new("assets/charSprite.png")).unwrap();

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

		t = t + 0.125 * delta_time;
		if t >= 256.0 {
			t = t - 256.0;
		}
		//println!("{:.1}", delta_time);
		let i = t as u8 % 255;

		//canvas.set_draw_color(Color::RGB(i as u8, 64, 255 - i as u8));
		canvas.clear();
		for i in 0..(240/TILE_SIZE) {
			for j in 0..(160/TILE_SIZE) {
				if (i+j) % 2 == 0 {
					canvas.set_draw_color(Color::RGB(255,255,255));
					canvas
						.fill_rect(Rect::new(i*TILE_SIZE, j*TILE_SIZE, TILE_SIZE as u32, TILE_SIZE as u32))
						.unwrap();
				} else {
					canvas.set_draw_color(Color::RGB(255,0,0));
					canvas
						.fill_rect(Rect::new(i*TILE_SIZE, j*TILE_SIZE, TILE_SIZE as u32, TILE_SIZE as u32))
						.unwrap();
				}
			}
		}

		for event in event_pump.poll_iter() {
			match event {
				Event::Quit {..} |
				Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
					break 'running
				}
				_ => {}
			}
		}

		input.handle_input(event_pump.keyboard_state(), &mut player);

		// The rest of the game loop goes here...
		player.update(&delta_time);
		player.render(&mut canvas, &texture);
		canvas.present();
	}
}