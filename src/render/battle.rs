/****************************************************/
// Created by: Logan Schmalz
// Description: Logic for rendering the battle scene
// including background, strays, menus, and other battle UI elements
/****************************************************/
use hecs::World;
use sdl2::{pixels::Color, rect::Rect, video::WindowContext};

use crate::{
    font_manager::FontManager,
    gamestate::battle::{Battle, BattleState},
    menu,
    resource_manager::TextureManager,
};

use super::{Renderer, PIXELS_X, PIXELS_Y};

impl Renderer {
    pub fn render_battle(
        &mut self,
        texture_manager: &mut TextureManager<WindowContext>,
        font_manager: &FontManager,
        battle: &Battle,
        menu_man: &mut menu::MenuManager,
        world: &World,
    ) -> Result<(), String> {
        let background = texture_manager.load("assets/backgrounds/battlebg.png")?;
        self.canvas.copy(&background, None, None)?;

        for (index, stray) in battle.player_strays.iter().enumerate() {
            if let Some(stray_data) = stray {
                if matches!(battle.battle_state, BattleState::SelectingFriendlyStray) {
                    if let Some(i) = battle.selected_stray {
                        if i == index {
                            let texture = texture_manager.load("assets/UI/team_select.png")?;
                            let dst = Rect::new(
                                -5 + 20 * index as i32,
                                45 + 70 + 10 * index as i32,
                                texture.query().width,
                                texture.query().height,
                            );
                            self.canvas.copy(&texture, None, dst)?;
                        }
                    }
                }

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
                if matches!(battle.battle_state, BattleState::SelectingOpponentStray) {
                    if let Some(i) = battle.selected_stray {
                        if i == index {
                            let texture = texture_manager.load("assets/UI/enemy_select.png")?;
                            let dst = Rect::new(
                                -5 + 110 + 20 * index as i32,
                                45 + 10 + 10 * index as i32,
                                texture.query().width,
                                texture.query().height,
                            );
                            self.canvas.copy(&texture, None, dst)?;
                        }
                    }
                }

                let texture = texture_manager.load(&stray_data.texture)?;
                let dst = Rect::new(
                    110 + 20 * index as i32,
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
                (PIXELS_X - healthbars.query().width) as i32 + 6,
                (PIXELS_Y - healthbars.query().height) as i32,
                healthbars.query().width,
                healthbars.query().height,
            ),
        )?;

        let creator = self.canvas.texture_creator();
        let healthbar = texture_manager.load("assets/UI/healthbar.png")?;

        for (index, stray) in battle.player_strays.iter().enumerate() {
            if let Some(stray_data) = stray {
                let text_color = //text color for stray name based on whether or not it is their turn currentl
                if battle.player_strays[battle.turn_order[0]].as_ref().unwrap().species == stray_data.species { //if it is the stray's turn
                    Color::RGB(167, 84, 94) //red
                } else {
                    Color::RGB(31, 27, 24) //black
                };
                let name_surface = font_manager
                    .fonts
                    .munro
                    .render(&stray_data.species)
                    .blended(text_color)
                    .map_err(|e| e.to_string())?;
                let name = creator
                    .create_texture_from_surface(name_surface)
                    .map_err(|e| e.to_string())?;
                let name_rect = Rect::new(
                    (PIXELS_X - healthbars.query().width) as i32 + 8 + 6,
                    (PIXELS_Y - healthbars.query().height) as i32 + 15 * index as i32,
                    name.query().width,
                    name.query().height,
                );
                self.canvas.copy(&name, None, name_rect)?;

                if stray_data.cur_hp > 0 {
                    let health_pixels =
                        (stray_data.cur_hp as f32 / stray_data.hp as f32 * 78.0).ceil() as u32;
                    let health_slice = Rect::new(
                        //cropping the healthbar png based on health percentage
                        79 - health_pixels as i32,
                        0 as i32,
                        health_pixels,
                        4,
                    );
                    let health_rect = Rect::new(
                        (PIXELS_X - healthbars.query().width) as i32 + 6 + 6,
                        (PIXELS_Y - healthbars.query().height) as i32 + 12 + 15 * index as i32,
                        (stray_data.cur_hp as f32 / stray_data.hp as f32 * 78.0).ceil() as u32,
                        //render health as a fraction of the whole health bar
                        4,
                    );
                    self.canvas.copy(&healthbar, health_slice, health_rect)?; //filling the healthbar with the healthbar png
                                                                              //self.canvas.fill_rect(health_rect)?;
                }
            }
        }

        for (index, stray) in battle.opponent_strays.iter().enumerate() {
            if let Some(stray_data) = stray {
                let mut text_color = Color::RGB(31, 27, 24); //black
                                                             //text color for stray name based on whether or not it is their turn currentl

                if battle.turn_order[0] > 3 {
                    if battle.opponent_strays[battle.turn_order[0] - 4]
                        .as_ref()
                        .unwrap()
                        .species
                        == stray_data.species
                    {
                        //if it is the stray's turn
                        text_color = Color::RGB(167, 84, 94); //red
                    }
                }

                let name_surface = font_manager
                    .fonts
                    .munro
                    .render(&stray_data.species)
                    .blended(text_color)
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
                    let health_pixels =
                        (stray_data.cur_hp as f32 / stray_data.hp as f32 * 78.0).ceil() as u32;
                    let health_slice = Rect::new(
                        //cropping the healthbar png based on health percentage
                        79 - health_pixels as i32,
                        0 as i32,
                        health_pixels,
                        4,
                    );
                    let health_rect = Rect::new(
                        0 as i32 + 6,
                        0 as i32 + 12 + 15 * index as i32,
                        health_pixels,
                        //render health as a fraction of the whole health bar
                        4,
                    );
                    self.canvas.copy(&healthbar, health_slice, health_rect)?;
                }
            }
        }
        self.render_menus(world, texture_manager, font_manager, &battle.menus)?; //render menu (either moves menu or enemy selection)

        Ok(())
    }

    pub fn _render_win(
        &mut self,
        texture_manager: &mut TextureManager<WindowContext>,
    ) -> Result<(), String> {
        //function to render win screen
        let background = texture_manager.load("assets/backgrounds/winscreen.png")?;
        self.canvas.copy(&background, None, None)?;

        Ok(())
    }

    pub fn _render_loss(
        &mut self,
        texture_manager: &mut TextureManager<WindowContext>,
    ) -> Result<(), String> {
        //function to render loss screen
        let background = texture_manager.load("assets/backgrounds/lossscreen.png")?;
        self.canvas.copy(&background, None, None)?;

        Ok(())
    }
}
