/****************************************************/
// Created by: Logan Schmalz
// Description: Logic for rendering for all menus in the game
/****************************************************/
use hecs::World;
use sdl2::{pixels::Color, rect::Rect, video::WindowContext};

use crate::{
    components::bag::Bag,
    font_manager::FontManager,
    menu::{
        bag_menu::BagMenu,
        main_menu::{MainMenu, MainMenuButton},
        moves_menu::MovesMenu,
        pause_menu::PauseMenu,
        textbox::Textbox,
    },
    resource_manager::TextureManager,
};

use super::{Renderer, PIXELS_X, PIXELS_Y};

impl Renderer {
    pub(super) fn render_bag_menu(
        &mut self,
        menu: &BagMenu,
        world: &World,
        texture_manager: &mut TextureManager<WindowContext>,
        font_man: &FontManager,
    ) -> Result<(), String> {
        // create new quad over the textbox texture (which is 41 px tall)
        //let box_quad = Rect::new(0, (PIXELS_Y - 41) as i32, PIXELS_X, 41 as u32);
        let box_quad = Rect::new(0, 0, PIXELS_X, PIXELS_Y);

        let texture = texture_manager.load("assets/UI/bag_menu.png")?;
        //let berry_texture = texture_manager.load("assets/tiles/objectsprites.png")?; //hard-coding berry texture currently
        //let berry_rect = Rect::new(32, 0, 16, 16);
        //let berry_dst = Rect::new(104, 6, 16, 16);

        self.canvas.copy(&texture, None, box_quad)?;
        //self.canvas.copy(&berry_texture, berry_rect, berry_dst)?;

        let mut text_quad = Rect::new(140, 10, 0, 0);

        let mut query = world.query_one::<&Bag>(menu.entity).unwrap();
        let bag = query.get();
        let empty = vec![];
        let items = match bag {
            Some(bag) => &bag.items,
            _ => &empty,
        };

        // loop to enable rendering the fonts and structure of the bag menu
        // All of these are creating surfaces and titles for each item in the bag (each sub-menu)
        for (idx, (item, amount)) in items.iter().enumerate() {
            let item_str = item.to_string();

            // item name
            let item_surface = font_man.fonts.munro.render(&item_str);
            let item_surface = if idx == menu.selected {
                item_surface.blended(Color::RGB(0, 183, 235))
            } else {
                item_surface.blended(Color::RGB(40, 40, 40))
            };
            let item_surface = item_surface.map_err(|e| e.to_string())?;

            // item amount
            let amount_str = amount.to_string();
            let amount_surface = font_man.fonts.munro.render(&amount_str);
            let amount_surface = if idx == menu.selected {
                amount_surface.blended(Color::RGB(0, 183, 235))
            } else {
                amount_surface.blended(Color::RGB(40, 40, 40))
            };
            let amount_surface = amount_surface.map_err(|e| e.to_string())?;

            text_quad.set_width(item_surface.width());
            text_quad.set_height(item_surface.height());

            let amount_quad = Rect::new(
                PIXELS_X as i32 - 10 - amount_surface.width() as i32,
                text_quad.y,
                amount_surface.width(),
                amount_surface.height(),
            );

            let creator = self.canvas.texture_creator();
            let item_texture = creator
                .create_texture_from_surface(&item_surface)
                .map_err(|e| e.to_string())?;
            let amount_texture = creator
                .create_texture_from_surface(&amount_surface)
                .map_err(|e| e.to_string())?;

            // render textures
            self.canvas.copy(&item_texture, None, text_quad)?;
            self.canvas.copy(&amount_texture, None, amount_quad)?;

            text_quad.set_y(text_quad.y + item_surface.height() as i32);
        }

        Ok(())
    }

    pub fn render_main_menu(
        &mut self,
        menu: &MainMenu,
        texture_manager: &mut TextureManager<WindowContext>,
        _font_man: &FontManager,
    ) -> Result<(), String> {
        // load textures into the texture manager
        let titlescreen = texture_manager.load("assets/backgrounds/titlescreen.png")?;
        let start_button = texture_manager.load("assets/UI/STARTbutton.png")?;
        let load_button = texture_manager.load("assets/UI/SAVELOADbutton.png")?;
        let settings_button = texture_manager.load("assets/UI/SETTINGSbutton.png")?;

        // render the main menu background textures
        // change button colors based on selected button
        let start_src = if menu.curr_button == MainMenuButton::Start {
            Rect::new(0, 24, 72, 24)
        } else {
            Rect::new(0, 0, 72, 24)
        };

        let load_src = if menu.curr_button == MainMenuButton::Load {
            Rect::new(0, 18, 16, 18)
        } else {
            Rect::new(0, 0, 16, 18)
        };

        let settings_src = if menu.curr_button == MainMenuButton::Settings {
            Rect::new(0, 18, 16, 18)
        } else {
            Rect::new(0, 0, 16, 18)
        };

        let screen_quad = Rect::new(0, 0, PIXELS_X, PIXELS_Y);
        let start_quad = Rect::new(
            PIXELS_X as i32 / 2 - start_src.width() as i32 / 2,
            100,
            start_src.width(),
            start_src.height(),
        );
        let load_quad = Rect::new(102, 122, 16, 18);
        let settings_quad = Rect::new(121, 122, 16, 18);

        // render
        self.canvas.copy(&titlescreen, None, screen_quad)?;
        self.canvas.copy(&start_button, start_src, start_quad)?;
        self.canvas.copy(&load_button, load_src, load_quad)?;
        self.canvas
            .copy(&settings_button, settings_src, settings_quad)?;

        self.canvas.present();

        Ok(())
    }

    pub(super) fn render_textbox(
        &mut self,
        textbox: &Textbox,
        texture_manager: &mut TextureManager<WindowContext>,
        font_man: &FontManager,
    ) -> Result<(), String> {
        // create new quad over the textbox texture (which is 41 px tall)
        let box_quad = Rect::new(0, (PIXELS_Y - 41) as i32, PIXELS_X, 41u32);

        // renders layered textbox attributes onto textbox texture
        let surface_top = font_man
            .fonts
            .munro
            .render(textbox.text_v[0].as_str())
            .blended(Color::RGB(40, 40, 40))
            .map_err(|e| e.to_string())?;

        let surface_bot = font_man
            .fonts
            .munro
            .render(textbox.text_v[1].as_str())
            .blended(Color::RGB(40, 40, 40))
            .map_err(|e| e.to_string())?;

        let text_quad_top = Rect::new(
            10,
            (PIXELS_Y - 41) as i32 + 7,
            surface_top.width(),
            surface_top.height(),
        );

        let text_quad_bot = Rect::new(
            10,
            (PIXELS_Y - 41) as i32 + 10 + surface_top.height() as i32 - 3,
            surface_bot.width(),
            surface_bot.height(),
        );

        let text_box = texture_manager.load("assets/UI/text_box.png")?;

        let creator = self.canvas.texture_creator();
        let texture_top = creator
            .create_texture_from_surface(&surface_top)
            .map_err(|e| e.to_string())?;
        let texture_bot = creator
            .create_texture_from_surface(&surface_bot)
            .map_err(|e| e.to_string())?;
        // copy the different textbox components into the canvas
        self.canvas.copy(&text_box, None, box_quad)?;
        self.canvas.copy(&texture_top, None, text_quad_top)?;
        self.canvas.copy(&texture_bot, None, text_quad_bot)?;

        Ok(())
    }

    pub(super) fn render_pause_menu(
        &mut self,
        menu: &PauseMenu,
        texture_manager: &mut TextureManager<WindowContext>,
        font_man: &FontManager,
    ) -> Result<(), String> {
        // create new quad over the textbox texture (which is 41 px tall)
        //let box_quad = Rect::new(0, (PIXELS_Y - 41) as i32, PIXELS_X, 41 as u32);
        let box_quad = Rect::new(0, 0, PIXELS_X, PIXELS_Y);

        let pause_menu = texture_manager.load("assets/UI/pause_menu.png")?;

        self.canvas.copy(&pause_menu, None, box_quad)?;

        let mut text_quad = Rect::new(180, 10, 0, 0);

        // render the textures for the pause menu
        for (idx, item) in menu.items.iter().enumerate() {
            let surface = font_man.fonts.munro.render(item);
            // generates surface rectangles for all of the pause menu items
            let surface = if idx == menu.selected {
                surface.blended(Color::RGB(0, 183, 235))
            } else {
                surface.blended(Color::RGB(40, 40, 40))
            };
            let surface = surface.map_err(|e| e.to_string())?;
        
            text_quad.set_width(surface.width());
            text_quad.set_height(surface.height());

            let creator = self.canvas.texture_creator();
            let texture = creator
                .create_texture_from_surface(&surface)
                .map_err(|e| e.to_string())?;

            // renders to canvas
            self.canvas.copy(&texture, None, text_quad)?;

            text_quad.set_y(text_quad.y + surface.height() as i32 + 4);
        }
        Ok(())
    }

    pub(super) fn render_moves_menu(
        //function for rendering the moves menu
        &mut self,
        menu: &MovesMenu,
        texture_manager: &mut TextureManager<WindowContext>,
        font_man: &FontManager,
    ) -> Result<(), String> {
        let box_quad = Rect::new(0, (PIXELS_Y - 66) as i32, 132, 66); //box in bottom right corner
        let moves_menu = texture_manager.load("assets/UI/moves_menu.png")?; //load moves menu image as a texture
        self.canvas.copy(&moves_menu, None, box_quad)?;

        let mut text_quad = Rect::new(10, (PIXELS_Y - 56) as i32, 132, 66);

        for (idx, mv) in menu.moves.iter().enumerate() {
            //mv = move
            // for given moves, render into the menu box
            if let Some(move_data) = mv {
                let surface = font_man.fonts.munro.render(&move_data.name); //render the name of the move

                let surface = if idx == menu.selected {
                    surface.blended(Color::RGB(212, 101, 99))
                } else {
                    surface.blended(Color::RGB(49, 41, 36))
                };

                let surface = surface.map_err(|e| e.to_string())?;

                text_quad.set_width(surface.width());
                text_quad.set_height(surface.height());

                let creator = self.canvas.texture_creator();
                let texture = creator
                    .create_texture_from_surface(&surface)
                    .map_err(|e| e.to_string())?;

                self.canvas.copy(&texture, None, text_quad)?;
            }
            if idx % 2 == 0 {
                text_quad.set_x(text_quad.x + 64 as i32)
            } else {
                text_quad.set_x(text_quad.x - 64 as i32);
                text_quad.set_y(text_quad.y + 27 as i32);
            }
        }

        Ok(())
    }
}
