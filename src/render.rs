use sdl2::{
    pixels::Color,
    rect::Rect,
    render::Canvas,
    video::{FullscreenType, Window}, image::LoadTexture,
};

use crate::{player, TILE_SIZE, tilemap};

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

pub const BUTTONS: [Button; 3] = [Button::StartButton, Button::LoadButton, Button::SettingsButton];

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

    pub fn render(&mut self, _delta_time: &f64, canvas: &mut Canvas<Window>, player: &player::Player, map: &tilemap::TileMap) {
        let texture_creator = canvas.texture_creator();
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        match self.display_screen {
            DisplayScreen::MainMenu => {
                let main_menu = texture_creator.load_texture("assets/titlescreen.png").unwrap();
                let mut start_button = texture_creator.load_texture("assets/STARTbutton.png").unwrap();
                if BUTTONS[self.curr_button] == Button::StartButton {
                    start_button.set_color_mod(255, 0, 0);
                }
                let mut load_button = texture_creator.load_texture("assets/SAVELOADbutton.png").unwrap();
                if BUTTONS[self.curr_button] == Button::LoadButton {
                    load_button.set_color_mod(255, 0, 0);
                }
                let mut settings_button = texture_creator.load_texture("assets/SETTINGSbutton.png").unwrap();
                if BUTTONS[self.curr_button] == Button::SettingsButton {
                    settings_button.set_color_mod(255, 0, 0);
                }
                let screen_quad = Rect::new(0, 0, map.size_x as u32 * TILE_SIZE as u32, map.size_y as u32 * TILE_SIZE as u32);
                let start_quad = Rect::new(100, 100, 32, 16);
                let load_quad = Rect::new(99, 120, 16, 16);
                let settings_quad = Rect::new(116, 120, 16, 16);
                canvas.copy(&main_menu, None, screen_quad).unwrap();
                canvas.copy(&start_button, None, start_quad).unwrap();
                canvas.copy(&load_button, None, load_quad).unwrap();
                canvas.copy(&settings_button, None, settings_quad).unwrap();
            }
            DisplayScreen::OverWorld => {
                let grass1 = texture_creator.load_texture("assets/grass1.png").unwrap();
                let grass2 = texture_creator.load_texture("assets/grass2.png").unwrap();
                let water1 = texture_creator.load_texture("assets/water1.png").unwrap();
                if self.real_map {//TODO: Remove this if else block, it's just for demonstrating map transitions (keep everything under this if but not under the else)
                    for i in 0..map.size_x {
                        for j in 0..map.size_y {
                            let render_quad = Rect::new(
                                i as i32 * TILE_SIZE,
                                j as i32 * TILE_SIZE,
                                TILE_SIZE as u32,
                                TILE_SIZE as u32,);
                            match map.floor.get(i + j*map.size_x) {
                                Some(tilemap::FloorTile::GRASS1) => canvas.copy(&grass1, None, render_quad).unwrap(),
                                Some(tilemap::FloorTile::GRASS2) => canvas.copy(&grass2, None, render_quad).unwrap(),
                                Some(tilemap::FloorTile::WATER1) => canvas.copy(&water1, None, render_quad).unwrap(),
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
                                TILE_SIZE as u32,);
                            canvas.copy(&water1, None, render_quad).unwrap();
                        }
                    }
                }

                player.render(canvas);

                if self.is_fading {
                    self.fade_anim_time = self.fade_anim_time - _delta_time;
                    if self.fade_anim_time <= 0.0 {
                        self.is_fading = false;
                    } else {
                        //might be timing issues here (starts at -_delta_time instead of the actual beginning)
                        let curr_fade_frame: i32 = (FADE_FRAMES as f64 * (1.0 - (self.fade_anim_time / FADE_TIME) as f64)).round() as i32;
                        let fade_texture = texture_creator.load_texture("assets/gooWipe.png").unwrap();
                        let screen_quad = Rect::new(0, 0, map.size_x as u32 * TILE_SIZE as u32, map.size_y as u32 * TILE_SIZE as u32); //TODO: change height and width of screen_quad to not require math
                        let fade_slice = Rect::new(240 * curr_fade_frame, 0, 240, 160);
                        canvas.copy(&fade_texture, fade_slice, screen_quad).unwrap();
                        //TODO: Remove this next bit, it's just for demonstrating map transitions
                        if (FADE_FRAMES as f64 * (1.0 - (self.fade_anim_time / FADE_TIME) as f64)).round() as i32 > FADE_FRAMES / 2  && !self.did_trans{
                            self.real_map = !self.real_map;
                            self.did_trans = true;
                        }
                    }
                }
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