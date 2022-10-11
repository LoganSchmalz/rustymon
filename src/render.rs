use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window, image::LoadTexture};

use crate::{player, TILE_SIZE};

pub fn render(_delta_time: &f64, canvas: &mut Canvas<Window>, player: &player::Player) {
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