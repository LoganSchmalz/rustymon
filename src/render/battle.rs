use hecs::World;
use sdl2::{pixels::Color, rect::Rect, video::WindowContext};

use crate::{font_manager::FontManager, gamestate::Battle, menu, resource_manager::TextureManager};

use super::{Renderer, Transition, PIXELS_X, PIXELS_Y};

impl Renderer {
    pub fn render_battle(
        &mut self,
        texture_manager: &mut TextureManager<WindowContext>,
        font_manager: &FontManager,
        delta_time: f32,
        battle: &Battle,
        menu_man: &mut menu::MenuManager,
        world: &World,
    ) -> Result<bool, String> {
        //self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        //self.canvas.clear();
        let background = texture_manager.load("assets/backgrounds/battlebg.png")?;
        self.canvas.copy(&background, None, None)?;

        for (index, stray) in battle.player_strays.iter().enumerate() {
            if let Some(stray_data) = stray {
                let texture = texture_manager.load(&stray_data.texture)?;
                let dst = Rect::new(
                    20 * index as i32,
                    70 + 10 * index as i32,
                    texture.query().width / 2,
                    texture.query().height,
                );
                let slice = Rect::new(
                    (texture.query().width / 2) as i32,
                    0,
                    texture.query().width / 2,
                    texture.query().height,
                );
                self.canvas.copy(&texture, slice, dst)?;
            }
        }

        for (index, stray) in battle.opponent_strays.iter().enumerate() {
            if let Some(stray_data) = stray {
                let texture = texture_manager.load(&stray_data.texture)?;
                let dst = Rect::new(
                    130 + 20 * index as i32,
                    10 + 10 * index as i32,
                    texture.query().width / 2,
                    texture.query().height,
                );
                let slice = Rect::new(0, 0, texture.query().width / 2, texture.query().height);
                self.canvas.copy(&texture, slice, dst)?;
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
        let healthbar = texture_manager.load("assets/UI/healthbar.png")?;

        for (index, stray) in battle.player_strays.iter().enumerate() {
            if let Some(stray_data) = stray {
                let name_surface = font_manager
                    .fonts
                    .munro
                    .render(&stray_data.species)
                    .blended(Color::RGB(31, 27, 24))
                    .map_err(|e| e.to_string())?;
                let name = creator
                    .create_texture_from_surface(name_surface)
                    .map_err(|e| e.to_string())?;
                let name_rect = Rect::new(
                    (PIXELS_X - healthbars.query().width) as i32 + 8,
                    (PIXELS_Y - healthbars.query().height) as i32 + 15 * index as i32,
                    name.query().width,
                    name.query().height,
                );
                self.canvas.copy(&name, None, name_rect)?;

                if stray_data.cur_hp > 0 {
                    let health_pixels = (stray_data.cur_hp as f32 / stray_data.hp as f32 * 78.0).ceil() as u32;
                    let health_slice = Rect::new( //cropping the healthbar png based on health percentage
                        (79 - health_pixels) as i32,
                        0 as i32,
                        health_pixels,
                        4
                    );
                    let health_rect = Rect::new(
                        (PIXELS_X - healthbars.query().width) as i32 + 6,
                        (PIXELS_Y - healthbars.query().height) as i32 + 12 + 15 * index as i32,
                        (stray_data.cur_hp as f32 / stray_data.hp as f32 * 78.0).ceil() as u32,
                        //render health as a fraction of the whole health bar
                        4,
                    );
                    self.canvas.copy(&healthbar, health_slice, health_rect); //filling the healthbar with the healthbar png
                    //self.canvas.fill_rect(health_rect)?;
                }
            }
        }

        for (index, stray) in battle.opponent_strays.iter().enumerate() {
            if let Some(stray_data) = stray {
                let name_surface = font_manager
                    .fonts
                    .munro
                    .render(&stray_data.species)
                    .blended(Color::RGB(31, 27, 24))
                    .map_err(|e| e.to_string())?;
                let name = creator
                    .create_texture_from_surface(name_surface)
                    .map_err(|e| e.to_string())?;
                let name_rect = Rect::new(
                    0 as i32 + 8,
                    0 as i32 + 15 * index as i32,
                    name.query().width,
                    name.query().height,
                );
                self.canvas.copy(&name, None, name_rect)?;

                if stray_data.cur_hp > 0 {
                    let health_pixels = (stray_data.cur_hp as f32 / stray_data.hp as f32 * 78.0).ceil() as u32;
                    let health_slice = Rect::new( //cropping the healthbar png based on health percentage
                        (79 - health_pixels) as i32,
                        0 as i32,
                        health_pixels,
                        4
                    );
                    let health_rect = Rect::new(
                        0 as i32 + 6,
                        0 as i32 + 12 + 15 * index as i32,
                        health_pixels,
                        //render health as a fraction of the whole health bar
                        4,
                    );
                    self.canvas.copy(&healthbar, health_slice, health_rect);
                }
            }
        }
        self.render_menus(world, texture_manager, font_manager, menu_man)?; //render menu (either moves menu or enemy selection)
        let did_trans = if self.is_fading {
            self.render_transition(texture_manager, delta_time, self.trans)?
        } else { false };
        self.canvas.present();
        Ok(did_trans)
    }

    pub fn render_win(
        &mut self,
        texture_manager: &mut TextureManager<WindowContext>,
        delta_time: f32,
    ) -> Result<(), String> {
        //function to render win screen
        let background = texture_manager.load("assets/backgrounds/winscreen.png")?;
        self.canvas.copy(&background, None, None)?;
        self.canvas.present();
        Ok(())
    }

    pub fn render_loss(
        &mut self,
        texture_manager: &mut TextureManager<WindowContext>,
        delta_time: f32,
    ) -> Result<(), String> {
        //function to render loss screen
        let background = texture_manager.load("assets/backgrounds/lossscreen.png")?;
        self.canvas.copy(&background, None, None)?;
        self.canvas.present();
        Ok(())
    }
}
