<<<<<<< HEAD
use sdl2::{
    pixels::Color,
    rect::Rect,
    render::Canvas,
    video::{FullscreenType, Window},
};

use crate::{player, TILE_SIZE};

pub const PIXELS_X: u32 = 240;
pub const PIXELS_Y: u32 = 160;

pub struct Renderer {
    window_x: u32,
    window_y: u32,
    old_window_x: u32,
    old_window_y: u32,
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            window_x: PIXELS_X,
            window_y: PIXELS_Y,
            old_window_x: PIXELS_X,
            old_window_y: PIXELS_Y,
        }
    }

    pub fn render(&self, _delta_time: &f64, canvas: &mut Canvas<Window>, player: &player::Player) {
        let texture_creator = canvas.texture_creator();
        let grass1 = texture_creator.load_texture("assets/grass1.png").unwrap();
        let grass2 = texture_creator.load_texture("assets/grass2.png").unwrap();
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        for i in 0..(240 / TILE_SIZE) {
            for j in 0..(160 / TILE_SIZE) {
                let render_quad = Rect::new(
                    i * TILE_SIZE,
                    j * TILE_SIZE,
                    TILE_SIZE as u32,
                    TILE_SIZE as u32,);
                if (i + j) % 2 == 0 {
                    canvas.copy(&grass1, None, render_quad)
                    .unwrap();
                } else {
                    canvas.copy(&grass2, None, render_quad).unwrap();
                }
            }
        }

        player.render(canvas);
        
        canvas.present();
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