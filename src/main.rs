mod components;
mod constants;
mod font_manager;
mod gamestate;
mod menu;
mod render;
mod resource_manager;
mod tilemap;
mod vec2;

use crate::{gamestate::State, resource_manager::TextureManager};

extern crate sdl2;
extern crate enum_map;

#[macro_use]
extern crate num_derive;

use tilemap::TileMap;

pub fn main() -> Result<(), String> {
    //creating sdl context, window, canvas
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let mut window = video_subsystem
        .window("Rustymon", render::PIXELS_X, render::PIXELS_Y)
        .position_centered()
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;

    window
        .set_minimum_size(render::PIXELS_X, render::PIXELS_Y)
        .map_err(|e| e.to_string())?;

    let canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    //create sdl event pump
    let mut event_pump = sdl_context.event_pump().map_err(|e| e.to_string())?;

    //create texture manager, font manager
    let texture_creator = canvas.texture_creator();
    let mut texture_manager = TextureManager::new(&texture_creator);

    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let fonts = font_manager::Fonts::load(&ttf_context)?;
    let font_manager = font_manager::FontManager::new(fonts);

    //create engine renderer
    let mut renderer = render::Renderer::new(canvas);

    //load starting map
    let mut map = TileMap::load(0);

    //create default gamestate
    let mut state = State {
        ..Default::default()
    };

    //start timing frames
    let mut time_now: u64 = sdl_context.timer().unwrap().performance_counter();

    'running: loop {
        //calculate frametime
        let time_last = time_now;
        time_now = sdl_context.timer().unwrap().performance_counter();
        let delta_time: f32 = ((time_now - time_last) * 1000
            / sdl_context.timer().unwrap().performance_frequency())
            as f32;

        //check if quit event happened
        if state.update_input(&mut event_pump, &mut renderer)? {
            break 'running;
        }
        //update gamestate
        state.update(delta_time, &mut map, &font_manager)?;
        //render game
        state.render(
            &mut renderer,
            &mut texture_manager,
            &font_manager,
            delta_time,
            &mut map,
        )?;
    }

    Ok(())
}
