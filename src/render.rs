use sdl2::{
    pixels::Color,
    rect::Rect,
    render::Canvas,
    //ttf::{Font, Sdl2TtfContext},
    video::{FullscreenType, Window},
};
use std::path::Path;

use crate::{
    coordinate::Coordinate,
    font_manager::FontManager,
    humanoid::{self, Humanoid},
    menu, object,
    object::TObject,
    player,
    texture_manager::{self, Sprite, TextureManager},
    tilemap, TILE_SIZE,
};
use object::npc;
use tilemap::TileMap;

pub const PIXELS_X: u32 = 240;
pub const PIXELS_Y: u32 = 160;
const FADE_FRAMES: i32 = 14;
const FADE_TIME: f64 = FADE_FRAMES as f64 * 64.0;
const _TEXT_TIME: f64 = 500.0;

#[derive(PartialEq)]
pub enum DisplayScreen {
    _MainMenu,
    _OverWorld,
}

pub struct Renderer {
    window_x: u32,
    window_y: u32,
    old_window_x: u32,
    old_window_y: u32,
    pub is_fading: bool,
    did_trans: bool,
    fade_anim_time: f64,
    camera_offset: (i32, i32),
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            window_x: PIXELS_X,
            window_y: PIXELS_Y,
            old_window_x: PIXELS_X,
            old_window_y: PIXELS_Y,
            is_fading: false,
            did_trans: false,
            fade_anim_time: FADE_TIME,
            camera_offset: (
                (TILE_SIZE as f64 - (PIXELS_X / 2 - humanoid::WIDTH / 2) as f64) as i32,
                (TILE_SIZE as f64 - (PIXELS_Y / 2 - humanoid::HEIGHT / 2) as f64) as i32,
            ),
        }
    }

    pub fn render_overworld_tiles(
        &mut self,
        canvas: &mut Canvas<Window>,
        texture_manager: &mut TextureManager,
        map: &tilemap::TileMap,
    ) {
        //TODO: remove next few lines, eventually we should just make the maps big enough to fill in the spaces that you can't walk into with actual tiles
        let screen_quad = Rect::new(0, 0, PIXELS_X, PIXELS_Y);
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.fill_rect(screen_quad).unwrap();

        for i in 0..map.size_x {
            for j in 0..map.size_y {
                let render_quad = Rect::new(
                    i as i32 * TILE_SIZE - self.camera_offset.0,
                    j as i32 * TILE_SIZE - self.camera_offset.1,
                    TILE_SIZE as u32,
                    TILE_SIZE as u32,
                );
                match map.floor.get(i + j * map.size_x) {
                    Some(tile) => {
                        let sprite = texture_manager.get_tile(tile.clone());
                        canvas
                            .copy(sprite.texture, sprite.src, render_quad)
                            .unwrap()
                    }
                    _ => {}
                };
                match map.walls.get(i + j * map.size_x) {
                    Some(tile) => {
                        let sprite = texture_manager.get_tile(tile.clone());
                        canvas
                            .copy(sprite.texture, sprite.src, render_quad)
                            .unwrap()
                    }
                    _ => {}
                };
            }
        }
    }

    pub fn render_objects(
        &mut self,
        canvas: &mut Canvas<Window>,
        texture_manager: &mut TextureManager,
        obj_man: &object::ObjectManager,
    ) {
        for obj in &obj_man.objects {
            let sprite = texture_manager.get_object(obj);
            let render_quad = Rect::new(
                obj.get_pos().0 as i32 * TILE_SIZE - self.camera_offset.0,
                obj.get_pos().1 as i32 * TILE_SIZE - self.camera_offset.1,
                TILE_SIZE as u32,
                TILE_SIZE as u32,
            );
            match obj {
                object::Object::NPC(ref npc) => {
                    self.render_npc(canvas, texture_manager, npc);
                }
                _ => canvas
                    .copy(sprite.texture, sprite.src, render_quad)
                    .unwrap(),
            }
        }
    }

    pub fn render_transition(
        &mut self,
        canvas: &mut Canvas<Window>,
        texture_manager: &mut TextureManager,
        delta_time: &f64,
        map: &mut tilemap::TileMap,
        obj_man: &mut object::ObjectManager,
    ) {
        if self.is_fading {
            self.fade_anim_time = self.fade_anim_time - delta_time;
            if self.fade_anim_time <= 0.0 {
                self.is_fading = false;
            } else {
                //might be timing issues here (starts at -_delta_time instead of the actual beginning)
                let curr_fade_frame: i32 = (FADE_FRAMES as f64
                    * (1.0 - (self.fade_anim_time / FADE_TIME) as f64))
                    .round() as i32;
                let screen_quad = Rect::new(0, 0, PIXELS_X, PIXELS_Y); //TODO: change height and width of screen_quad to not require math
                let fade_slice = Rect::new(240 * curr_fade_frame, 0, 240, 160);
                canvas
                    .copy(
                        &texture_manager.textures.fade_texture,
                        fade_slice,
                        screen_quad,
                    )
                    .unwrap();
                if (FADE_FRAMES as f64 * (1.0 - (self.fade_anim_time / FADE_TIME) as f64)).round()
                    as i32
                    > FADE_FRAMES / 2
                    && !self.did_trans
                {
                    match map.map_id {
                        0 => {
                            *map = TileMap::load(Path::new("maps/map1/"), 1);
                            obj_man.load_objects(Path::new("maps/map1/"));
                        }
                        1 => {
                            *map = TileMap::load(Path::new("maps/map0/"), 0);
                            obj_man.load_objects(Path::new("maps/map0"));
                        }
                        _ => panic!("Trying to load map that doesn't exist"),
                    }
                    self.did_trans = true;
                }
            }
        }
    }

    pub fn render_player(
        &mut self,
        canvas: &mut Canvas<Window>,
        texture_manager: &TextureManager,
        player: &player::Player,
    ) {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        let render_quad = Rect::new(
            (PIXELS_X / 2 - humanoid::WIDTH / 2) as i32,
            (PIXELS_Y / 2 - humanoid::HEIGHT / 2) as i32 - 4,
            humanoid::WIDTH,
            humanoid::HEIGHT,
        );
        /*let render_quad = Rect::new(
            player.pos.0 as i32,
            player.pos.1 as i32,
            player::PLAYER_WIDTH,
            player::PLAYER_HEIGHT,
        );*/
        canvas
            .copy(
                &texture_manager.textures.player,
                player.get_texture(),
                render_quad,
            )
            .unwrap();
    }

    pub fn render_npc(
        //TODO MAKE IT SO YOU CAN ACTUALLY HAVE MULTIPLE NPCs
        &mut self,
        canvas: &mut Canvas<Window>,
        texture_manager: &TextureManager,
        npc: &npc::NPC,
    ) {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        let Coordinate(i, j) = Humanoid::get_pos(npc);
        let render_quad = Rect::new(
            (i * TILE_SIZE as f64) as i32 - self.camera_offset.0,
            (j * TILE_SIZE as f64) as i32 - self.camera_offset.1 - 4,
            TILE_SIZE as u32,
            TILE_SIZE as u32,
        );
        let Sprite { texture, src } = &texture_manager.get_npc(npc);

        canvas.copy(*texture, *src, render_quad).unwrap();
    }

    pub fn render_menus(
        &mut self,
        canvas: &mut Canvas<Window>,
        texture_manager: &mut TextureManager,
        font_man: &FontManager,
        menu_man: &mut menu::MenuManager,
    ) {
        menu_man.render(canvas, texture_manager, font_man);
    }
    /*pub fn render_menus(
        &mut self,
        canvas: &mut Canvas<Window>,
        textures: &mut Textures,
        menu_man: &menu::Menu_Manager,
    ) {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        match menu.get_menu() {
            Menu_State::MAIN_MENU => {
                //self.render_main_menu(canvas, textures);
            }
            Menu_State::TEXTBOX => {
                let render_quad = Rect::new(
                    (1 * PIXELS_X / 16) as i32,
                    (11 * PIXELS_Y / 16) as i32,
                    PIXELS_X * 14 / 16,
                    PIXELS_Y * 4 / 16,
                );
                canvas.fill_rect(render_quad);
            }
            _ => {}
        }
    }*/

    pub fn render(
        &mut self,
        canvas: &mut Canvas<Window>,
        texture_manager: &mut texture_manager::TextureManager,
        font_man: &FontManager,
        delta_time: &f64,
        player: &player::Player,
        map: &mut tilemap::TileMap,
        menu_man: &mut menu::MenuManager,
        obj_man: &mut object::ObjectManager,
    ) {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        let Coordinate(x, y) = player.get_pos();

        self.camera_offset = (
            (x * TILE_SIZE as f64 - (PIXELS_X / 2 - humanoid::WIDTH / 2) as f64) as i32,
            (y * TILE_SIZE as f64 - (PIXELS_Y / 2 - humanoid::HEIGHT / 2) as f64) as i32,
        );
        self.render_overworld_tiles(canvas, texture_manager, map);
        self.render_objects(canvas, texture_manager, obj_man);
        self.render_player(canvas, texture_manager, player);
        self.render_menus(canvas, texture_manager, font_man, menu_man);
        self.render_transition(canvas, texture_manager, delta_time, map, obj_man);

        canvas.present();
    }

    pub fn play_fade(&mut self) {
        //TODO LOCK PLAYER WHEN FADE IS PLAYING SO THEY CANT WALK ON WATER
        self.is_fading = true;
        self.did_trans = false;
        self.fade_anim_time = FADE_TIME;
    }

    pub fn toggle_fullscreen(&mut self, canvas: &mut Canvas<Window>) {
        match canvas.window().fullscreen_state() {
            FullscreenType::Off => {
                let display = canvas.window().subsystem().display_bounds(0).unwrap();
                self.old_window_x = self.window_x;
                self.old_window_y = self.window_y;
                self.window_x = display.width();
                self.window_y = display.height();

                canvas
                    .window_mut()
                    .set_size(self.window_x, self.window_y)
                    .unwrap();
                canvas
                    .window_mut()
                    .set_fullscreen(FullscreenType::Desktop)
                    .unwrap();
            }
            FullscreenType::Desktop => {
                self.window_x = self.old_window_x;
                self.window_y = self.old_window_y;

                canvas
                    .window_mut()
                    .set_fullscreen(FullscreenType::Off)
                    .unwrap();
                canvas
                    .window_mut()
                    .set_size(self.window_x, self.window_y)
                    .unwrap();
            }
            _ => {
                panic!("Bad fullscreen state")
            }
        };
        let scale_x = self.window_x as f32 / PIXELS_X as f32;
        let scale_y = self.window_y as f32 / PIXELS_Y as f32;
        let scale = if scale_x <= scale_y {
            scale_x.floor()
        } else {
            scale_y.floor()
        } as u32;
        canvas.set_scale(scale as f32, scale as f32).unwrap();
        let bb_x = ((self.window_x - PIXELS_X * scale) / 2) / scale;
        let bb_y = ((self.window_y - PIXELS_Y * scale) / 2) / scale;
        let viewport = sdl2::rect::Rect::new(bb_x as i32, bb_y as i32, 10, 10);
        canvas.set_viewport(viewport);
    }

    pub fn resize(&mut self, canvas: &mut Canvas<Window>, width: i32, height: i32) {
        self.window_x = width as u32;
        self.window_y = height as u32;
        //canvas.set_integer_scale(true).unwrap();
        let scale_x = self.window_x as f32 / PIXELS_X as f32;
        let scale_y = self.window_y as f32 / PIXELS_Y as f32;
        let scale = if scale_x <= scale_y {
            scale_x.floor()
        } else {
            scale_y.floor()
        } as u32;
        canvas.set_scale(scale as f32, scale as f32).unwrap();
        // put top left corner of renderer at top left scaled of "screen" to maintain aspect ratio
        let bb_x = ((self.window_x - PIXELS_X * scale) / 2) / scale; //TODO: FIX BUG THAT CRASHES GAME WHEN YOU RESIZE SCREEN TOO SMALL
        let bb_y = ((self.window_y - PIXELS_Y * scale) / 2) / scale;
        let viewport = sdl2::rect::Rect::new(bb_x as i32, bb_y as i32, PIXELS_X, PIXELS_Y);
        canvas.set_viewport(viewport);
    }
}
