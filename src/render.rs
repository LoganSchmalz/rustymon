use sdl2::{
    image::LoadTexture,
    pixels::Color,
    rect::Rect,
    render::{Canvas, Texture, TextureCreator},
    video::{FullscreenType, Window, WindowContext},
};
use std::path::Path;

use crate::{player, tilemap, TILE_SIZE};
use tilemap::load_tilemap;

pub const PIXELS_X: u32 = 240;
pub const PIXELS_Y: u32 = 160;
const FADE_FRAMES: i32 = 14;
const FADE_TIME: f64 = FADE_FRAMES as f64 * 64.0;

#[derive(PartialEq)]
pub enum DisplayScreen {
    MainMenu,
    OverWorld,
}

#[derive(PartialEq)]
pub enum Button {
    StartButton,
    LoadButton,
    SettingsButton,
}

pub struct Renderer {
    window_x: u32,
    window_y: u32,
    old_window_x: u32,
    old_window_y: u32,
    pub display_screen: DisplayScreen,
    pub curr_button: usize,
    pub is_fading: bool,
    did_trans: bool,
    fade_anim_time: f64,
}

pub const BUTTONS: [Button; 3] = [
    Button::StartButton,
    Button::LoadButton,
    Button::SettingsButton,
];

pub struct Textures<'a> {
    main_menu: Texture<'a>,
    start_button: Texture<'a>,
    load_button: Texture<'a>,
    settings_button: Texture<'a>,
    fade_texture: Texture<'a>,
    player: Texture<'a>,
    grass1: Texture<'a>,
    grass2: Texture<'a>,
    water1: Texture<'a>,
    berry: Texture<'a>,
    door1: Texture<'a>,
    water_grass: Texture<'a>,
    wood: Texture<'a>,
}

impl<'a> Textures<'a> {
    pub fn load(creator: &'a TextureCreator<WindowContext>) -> Self {
        let main_menu = creator.load_texture("assets/titlescreen.png").unwrap();
        let start_button = creator.load_texture("assets/STARTbutton.png").unwrap();
        let load_button = creator.load_texture("assets/SAVELOADbutton.png").unwrap();
        let settings_button = creator.load_texture("assets/SETTINGSbutton.png").unwrap();
        let fade_texture = creator.load_texture("assets/gooWipe.png").unwrap();
        let player = creator.load_texture("assets/newcharsprite.png").unwrap();
        let grass1 = creator.load_texture("assets/grass1.png").unwrap();
        let grass2 = creator.load_texture("assets/grass2.png").unwrap();
        let water1 = creator.load_texture("assets/water1.png").unwrap();
        let berry = creator.load_texture("assets/berry.png").unwrap();
        let door1 = creator.load_texture("assets/door1.png").unwrap();
        let water_grass = creator.load_texture("assets/water-grass.png").unwrap();
        let wood = creator.load_texture("assets/woodcorners.png").unwrap();
        Textures {
            main_menu,
            start_button,
            load_button,
            settings_button,
            fade_texture,
            player,
            grass1,
            grass2,
            water1,
            berry,
            door1,
            water_grass,
            wood,
        }
    }
}

pub struct TileRect {
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
}

impl TileRect {
    pub fn new() -> TileRect {
        TileRect {
            wg_tl: Rect::new(0, 0, 16, 16),
            wg_t: Rect::new(16, 0, 16, 16),
            wg_tr: Rect::new(32, 0, 16, 16),
            wg_r: Rect::new(32, 16, 16, 16),
            wg_br: Rect::new(32, 32, 16, 16),
            wg_b: Rect::new(16, 32, 16, 16),
            wg_bl: Rect::new(0, 32, 16, 16),
            wg_l: Rect::new(0, 16, 16, 16),
            gw_tl: Rect::new(48, 0, 16, 16),
            gw_tr: Rect::new(80, 0, 16, 16),
            gw_br: Rect::new(80, 32, 16, 16),
            gw_bl: Rect::new(48, 32, 16, 16),
            wood_l: Rect::new(0, 0, 16, 16),
            wood_r: Rect::new(32, 0, 16, 16),
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
            display_screen: DisplayScreen::MainMenu,
            curr_button: 0,
            is_fading: false,
            did_trans: false,
            fade_anim_time: FADE_TIME,
        }
    }

    pub fn render_main_menu(&mut self, canvas: &mut Canvas<Window>, textures: &mut Textures) {
        if BUTTONS[self.curr_button] == Button::StartButton {
            textures.start_button.set_color_mod(255, 0, 0);
        } else {
            textures.start_button.set_color_mod(255, 255, 255);
        }

        if BUTTONS[self.curr_button] == Button::LoadButton {
            textures.load_button.set_color_mod(255, 0, 0);
        } else {
            textures.load_button.set_color_mod(255, 255, 255);
        }

        if BUTTONS[self.curr_button] == Button::SettingsButton {
            textures.settings_button.set_color_mod(255, 0, 0);
        } else {
            textures.settings_button.set_color_mod(255, 255, 255);
        }
        let screen_quad = Rect::new(0, 0, PIXELS_X, PIXELS_Y);
        let start_quad = Rect::new(100, 100, 32, 16);
        let load_quad = Rect::new(99, 120, 16, 16);
        let settings_quad = Rect::new(116, 120, 16, 16);

        canvas.copy(&textures.main_menu, None, screen_quad).unwrap();
        canvas
            .copy(&textures.start_button, None, start_quad)
            .unwrap();
        canvas.copy(&textures.load_button, None, load_quad).unwrap();
        canvas
            .copy(&textures.settings_button, None, settings_quad)
            .unwrap();
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
                        canvas.copy(&textures.grass1, None, render_quad).unwrap()
                    }
                    Some(tilemap::FloorTile::GRASS2) => {
                        canvas.copy(&textures.grass2, None, render_quad).unwrap()
                    }
                    Some(tilemap::FloorTile::WATER1) => {
                        canvas.copy(&textures.water1, None, render_quad).unwrap()
                    }
                    Some(tilemap::FloorTile::WG_TL) => {
                        canvas.copy(&textures.water_grass, tile_rects.wg_tl, render_quad).unwrap()
                    }
                    Some(tilemap::FloorTile::WG_T) => {
                        canvas.copy(&textures.water_grass, tile_rects.wg_t, render_quad).unwrap()
                    }
                    Some(tilemap::FloorTile::WG_TR) => {
                        canvas.copy(&textures.water_grass, tile_rects.wg_tr, render_quad).unwrap()
                    }
                    Some(tilemap::FloorTile::WG_R) => {
                        canvas.copy(&textures.water_grass, tile_rects.wg_r, render_quad).unwrap()
                    }
                    Some(tilemap::FloorTile::WG_BR) => {
                        canvas.copy(&textures.water_grass, tile_rects.wg_br, render_quad).unwrap()
                    }
                    Some(tilemap::FloorTile::WG_B) => {
                        canvas.copy(&textures.water_grass, tile_rects.wg_b, render_quad).unwrap()
                    }
                    Some(tilemap::FloorTile::WG_BL) => {
                        canvas.copy(&textures.water_grass, tile_rects.wg_bl, render_quad).unwrap()
                    }
                    Some(tilemap::FloorTile::WG_L) => {
                        canvas.copy(&textures.water_grass, tile_rects.wg_l, render_quad).unwrap()
                    }
                    Some(tilemap::FloorTile::GW_TL) => {
                        canvas.copy(&textures.water_grass, tile_rects.gw_tl, render_quad).unwrap()
                    }
                    Some(tilemap::FloorTile::GW_TR) => {
                        canvas.copy(&textures.water_grass, tile_rects.gw_tr, render_quad).unwrap()
                    }
                    Some(tilemap::FloorTile::GW_BR) => {
                        canvas.copy(&textures.water_grass, tile_rects.gw_br, render_quad).unwrap()
                    }
                    Some(tilemap::FloorTile::GW_BL) => {
                        canvas.copy(&textures.water_grass, tile_rects.gw_bl, render_quad).unwrap()
                    }
                    None => {}

                };
                match map.objects.get(i + j * map.size_x) {
                    Some(tilemap::ObjectTile::BERRY) => {
                        canvas.copy(&textures.berry, None, render_quad).unwrap()
                    }
                    Some(tilemap::ObjectTile::DOOR) => {
                        canvas.copy(&textures.door1, None, render_quad).unwrap()
                    }
                    Some(tilemap::ObjectTile::WOOD_L) => {
                        canvas.copy(&textures.wood, tile_rects.wood_l, render_quad).unwrap()
                    }
                    Some(tilemap::ObjectTile::WOOD_R) => {
                        canvas.copy(&textures.wood, tile_rects.wood_r, render_quad).unwrap()
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

    pub fn render_player(&mut self, canvas: &mut Canvas<Window>, textures: &mut Textures, player: &player::Player) {
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

    pub fn render(&mut self, canvas: &mut Canvas<Window>, textures: &mut Textures, delta_time: &f64, player: &player::Player, map: &mut tilemap::TileMap) {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        match self.display_screen {
            DisplayScreen::MainMenu => {
                self.render_main_menu(canvas, textures);
            }
            DisplayScreen::OverWorld => {
                let camera_offset = (
                    (player.pos.0 - (PIXELS_X / 2 - player::PLAYER_WIDTH / 2) as f64) as i32,
                    (player.pos.1 - (PIXELS_Y / 2 - player::PLAYER_HEIGHT / 2) as f64) as i32,
                );
                self.render_overworld_tiles(canvas, textures, map, camera_offset);
                self.render_player(canvas, textures, player);
                self.render_transition(canvas, textures, delta_time, map);
            }
        }

        canvas.present();
    }

    pub fn play_fade(&mut self) {
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
        let viewport = sdl2::rect::Rect::new(
            bb_x as i32,
            bb_y as i32,
            10,
            10,
        );
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
        let viewport = sdl2::rect::Rect::new(
            bb_x as i32,
            bb_y as i32,
            PIXELS_X,
            PIXELS_Y,
        );
        canvas.set_viewport(viewport);
    }
}
