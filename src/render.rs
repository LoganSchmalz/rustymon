use sdl2::{
    image::LoadTexture,
    pixels::Color,
    rect::Rect,
    render::{Canvas, Texture, TextureCreator},
    video::{FullscreenType, Window, WindowContext},
    ttf::{Sdl2TtfContext, Font},
};
use std::path::Path;

use crate::{menu, player, tilemap, TILE_SIZE};
use tilemap::load_tilemap;

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
}

pub struct Textures<'a> {
    //Sprite sheets
    tilesprites: Texture<'a>,
    objectsprites: Texture<'a>,
    //Main menu assets
    pub main_menu: Texture<'a>,
    pub start_button: Texture<'a>,
    pub load_button: Texture<'a>,
    pub settings_button: Texture<'a>,
    //Transitions
    fade_texture: Texture<'a>,
    //Characters
    player: Texture<'a>,
    //Text Box
    pub text_box: Texture<'a>,
}

pub struct Fonts<'ttf_module, 'rwops> {
    pub press_start_2p: Font<'ttf_module, 'rwops>,
}

impl<'ttf_module, 'rwops> Fonts<'ttf_module, 'rwops> {
    pub fn load(font_loader: &'ttf_module Sdl2TtfContext) -> Self {
        let press_start_2p = font_loader.load_font("assets/PressStart2P-Regular.ttf", 120).unwrap();

        Fonts {
            press_start_2p,
        }
    }
}

impl<'a> Textures<'a> {
    pub fn load(creator: &'a TextureCreator<WindowContext>) -> Self {
        let tilesprites = creator.load_texture("assets/tilesprites.png").unwrap();
        let objectsprites = creator.load_texture("assets/objectsprites.png").unwrap();
        let main_menu = creator.load_texture("assets/titlescreen.png").unwrap();
        let start_button = creator.load_texture("assets/STARTbutton.png").unwrap();
        let load_button = creator.load_texture("assets/SAVELOADbutton.png").unwrap();
        let settings_button = creator.load_texture("assets/SETTINGSbutton.png").unwrap();
        let fade_texture = creator.load_texture("assets/gooWipe.png").unwrap();
        let player = creator.load_texture("assets/newcharsprite.png").unwrap();
        let text_box = creator.load_texture("assets/text_box.png").unwrap();

        Textures {
            tilesprites,
            objectsprites,
            main_menu,
            start_button,
            load_button,
            settings_button,
            fade_texture,
            player,
            text_box,
        }
    }
}

pub struct TileRect {
    //Tile sprites
    g1: Rect,
    g2: Rect,
    w1: Rect,
    wg_tl: Rect,
    wg_t: Rect,
    wg_tr: Rect,
    wg_r: Rect,
    wg_br: Rect,
    wg_b: Rect,
    wg_bl: Rect,
    wg_l: Rect,
    gw_tl: Rect,
    gw_tr: Rect,
    gw_br: Rect,
    gw_bl: Rect,
    wood_l: Rect,
    wood_r: Rect,
    door1: Rect,
    fb1: Rect,
    //Object sprites
    berry1: Rect,
}

impl TileRect {
    pub fn new() -> TileRect {
        TileRect {
            //Rect::new(x, y, width, height)
            //Tile sprites
            g1: Rect::new(32, 0, 16, 16), //Grass 1
            g2: Rect::new(48, 0, 16, 16), //Grass 2
            w1: Rect::new(16, 64, 16, 16), //Water 1
            wg_tl: Rect::new(0, 48, 16, 16), //Water-Grass Top Left
            wg_t: Rect::new(16, 48, 16, 16), //Water-Grass Top
            wg_tr: Rect::new(32, 48, 16, 16), //Water-Grass Top Right
            wg_l: Rect::new(0, 64, 16, 16), //Water-Grass Left
            wg_r: Rect::new(32, 64, 16, 16), //Water-Grass Right
            wg_bl: Rect::new(0, 80, 16, 16), //Water-Grass Bottom Left
            wg_b: Rect::new(16, 80, 16, 16), //Water-Grass Bottom
            wg_br: Rect::new(32, 80, 16, 16), //Water-Grass Bottom Right
            gw_tl: Rect::new(48, 48, 16, 16), //Grass-Water Top Left
            gw_tr: Rect::new(80, 48, 16, 16), //Grass-Water Top Right
            gw_bl: Rect::new(48, 80, 16, 16), //Grass-Water Bottom Left
            gw_br: Rect::new(80, 80, 16, 16), //Grass-Water Bottom Right
            wood_l: Rect::new(128, 0, 16, 16), //Wood Left
            wood_r: Rect::new(160, 0, 16, 16), //Wood Right
            door1: Rect::new(96, 0, 16, 16), //Door 1
            fb1: Rect::new(112, 0, 16, 16), //Floor Base 1
            //Object sprites
            berry1: Rect::new(0, 0, 16, 16), //Berry 1
        }
    }
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
        }
    }

    pub fn render_overworld_tiles(
        &mut self,
        canvas: &mut Canvas<Window>,
        textures: &mut Textures,
        map: &tilemap::TileMap,
        camera_offset: (i32, i32),
    ) {
        //TODO: remove next few lines, eventually we should just make the maps big enough to fill in the spaces that you can't walk into with actual tiles
        let screen_quad = Rect::new(0, 0, PIXELS_X, PIXELS_Y);
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.fill_rect(screen_quad).unwrap();

        let tile_rects = TileRect::new();

        for i in 0..map.size_x {
            for j in 0..map.size_y {
                let render_quad = Rect::new(
                    i as i32 * TILE_SIZE - camera_offset.0,
                    j as i32 * TILE_SIZE - camera_offset.1,
                    TILE_SIZE as u32,
                    TILE_SIZE as u32,
                );
                match map.floor.get(i + j * map.size_x) {
                    Some(tilemap::FloorTile::GRASS1) => {
                        canvas.copy(&textures.tilesprites, tile_rects.g1, render_quad).unwrap()
                    }
                    Some(tilemap::FloorTile::GRASS2) => {
                        canvas.copy(&textures.tilesprites, tile_rects.g2, render_quad).unwrap()
                    }
                    Some(tilemap::FloorTile::WATER1) => {
                        canvas.copy(&textures.tilesprites, tile_rects.w1, render_quad).unwrap()
                    }
                    Some(tilemap::FloorTile::WGTL) => {
                        canvas.copy(&textures.tilesprites, tile_rects.wg_tl, render_quad).unwrap()
                    }
                    Some(tilemap::FloorTile::WGT) => {
                        canvas.copy(&textures.tilesprites, tile_rects.wg_t, render_quad).unwrap()
                    }
                    Some(tilemap::FloorTile::WGTR) => {
                        canvas.copy(&textures.tilesprites, tile_rects.wg_tr, render_quad).unwrap()
                    }
                    Some(tilemap::FloorTile::WGR) => {
                        canvas.copy(&textures.tilesprites, tile_rects.wg_r, render_quad).unwrap()
                    }
                    Some(tilemap::FloorTile::WGBR) => {
                        canvas.copy(&textures.tilesprites, tile_rects.wg_br, render_quad).unwrap()
                    }
                    Some(tilemap::FloorTile::WGB) => {
                        canvas.copy(&textures.tilesprites, tile_rects.wg_b, render_quad).unwrap()
                    }
                    Some(tilemap::FloorTile::WGBL) => {
                        canvas.copy(&textures.tilesprites, tile_rects.wg_bl, render_quad).unwrap()
                    }
                    Some(tilemap::FloorTile::WGL) => {
                        canvas.copy(&textures.tilesprites, tile_rects.wg_l, render_quad).unwrap()
                    }
                    Some(tilemap::FloorTile::GWTL) => {
                        canvas.copy(&textures.tilesprites, tile_rects.gw_tl, render_quad).unwrap()
                    }
                    Some(tilemap::FloorTile::GWTR) => {
                        canvas.copy(&textures.tilesprites, tile_rects.gw_tr, render_quad).unwrap()
                    }
                    Some(tilemap::FloorTile::GWBR) => {
                        canvas.copy(&textures.tilesprites, tile_rects.gw_br, render_quad).unwrap()
                    }
                    Some(tilemap::FloorTile::GWBL) => {
                        canvas.copy(&textures.tilesprites, tile_rects.gw_bl, render_quad).unwrap()
                    }
                    Some(tilemap::FloorTile::FB1) => {
                        canvas.copy(&textures.tilesprites, tile_rects.fb1, render_quad).unwrap()
                    }
                    None => {}
                };
                match map.objects.get(i + j * map.size_x) {
                    Some(tilemap::ObjectTile::BERRY) => {
                        canvas.copy(&textures.objectsprites, tile_rects.berry1, render_quad).unwrap()
                    }
                    Some(tilemap::ObjectTile::DOOR) => {
                        canvas.copy(&textures.tilesprites, tile_rects.door1, render_quad).unwrap()
                    }
                    Some(tilemap::ObjectTile::WOODL) => {
                        canvas.copy(&textures.tilesprites, tile_rects.wood_l, render_quad).unwrap()
                    }
                    Some(tilemap::ObjectTile::WOODR) => {
                        canvas.copy(&textures.tilesprites, tile_rects.wood_r, render_quad).unwrap()
                    }
                    _ => {}
                };
            }
        }
    }

    pub fn render_transition(&mut self, canvas: &mut Canvas<Window>, textures: &mut Textures, delta_time: &f64, map: &mut tilemap::TileMap) {
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
                    .copy(&textures.fade_texture, fade_slice, screen_quad)
                    .unwrap();
                if (FADE_FRAMES as f64 * (1.0 - (self.fade_anim_time / FADE_TIME) as f64)).round()
                    as i32
                    > FADE_FRAMES / 2
                    && !self.did_trans
                {
                    match map.map_id {
                        0 => *map = load_tilemap(Path::new("maps/map1/"), 1),
                        1 => *map = load_tilemap(Path::new("maps/map0/"), 0),
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
        textures: &mut Textures,
        player: &player::Player,
    ) {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        let render_quad = Rect::new(
            (PIXELS_X / 2 - player::PLAYER_WIDTH / 2) as i32,
            (PIXELS_Y / 2 - player::PLAYER_HEIGHT / 2) as i32,
            player::PLAYER_WIDTH,
            player::PLAYER_HEIGHT,
        );
        /*let render_quad = Rect::new(
            player.pos.0 as i32,
            player.pos.1 as i32,
            player::PLAYER_WIDTH,
            player::PLAYER_HEIGHT,
        );*/
        canvas
            .copy(&textures.player, player.get_texture(), render_quad)
            .unwrap();
    }

    pub fn render_menus(
        &mut self,
        canvas: &mut Canvas<Window>,
        textures: &mut Textures,
        fonts: &Fonts,
        menu_man: &mut menu::MenuManager,
    ) {
        menu_man.render(canvas, textures, fonts);
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
        textures: &mut Textures,fonts: &mut Fonts,
        delta_time: &f64,
        player: &player::Player,
        map: &mut tilemap::TileMap,
        menu_man: &mut menu::MenuManager,
    ) {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        let camera_offset = (
            (player.pos.0 - (PIXELS_X / 2 - player::PLAYER_WIDTH / 2) as f64) as i32,
            (player.pos.1 - (PIXELS_Y / 2 - player::PLAYER_HEIGHT / 2) as f64) as i32,
        );
        self.render_overworld_tiles(canvas, textures, map, camera_offset);
        self.render_player(canvas, textures, player);
        self.render_menus(canvas, textures, fonts, menu_man);
        self.render_transition(canvas, textures, delta_time, map);

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
