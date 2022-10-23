use sdl2::{
    image::LoadTexture,
    pixels::Color,
    rect::Rect,
    render::{Canvas, Texture, TextureCreator},
    video::{FullscreenType, Window, WindowContext},
};

use crate::{player, tilemap, TILE_SIZE};

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
    real_map: bool, //TODO: Remove this, it's just for demonstrating map transitions
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
}

impl<'a> Textures<'a> {
    pub fn load(creator: &'a TextureCreator<WindowContext>) -> Self {
        let main_menu = creator.load_texture("assets/titlescreen.png").unwrap();
        let start_button = creator.load_texture("assets/STARTbutton.png").unwrap();
        let load_button = creator.load_texture("assets/SAVELOADbutton.png").unwrap();
        let settings_button = creator.load_texture("assets/SETTINGSbutton.png").unwrap();
        let fade_texture = creator.load_texture("assets/gooWipe.png").unwrap();
        let player = creator.load_texture("assets/charSprite.png").unwrap();
        let grass1 = creator.load_texture("assets/grass1.png").unwrap();
        let grass2 = creator.load_texture("assets/grass2.png").unwrap();
        let water1 = creator.load_texture("assets/water1.png").unwrap();
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
            real_map: true,
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
        if self.real_map {
            //TODO: Remove this if else block, it's just for demonstrating map transitions (keep everything under this if but not under the else)
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
                        None => {}
                    };
                }
            }
        } else {
            for i in 0..map.size_x {
                for j in 0..map.size_y {
                    let render_quad = Rect::new(
                        i as i32 * TILE_SIZE,
                        j as i32 * TILE_SIZE,
                        TILE_SIZE as u32,
                        TILE_SIZE as u32,
                    );
                    canvas.copy(&textures.water1, None, render_quad).unwrap();
                }
            }
        }
    }

    pub fn render_transition(&mut self, canvas: &mut Canvas<Window>, textures: &mut Textures, delta_time: &f64) {
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
                //TODO: Remove this next bit, it's just for demonstrating map transitions
                if (FADE_FRAMES as f64 * (1.0 - (self.fade_anim_time / FADE_TIME) as f64)).round()
                    as i32
                    > FADE_FRAMES / 2
                    && !self.did_trans
                {
                    self.real_map = !self.real_map;
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

    pub fn render(&mut self, canvas: &mut Canvas<Window>, textures: &mut Textures, delta_time: &f64, player: &player::Player, map: &tilemap::TileMap) {
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
                self.render_transition(canvas, textures, delta_time);
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
            self.window_x - bb_x,
            self.window_y - bb_y,
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
        let bb_x = ((self.window_x - PIXELS_X * scale) / 2) / scale; //TODO: FIX BUG THAT CRASHES GAME WHEN YOU RESIZE SCREEN TOO SMALL
        let bb_y = ((self.window_y - PIXELS_Y * scale) / 2) / scale;
        let viewport = sdl2::rect::Rect::new(
            bb_x as i32,
            bb_y as i32,
            self.window_x - bb_x,
            self.window_y - bb_y,
        );
        canvas.set_viewport(viewport);
    }
}
