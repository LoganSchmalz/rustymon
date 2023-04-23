/****************************************************/
// Created by: Logan Schmalz
// Description: Logic for resizing the screen and maintaining the information displayed on the screen,
// this is handled specially in the renderer because it relies on the input handler and also needs to resize the window and change the canvas scale
/****************************************************/
use sdl2::video::FullscreenType;

use crate::render::{PIXELS_X, PIXELS_Y};

use super::Renderer;

impl Renderer {
    //makes the window fullscreen or windowed
    pub fn toggle_fullscreen(&mut self) -> Result<(), String> {
        match self.canvas.window().fullscreen_state() {
            FullscreenType::Off => {
                //if it's not currently fullscreen, we need to determine the size of the screen
                let display = self.canvas.window().subsystem().display_bounds(0).unwrap();
                self.old_window_x = self.window_x;
                self.old_window_y = self.window_y;
                self.window_x = display.width();
                self.window_y = display.height();

                //change the window size and state
                self.canvas
                    .window_mut()
                    .set_size(self.window_x, self.window_y)
                    .unwrap();
                self.canvas
                    .window_mut()
                    .set_fullscreen(FullscreenType::Desktop)?;
            }
            FullscreenType::Desktop => {
                //if it is currently fullscreen, we need to bring it back to the previous window size
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
                //any other states should be impossible
                return Err("Bad fullscreen state".to_string());
            }
        };
        //after resizing the window, we need to recalculate the canvas scaling
        let scale_x = self.window_x as f32 / PIXELS_X as f32;
        let scale_y = self.window_y as f32 / PIXELS_Y as f32;
        let scale = if scale_x <= scale_y {
            scale_x.floor()
        } else {
            scale_y.floor()
        } as u32;
        self.canvas.set_scale(scale as f32, scale as f32).unwrap();
        //we also need to recalculate our black bars to maintain the intended aspect ratio
        let bb_x = ((self.window_x - PIXELS_X * scale) / 2) / scale;
        let bb_y = ((self.window_y - PIXELS_Y * scale) / 2) / scale;
        let viewport = sdl2::rect::Rect::new(bb_x as i32, bb_y as i32, PIXELS_X, PIXELS_Y);
        println!("{:?}", viewport);
        self.canvas.set_viewport(viewport);

        Ok(())
    }

    //this function handles regular window resizing, similarly to above it needs to recalculate the canvas scale and black bars
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
