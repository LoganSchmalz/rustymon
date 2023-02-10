const TILE_SIZE: i32 = 16;

mod bag;
mod engine_structures;
mod event;
mod font_manager;
mod gamestate;
mod humanoid;
mod input;
mod menu;
mod object;
mod player;
mod render;
mod resource_manager;
mod tilemap;

use crate::{
    engine_structures::{coordinate, *},
    gamestate::State,
    resource_manager::TextureManager,
};

extern crate sdl2;

extern crate enum_map;

#[macro_use]
extern crate num_derive;

use tilemap::TileMap;

pub fn main() -> Result<(), String> {
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

    let mut event_pump = sdl_context.event_pump().map_err(|e| e.to_string())?;

    let mut input = input::Input::new();

    let texture_creator = canvas.texture_creator();
    let mut texture_manager = TextureManager::new(&texture_creator);

    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let fonts = font_manager::Fonts::load(&ttf_context)?;
    let font_manager = font_manager::FontManager::new(fonts);
    let mut renderer = render::Renderer::new(canvas);
    let mut menu_man = menu::MenuManager::new();

    let mut map = TileMap::load(0);

    let mut time_now: u64 = sdl_context.timer().unwrap().performance_counter();

    let mut state = State {
        allow_input: true,
        screen: gamestate::Screen::Gameplay,
        ..Default::default()
    };

    'running: loop {
        let time_last = time_now;
        time_now = sdl_context.timer().unwrap().performance_counter();
        let delta_time: f32 = ((time_now - time_last) * 1000
            / sdl_context.timer().unwrap().performance_frequency())
            as f32;

        if input.handle_events(&mut event_pump, &mut renderer)? {
            break 'running;
        };
        //let exit = event_man.handle_input_event(input_events, &mut menu_man, &mut renderer);

        /*event_man.handle_gameplay_event(
            &mut menu_man,
            &mut player,
            &mut obj_man,
            &mut renderer,
            &font_manager,
            &mut bag,
        );

        player.update(&delta_time, &map, &obj_man.collision_manager);
        obj_man.update_objects(&delta_time, &map);*/

        state.update(&delta_time, &mut input, &mut map)?;
        state.render(
            &mut renderer,
            &mut texture_manager,
            &font_manager,
            &delta_time,
            &mut map,
            &mut menu_man,
        )?;
    }

    Ok(())
}
