use sdl2::{
    pixels::Color,
    rect::Rect,
    render::Canvas,
    video::{FullscreenType, Window}, image::LoadTexture,
};

use crate::{player, TILE_SIZE, tilemap};

pub const PIXELS_X: u32 = 240;
pub const PIXELS_Y: u32 = 160;

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
    display_screen: DisplayScreen,
    curr_button: usize,
}

static buttons: [Button; 3] = [Button::StartButton, Button::LoadButton, Button::SettingsButton];

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            window_x: PIXELS_X,
            window_y: PIXELS_Y,
            old_window_x: PIXELS_X,
            old_window_y: PIXELS_Y,
            display_screen: DisplayScreen::MainMenu,
            curr_button: 0,
        }
    }

    pub fn render(&self, _delta_time: &f64, canvas: &mut Canvas<Window>, player: &player::Player, map: &tilemap::TileMap) {
        let texture_creator = canvas.texture_creator();
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        match self.display_screen {
            DisplayScreen::MainMenu => {
                let main_menu = texture_creator.load_texture("assets/titlescreen.png").unwrap();
                let mut start_button = texture_creator.load_texture("assets/STARTbutton.png").unwrap();
                if buttons[self.curr_button] == Button::StartButton {
                    start_button.set_color_mod(255, 0, 0);
                }
                let mut load_button = texture_creator.load_texture("assets/SAVELOADbutton.png").unwrap();
                if buttons[self.curr_button] == Button::LoadButton {
                    load_button.set_color_mod(255, 0, 0);
                }
                let mut settings_button = texture_creator.load_texture("assets/SETTINGSbutton.png").unwrap();
                if buttons[self.curr_button] == Button::SettingsButton {
                    settings_button.set_color_mod(255, 0, 0);
                }
                canvas.copy(&main_menu, None, None).unwrap();
                let start_quad = Rect::new(100, 100, 32, 16);
                let load_quad = Rect::new(99, 120, 16, 16);
                let settings_quad = Rect::new(116, 120, 16, 16);
                canvas.copy(&start_button, None, start_quad).unwrap();
                canvas.copy(&load_button, None, load_quad).unwrap();
                canvas.copy(&settings_button, None, settings_quad).unwrap();
            }
            DisplayScreen::OverWorld => {
                let grass1 = texture_creator.load_texture("assets/grass1.png").unwrap();
                let grass2 = texture_creator.load_texture("assets/grass2.png").unwrap();
                let water1 = texture_creator.load_texture("assets/water1.png").unwrap();
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

                player.render(canvas);
            }
        }
        
        canvas.present();
    }

    pub fn next_button(&mut self) {
        self.curr_button = (self.curr_button + 1) % 3;
    }

    pub fn prev_button(&mut self) {
        if self.curr_button == 0 {
            self.curr_button = 2;
        } else {
            self.curr_button = self.curr_button - 1;
        }
    }

    pub fn select_button(&mut self) { //TODO: MOVE TO input.rs
        if self.display_screen == DisplayScreen::MainMenu && buttons[self.curr_button] == Button::StartButton {
            self.display_screen = DisplayScreen::OverWorld;
        }
    }

    pub fn get_display_screen(&mut self) -> &DisplayScreen {
        return &self.display_screen;
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
}