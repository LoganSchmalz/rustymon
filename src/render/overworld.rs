use hecs::World;
use sdl2::{pixels::Color, rect::Rect, video::WindowContext};

use crate::{
    components::{animation::HumanWalkAnimation, sprite::Sprite, Player, Position},
    constants::TILE_SIZE,
    font_manager::FontManager,
    menu,
    resource_manager::TextureManager,
    tilemap::{self, FloorTile, WallTile},
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
    pub fn render_overworld(
        &mut self,
        texture_manager: &mut TextureManager<WindowContext>,
        font_man: &FontManager,
        delta_time: f32,
        world: &World,
        map: &mut tilemap::TileMap,
        menu_man: &mut menu::MenuManager,
    ) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        self.canvas.clear();

        self.update_camera(world)?;
        self.render_overworld_tiles(texture_manager, map)?;
        self.render_entities(world, texture_manager)?;
        self.render_menus(world, texture_manager, font_man, menu_man)?;
        /*if self.is_fading {
            self.render_transition(texture_manager, delta_time, map, obj_man);
        }*/

        self.canvas.present();

        Ok(())
    }

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

    pub fn render_overworld_tiles(
        &mut self,
        texture_manager: &mut TextureManager<WindowContext>,
        map: &tilemap::TileMap,
    ) -> Result<(), String> {
        //TODO: remove next few lines, eventually we should just make the maps big enough to fill in the spaces that you can't walk into with actual tiles
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
                if !matches!(tile, WallTile::NONE) {
                    let src = self.walltile_rects[*tile];
                    self.canvas.copy(&texture, src, render_quad)?
                }
            };
        }

        Ok(())
    }
}
