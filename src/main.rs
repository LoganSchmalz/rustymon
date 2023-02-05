const TILE_SIZE: i32 = 16;

mod bag;
mod engine_structures;
mod event;
mod font_manager;
mod humanoid;
mod input;
mod menu;
mod object;
mod player;
mod render;
mod texture_manager;
mod tilemap;

use crate::{
    bag::Bag,
    engine_structures::{coordinate, *},
    event::EventManager,
    humanoid::Humanoid,
};

extern crate sdl2;

extern crate enum_map;

#[macro_use]
extern crate num_derive;

use std::path::Path;
use tilemap::TileMap;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let mut window = video_subsystem
        .window("Rustymon", render::PIXELS_X, render::PIXELS_Y)
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    window
        .set_minimum_size(render::PIXELS_X, render::PIXELS_Y)
        .unwrap();

    let canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut input = input::Input::new();
    let mut player: player::Player = player::Player::new();

    let mut time_now: u64 = sdl_context.timer().unwrap().performance_counter();

    let texture_creator = canvas.texture_creator();
    let textures = texture_manager::Textures::load(&texture_creator);
    let mut texture_manager = texture_manager::TextureManager::new(textures);

    let ttf_context = sdl2::ttf::init();
    let font_loader = ttf_context.expect("Missing ttf context");
    let fonts = font_manager::Fonts::load(&font_loader);
    let font_manager = font_manager::FontManager::new(fonts);
    let mut renderer = render::Renderer::new(canvas);
    let mut menu_man = menu::MenuManager::new();
    let mut obj_man = object::ObjectManager::new();
    //menu_man.borrow_mut().open_menu(Box::new(MainMenu));

    //load original maps into current save
    //TODO: CHANGE THIS SAVING FUNCTIONALITY WE'RE NOT EVEN USING IT ANYMORE
    //init_map_save("map0".to_string());
    //init_map_save("map1".to_string());

    let mut map = TileMap::load(Path::new("maps/map0/"), 0);
    obj_man.load_objects(Path::new("maps/map0"));

    println!(
        "{}",
        serde_json::to_string(&obj_man.objects).expect("Error")
    );

    let mut bag = Bag::new();

    let mut event_man = EventManager::new();

    'running: loop {
        let time_last = time_now;
        time_now = sdl_context.timer().unwrap().performance_counter();
        let delta_time: f32 = ((time_now - time_last) * 1000
            / sdl_context.timer().unwrap().performance_frequency())
            as f32;

        player.set_try_walking(None);

        let input_events = input.handle_input(&mut event_pump);

        let exit = event_man.handle_input_event(input_events, &mut menu_man, &mut renderer);

        if exit {
            break 'running;
        }

        event_man.handle_gameplay_event(
            &mut menu_man,
            &mut player,
            &mut obj_man,
            &mut renderer,
            &mut bag,
        );

        //println!("{:?}", delta_time);
        if !menu_man.paused {
            player.update(&delta_time, &map, &obj_man.collision_manager);
            obj_man.update_objects(&delta_time, &map);
        }
        renderer.render(
            &mut texture_manager,
            &font_manager,
            &delta_time,
            &player,
            &mut map,
            &mut menu_man,
            &mut obj_man,
        );
    }
}
