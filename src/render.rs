use enum_map::{enum_map, EnumMap};
use hecs::World;
use sdl2::{
    pixels::Color,
    rect::Rect,
    render::Canvas,
    video::{Window, WindowContext},
};

use crate::{
    coordinate::Coordinate,
    engine_structures::{
        components::{HumanWalkAnimation, Player, Position, Sprite},
        humanoid_properties,
    },
    font_manager::FontManager,
    menu,
    resource_manager::{self, TextureManager},
    tilemap::{self, Tile},
    TILE_SIZE,
};

mod render_menus;
mod resize;

pub const PIXELS_X: u32 = 240;
pub const PIXELS_Y: u32 = 160;
const FADE_FRAMES: i32 = 14;
const FADE_TIME: f32 = FADE_FRAMES as f32 * 64.0;
const _TEXT_TIME: f32 = 500.0;

#[derive(PartialEq)]
pub enum DisplayScreen {
    _MainMenu,
    _OverWorld,
}

#[derive(Default)]
pub struct Camera {
    offset: (i32, i32),
    top_left: (i32, i32),
    bottom_right: (i32, i32),
}

pub struct Renderer {
    window_x: u32,
    window_y: u32,
    old_window_x: u32,
    old_window_y: u32,
    canvas: Canvas<Window>,
    pub is_fading: bool,
    did_trans: bool,
    fade_anim_time: f32,
    camera: Camera,
    tile_rects: EnumMap<Tile, Rect>,
}

impl Renderer {
    pub fn new(canvas: Canvas<Window>) -> Renderer {
        let tile_rects = enum_map! {
            Tile::NONE => Rect::new(0,0,0,0),
            Tile::GRASS1 => Rect::new(32, 0, 16, 16),
            Tile::GRASS2 => Rect::new(48, 0, 16, 16),
            Tile::WATER1 => Rect::new(16, 64, 16, 16),
            Tile::WGTL => Rect::new(0, 48, 16, 16),
            Tile::WGT => Rect::new(16, 48, 16, 16),
            Tile::WGTR => Rect::new(32, 48, 16, 16),
            Tile::WGL => Rect::new(0, 64, 16, 16),
            Tile::WGR => Rect::new(32, 64, 16, 16),
            Tile::WGBL => Rect::new(0, 80, 16, 16),
            Tile::WGB => Rect::new(16, 80, 16, 16),
            Tile::WGBR => Rect::new(32, 80, 16, 16),
            Tile::GWTL => Rect::new(48, 48, 16, 16),
            Tile::GWTR => Rect::new(80, 48, 16, 16),
            Tile::GWBL => Rect::new(48, 80, 16, 16),
            Tile::GWBR => Rect::new(80, 80, 16, 16),
            Tile::FB1 => Rect::new(112, 0, 16, 16),
            Tile::WOODL => Rect::new(128, 0, 16, 16),
            Tile::WOODR => Rect::new(160, 0, 16, 16),
        };

        Renderer {
            window_x: PIXELS_X,
            window_y: PIXELS_Y,
            old_window_x: PIXELS_X,
            old_window_y: PIXELS_Y,
            canvas,
            is_fading: false,
            did_trans: false,
            fade_anim_time: FADE_TIME,
            camera: Camera::default(),
            tile_rects,
        }
    }

    pub fn render_overworld_tiles(
        &mut self,
        texture_manager: &mut resource_manager::TextureManager<WindowContext>,
        map: &tilemap::TileMap,
    ) -> Result<(), String> {
        //TODO: remove next few lines, eventually we should just make the maps big enough to fill in the spaces that you can't walk into with actual tiles
        let screen_quad = Rect::new(0, 0, PIXELS_X, PIXELS_Y);
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.fill_rect(screen_quad)?;

        let texture = texture_manager.load("assets/tiles/tilesprites.png")?;

        let top_left = Coordinate::from(self.camera.top_left).to_usize(map.size_x);
        let bottom_right = Coordinate::from(self.camera.bottom_right).to_usize(map.size_x);

        for i in top_left..bottom_right {
            let render_quad = Rect::new(
                (i % map.size_x) as i32 * TILE_SIZE - self.camera.offset.0,
                (i / map.size_x) as i32 * TILE_SIZE - self.camera.offset.1,
                TILE_SIZE as u32,
                TILE_SIZE as u32,
            );

            if let Some(tile) = map.floor.get(i) {
                let src = self.tile_rects[*tile];
                self.canvas.copy(&texture, src, render_quad)?
            };
            if let Some(tile) = map.walls.get(i) {
                let src = self.tile_rects[*tile];
                self.canvas.copy(&texture, src, render_quad)?
            };
        }

        Ok(())
    }

    pub fn render_transition(
        &mut self,
        texture_manager: &mut TextureManager<WindowContext>,
        delta_time: &f32,
        map: &mut tilemap::TileMap,
    ) -> Result<bool, String> {
        if self.is_fading {
            let fade_texture = texture_manager.load("assets/transitions/gooWipe.png")?;

            self.fade_anim_time -= delta_time;
            if self.fade_anim_time <= 0.0 {
                self.is_fading = false;
            } else {
                //might be timing issues here (starts at -_delta_time instead of the actual beginning)
                let curr_fade_frame: i32 = (FADE_FRAMES as f64
                    * (1.0 - (self.fade_anim_time / FADE_TIME) as f64))
                    .round() as i32;
                let screen_quad = Rect::new(0, 0, PIXELS_X, PIXELS_Y); //TODO: change height and width of screen_quad to not require math
                let fade_slice = Rect::new(240 * curr_fade_frame, 0, 240, 160);
                self.canvas.copy(&fade_texture, fade_slice, screen_quad)?;
                if (FADE_FRAMES as f64 * (1.0 - (self.fade_anim_time / FADE_TIME) as f64)).round()
                    as i32
                    > FADE_FRAMES / 2
                    && !self.did_trans
                {
                    match map.id {
                        /*0 => {
                            *map = TileMap::load(1);
                            obj_man.load_objects(Path::new("maps/map1/"));
                        }
                        1 => {
                            *map = TileMap::load(0);
                            obj_man.load_objects(Path::new("maps/map0"));
                        }*/
                        _ => panic!("Trying to load map that doesn't exist"),
                    }
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }

    pub fn render_menus(
        &mut self,
        texture_manager: &mut TextureManager<WindowContext>,
        font_man: &FontManager,
        menu_man: &mut menu::MenuManager,
    ) -> Result<(), String> {
        for menu_item in menu_man.menus.iter() {
            match menu_item {
                menu::Menu::MainMenu(menu) => {
                    self.render_main_menu(menu, texture_manager, font_man)?
                }
                menu::Menu::Textbox(textbox) => {
                    self.render_textbox(textbox, texture_manager, font_man)?
                }
                menu::Menu::PauseMenu(menu) => {
                    self.render_pause_menu(menu, texture_manager, font_man)?
                }
                menu::Menu::BagMenu(menu) => {
                    self.render_bag_menu(menu, texture_manager, font_man)?
                }
            }
        }

        Ok(())
    }

    pub fn render(
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
        self.render_menus(texture_manager, font_man, menu_man)?;
        /*if self.is_fading {
            self.render_transition(texture_manager, delta_time, map, obj_man);
        }*/

        self.canvas.present();

        Ok(())
    }

    pub fn update_camera(&mut self, world: &World) -> Result<(), String> {
        let mut q = world.query::<(&Player, &Position)>();
        let (_, (_player, Position(pos))) = q.iter().next().ok_or("No player found")?;

        let offset = (
            (pos.0 * TILE_SIZE as f32).round() as i32
                - (PIXELS_X / 2 - humanoid_properties::WIDTH / 2) as i32,
            (pos.1 * TILE_SIZE as f32).round() as i32
                - (PIXELS_Y / 2 - humanoid_properties::HEIGHT / 2) as i32,
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
        texture_manager: &mut resource_manager::TextureManager<WindowContext>,
    ) -> Result<(), String> {
        let mut entity_query = world.query::<(&Position, &Sprite, Option<&HumanWalkAnimation>)>();

        let mut list = entity_query
            .iter()
            .filter(|(_, (Position(c), ..))| {
                Coordinate::from(self.camera.top_left) <= *c
                    && *c <= Coordinate::from(self.camera.bottom_right)
            })
            .collect::<Vec<_>>();
        list.sort_by(|(_, (Position(c1), ..)), (_, (Position(c2), ..))| {
            c1.partial_cmp(c2).unwrap()
        });

        for (_, (Position(Coordinate(x, y)), sprite, anim)) in list {
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

    pub fn play_fade(&mut self) {
        //TODO LOCK PLAYER WHEN FADE IS PLAYING SO THEY CANT WALK ON WATER
        self.is_fading = true;
        self.did_trans = false;
        self.fade_anim_time = FADE_TIME;
    }
}
