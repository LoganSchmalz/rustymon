use sdl2::video::FullscreenType;

use crate::render::{PIXELS_X, PIXELS_Y};

use super::Renderer;

impl Renderer {
    pub fn toggle_fullscreen(&mut self) -> Result<(), String> {
        match self.canvas.window().fullscreen_state() {
            FullscreenType::Off => {
                let display = self.canvas.window().subsystem().display_bounds(0).unwrap();
                self.old_window_x = self.window_x;
                self.old_window_y = self.window_y;
                self.window_x = display.width();
                self.window_y = display.height();

                self.canvas
                    .window_mut()
                    .set_size(self.window_x, self.window_y)
                    .unwrap();
                self.canvas
                    .window_mut()
                    .set_fullscreen(FullscreenType::Desktop)?;
            }
            FullscreenType::Desktop => {
                self.window_x = self.old_window_x;
                self.window_y = self.old_window_y;

                self.canvas
                    .window_mut()
                    .set_size(self.window_x, self.window_y)
                    .unwrap();
                self.canvas
                    .window_mut()
                    .set_fullscreen(FullscreenType::Off)?;
            }
            _ => {
                return Err("Bad fullscreen state".to_string());
            }
        };
        let scale_x = self.window_x as f32 / PIXELS_X as f32;
        let scale_y = self.window_y as f32 / PIXELS_Y as f32;
        let scale = if scale_x <= scale_y {
            scale_x.floor()
        } else {
            scale_y.floor()
        } as u32;
        self.canvas.set_scale(scale as f32, scale as f32).unwrap();
        let bb_x = ((self.window_x - PIXELS_X * scale) / 2) / scale;
        let bb_y = ((self.window_y - PIXELS_Y * scale) / 2) / scale;
        let viewport = sdl2::rect::Rect::new(bb_x as i32, bb_y as i32, PIXELS_X, PIXELS_Y);
        println!("{:?}", viewport);
        self.canvas.set_viewport(viewport);

        Ok(())
    }

    pub fn resize(&mut self, width: i32, height: i32) -> Result<(), String> {
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
        self.canvas.set_scale(scale as f32, scale as f32)?;
        // put top left corner of renderer at top left scaled of "screen" to maintain aspect ratio
        let bb_x = ((self.window_x - PIXELS_X * scale) / 2) / scale; //TODO: FIX BUG THAT CRASHES GAME WHEN YOU RESIZE SCREEN TOO SMALL
        let bb_y = ((self.window_y - PIXELS_Y * scale) / 2) / scale;
        let viewport = sdl2::rect::Rect::new(bb_x as i32, bb_y as i32, PIXELS_X, PIXELS_Y);
        self.canvas.set_viewport(viewport);
        Ok(())
    }
}
