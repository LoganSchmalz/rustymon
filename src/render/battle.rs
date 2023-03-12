use sdl2::{pixels::Color, rect::Rect, video::WindowContext};

use crate::{font_manager::FontManager, gamestate::Battle, resource_manager::TextureManager};

use super::{Renderer, PIXELS_X, PIXELS_Y};

impl Renderer {
    pub fn render_battle(
        &mut self,
        texture_manager: &mut TextureManager<WindowContext>,
        font_manager: &FontManager,
        battle: &Battle,
    ) -> Result<(), String> {
        //self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        //self.canvas.clear();
        let background = texture_manager.load("assets/backgrounds/battlebg.png")?;
        self.canvas.copy(&background, None, None)?;

        for (index, stray) in battle.player_strays.iter().enumerate() {}

        for (index, stray) in battle.opponent_strays.iter().enumerate() {
            if let Some(stray) = stray {
                let texture = texture_manager.load("assets/strays/palliub.png")?;
                let dst = Rect::new(
                    130 + 20 * index as i32,
                    10 + 10 * index as i32,
                    texture.query().width,
                    texture.query().height,
                );
                self.canvas.copy(&texture, None, dst)?;
            }
        }

        let healthbars = texture_manager.load("assets/UI/healthbars.png")?;
        self.canvas.copy(
            &healthbars,
            None,
            Rect::new(0, 0, healthbars.query().width, healthbars.query().height),
        )?;
        self.canvas.copy(
            &healthbars,
            None,
            Rect::new(
                (PIXELS_X - healthbars.query().width) as i32,
                (PIXELS_Y - healthbars.query().height) as i32,
                healthbars.query().width,
                healthbars.query().height,
            ),
        )?;

        let creator = self.canvas.texture_creator();

        for (index, stray) in battle.player_strays.iter().enumerate() {
            if let Some(health) = stray {
                let name_surface = font_manager
                    .fonts
                    .press_start_2p
                    .render("Stray")
                    .blended(Color::RGB(40, 40, 40))
                    .map_err(|e| e.to_string())?;
                let name = creator
                    .create_texture_from_surface(name_surface)
                    .map_err(|e| e.to_string())?;
                let name_rect = Rect::new(
                    (PIXELS_X - healthbars.query().width) as i32 + 2,
                    (PIXELS_Y - healthbars.query().height) as i32 + 4 + 15 * index as i32,
                    name.query().width,
                    name.query().height,
                );
                self.canvas.copy(&name, None, name_rect)?;

                let health_rect = Rect::new(
                    (PIXELS_X - healthbars.query().width) as i32 + 3,
                    (PIXELS_Y - healthbars.query().height) as i32 + 13 + 15 * index as i32,
                    (*health as f32 / 100.0 * 90.0).ceil() as u32,
                    3,
                );
                self.canvas.set_draw_color(Color::RGB(0, 255, 0));
                self.canvas.fill_rect(health_rect)?;
            }
        }

        for (index, stray) in battle.opponent_strays.iter().enumerate() {
            if let Some(health) = stray {
                let name_surface = font_manager
                    .fonts
                    .press_start_2p
                    .render("Stray")
                    .blended(Color::RGB(40, 40, 40))
                    .map_err(|e| e.to_string())?;
                let name = creator
                    .create_texture_from_surface(name_surface)
                    .map_err(|e| e.to_string())?;
                let name_rect = Rect::new(
                    0 as i32 + 2,
                    0 as i32 + 4 + 15 * index as i32,
                    name.query().width,
                    name.query().height,
                );
                self.canvas.copy(&name, None, name_rect)?;

                let health_rect = Rect::new(
                    0 as i32 + 3,
                    0 as i32 + 13 + 15 * index as i32,
                    (*health as f32 / 100.0 * 90.0).ceil() as u32,
                    3,
                );
                self.canvas.set_draw_color(Color::RGB(0, 255, 0));
                self.canvas.fill_rect(health_rect)?;
            }
        }

        self.canvas.present();
        Ok(())
    }
}
