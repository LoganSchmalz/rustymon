use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

use crate::{player, TILE_SIZE};

pub fn render(_delta_time: &f64, canvas: &mut Canvas<Window>, player: &player::Player) {
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    for i in 0..(240 / TILE_SIZE) {
        for j in 0..(160 / TILE_SIZE) {
            if (i + j) % 2 == 0 {
                canvas.set_draw_color(Color::RGB(255, 255, 255));
                canvas
                    .fill_rect(Rect::new(
                        i * TILE_SIZE,
                        j * TILE_SIZE,
                        TILE_SIZE as u32,
                        TILE_SIZE as u32,
                    ))
                    .unwrap();
            } else {
                canvas.set_draw_color(Color::RGB(255, 0, 0));
                canvas
                    .fill_rect(Rect::new(
                        i * TILE_SIZE,
                        j * TILE_SIZE,
                        TILE_SIZE as u32,
                        TILE_SIZE as u32,
                    ))
                    .unwrap();
            }
        }
    }

    player.render(canvas);
	
	canvas.present();
}
