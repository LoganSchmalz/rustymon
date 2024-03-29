/****************************************************/
// Created by: Logan Schmalz
// Description: Logic for rendering overworld
// including logic for the camera and rendering overworld tiles
/****************************************************/
use hecs::World;
use sdl2::{pixels::Color, rect::Rect, video::WindowContext};

use crate::{
    components::{animation::HumanWalkAnimation, sprite::Sprite, Player, Position, Collision},
    constants::TILE_SIZE,
    font_manager::FontManager,
    menu,
    resource_manager::TextureManager,
    tilemap::{self, FloorTile, WallTile, CollisionTile},
    vec2::Vec2,
};

use super::{Renderer, PIXELS_X, PIXELS_Y};

#[derive(Default)]
pub struct Camera {
    offset: (i32, i32),
    top_left: (i32, i32),
    bottom_right: (i32, i32),
}

impl Renderer {
    //renders the overworld (the outside world in the game)
    //takes in the texture manager, the font manager, the world, the current map, and the menu manager, return ok if no problems
    pub fn render_overworld(
        &mut self,
        texture_manager: &mut TextureManager<WindowContext>,
        font_man: &FontManager,
        world: &World,
        map: &mut tilemap::TileMap,
        menu_man: &mut menu::MenuManager,
    ) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RGB(39, 45, 46));
        self.canvas.clear();

        self.update_camera(world)?;
        self.render_overworld_tiles(texture_manager, map)?;
        self.render_entities(world, texture_manager)?;
        self.render_front_overworld_tiles(texture_manager, map)?;
        self.render_menus(world, texture_manager, font_man, menu_man)?;

        Ok(())
    }

    //changes the camera offset to stay focussed on the player
    //takes in the world, returns ok if no problems
    pub fn update_camera(&mut self, world: &World) -> Result<(), String> {
        let mut q = world.query::<(&Player, &Position, &Sprite)>();
        let (_, (_player, Position(pos), sprite)) = q.iter().next().ok_or("No player found")?;

        let offset = (
            (pos.0 * TILE_SIZE as f32).round() as i32
                - (PIXELS_X / 2 - sprite.src.width() / 2) as i32,
            (pos.1 * TILE_SIZE as f32).round() as i32
                - (PIXELS_Y / 2 - sprite.src.height() / 2) as i32,
        );
        let top_left = ((pos.0 - 8.0).floor() as i32, (pos.1 - 5.0).floor() as i32);
        let bottom_right = ((pos.0 + 8.0).ceil() as i32, (pos.1 + 5.0).ceil() as i32);

        self.camera = Camera {
            offset,
            top_left,
            bottom_right,
        };

        Ok(())
    }

    //renders the entities in the world
    //takes in the world and the texture_manager, returns ok if no problems
    pub fn render_entities(
        &mut self,
        world: &World,
        texture_manager: &mut TextureManager<WindowContext>,
    ) -> Result<(), String> {
        let mut entity_query = world.query::<(&Position, &Sprite, Option<&HumanWalkAnimation>)>();

        let mut list = entity_query
            .iter()
            .filter(|(_, (Position(c), ..))| {
                Vec2::from(self.camera.top_left) <= *c && *c <= Vec2::from(self.camera.bottom_right)
            })
            .collect::<Vec<_>>();
        list.sort_by(|(_, (Position(c1), ..)), (_, (Position(c2), ..))| {
            c1.partial_cmp(c2).unwrap()
        });

        for (_, (Position(Vec2(x, y)), sprite, anim)) in list {
            let render_quad = Rect::new(
                (*x * TILE_SIZE as f32).round() as i32 - self.camera.offset.0 + sprite.shift_x,
                (*y * TILE_SIZE as f32).round() as i32 - self.camera.offset.1 + sprite.shift_y,
                sprite.src.width(),
                sprite.src.height(),
            );

            let texture = texture_manager.load(&sprite.texture)?;
            match anim {
                Some(anim) => {
                    self.canvas.copy(&texture, anim.get_src(), render_quad)?;
                }
                None => {
                    self.canvas.copy(&texture, sprite.src, render_quad)?;
                }
            }
        }

        Ok(())
    }

    //renders the tiles in the overworld map
    //takes in the texture manager and the map to be rendered, returns ok if no problems
    pub fn render_overworld_tiles(
        &mut self,
        texture_manager: &mut TextureManager<WindowContext>,
        map: &tilemap::TileMap,
    ) -> Result<(), String> {
        let screen_quad = Rect::new(0, 0, PIXELS_X, PIXELS_Y);
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.fill_rect(screen_quad)?;

        let texture = texture_manager.load("assets/tiles/tilesprites.png")?;

        let top_left = Vec2::from(self.camera.top_left).to_usize(map.size_x);
        let bottom_right = Vec2::from(self.camera.bottom_right).to_usize(map.size_x);

        for i in top_left..bottom_right {
            let render_quad = Rect::new(
                (i % map.size_x) as i32 * TILE_SIZE - self.camera.offset.0,
                (i / map.size_x) as i32 * TILE_SIZE - self.camera.offset.1,
                TILE_SIZE as u32,
                TILE_SIZE as u32,
            );

            if let Some(tile) = map.floor.get(i) {
                if !matches!(tile, FloorTile::NONE) {
                    let src = self.floortile_rects[*tile];
                    self.canvas.copy(&texture, src, render_quad)?
                }
            };
            if let Some(tile) = map.walls.get(i) {
                // check if the tile is empty AND if it is not a front tile (to be rendered after the entities)
                if !matches!(tile, WallTile::NONE) && !map.front_filter.contains(tile) {
                    let src = self.walltile_rects[*tile];
                    self.canvas.copy(&texture, src, render_quad)?
                }
            };
        }

        Ok(())
    }

    //renders the tiles in front of the player on the map
    //takes in the texture manager and the current map, returns ok if no problems
    pub fn render_front_overworld_tiles(
        &mut self,
        texture_manager: &mut TextureManager<WindowContext>,
        map: &tilemap::TileMap,
    ) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));

        let texture = texture_manager.load("assets/tiles/tilesprites.png")?;

        let top_left = Vec2::from(self.camera.top_left).to_usize(map.size_x);
        let bottom_right = Vec2::from(self.camera.bottom_right).to_usize(map.size_x);

        for i in top_left..bottom_right {
            let render_quad = Rect::new(
                (i % map.size_x) as i32 * TILE_SIZE - self.camera.offset.0,
                (i / map.size_x) as i32 * TILE_SIZE - self.camera.offset.1,
                TILE_SIZE as u32,
                TILE_SIZE as u32,
            );

            // render wall tiles in the front_filter after entities are rendered
            if let Some(tile) = map.walls.get(i) {
                if map.front_filter.contains(tile) {
                    let src = self.walltile_rects[*tile];
                    self.canvas.copy(&texture, src, render_quad)?
                }
            };
        }

        Ok(())
    }
}
