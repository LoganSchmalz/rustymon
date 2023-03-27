use enum_map::{enum_map, EnumMap};
use hecs::World;
use sdl2::{
    rect::Rect,
    render::Canvas,
    video::{Window, WindowContext},
};

use crate::{
    constants::TILE_SIZE,
    font_manager::FontManager,
    menu,
    resource_manager::TextureManager,
    tilemap::{self, FloorTile, WallTile},
};

use self::overworld::Camera;

mod battle;
mod menus;
mod overworld;
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

#[derive(Copy, Clone)]
pub enum Transition {
    Fade,
    Win,
    Loss,
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
    floortile_rects: EnumMap<FloorTile, Rect>,
    walltile_rects: EnumMap<WallTile, Rect>,
    pub trans: Transition,
}

impl Renderer {
    pub fn new(canvas: Canvas<Window>) -> Renderer {
        let size = TILE_SIZE as u32;
        let floortile_rects = enum_map! {
            FloorTile::NONE => Rect::new(0,0, size, size),
            FloorTile::GRASS1 => Rect::new(32, 0, size, size),
            FloorTile::GRASS2 => Rect::new(48, 0, size, size),
            FloorTile::WATER1 => Rect::new(16, 64, size, size),
            FloorTile::WGTL => Rect::new(0, 48, size, size),
            FloorTile::WGT => Rect::new(16, 48, size, size),
            FloorTile::WGTR => Rect::new(32, 48, size, size),
            FloorTile::WGL => Rect::new(0, 64, size, size),
            FloorTile::WGR => Rect::new(32, 64, size, size),
            FloorTile::WGBL => Rect::new(0, 80, size, size),
            FloorTile::WGB => Rect::new(16, 80, size, size),
            FloorTile::WGBR => Rect::new(32, 80, size, size),
            FloorTile::GWTL => Rect::new(48, 48, size, size),
            FloorTile::GWTR => Rect::new(80, 48, size, size),
            FloorTile::GWBL => Rect::new(48, 80, size, size),
            FloorTile::GWBR => Rect::new(80, 80, size, size),
            FloorTile::SGTL => Rect::new( 96,48, size, size),
            FloorTile::SGTM => Rect::new( 112,48, size, size),
            FloorTile::SGTR => Rect::new( 128,48, size, size),
            FloorTile::SGML => Rect::new( 96,64, size, size),
            FloorTile::SGC  => Rect::new( 112,64, size, size),
            FloorTile::SGMR => Rect::new( 128,64, size, size),
            FloorTile::SGBL => Rect::new( 96,80, size, size),
            FloorTile::SGBM => Rect::new(112 ,80, size, size),
            FloorTile::SGBR => Rect::new(128,80, size, size),
            FloorTile::FB1 => Rect::new(112, 0, size, size),
            FloorTile::GRASSPATH_V => Rect::new(192,80,size,size),
            FloorTile::GRASSPATH_AB => Rect::new(112,48,size,size),
            FloorTile::GRASSPATH_NB => Rect::new(144,48,size,size),
            FloorTile::GRASSPATH_LB => Rect::new(144,64,size,size),
            FloorTile::GRASSPATH_TB => Rect::new(160,48,size,size),
            FloorTile::GRASSPATH_BB => Rect::new(160,80,size,size),
            FloorTile::GRASSPATH_RB => Rect::new(176,64,size,size),
            FloorTile::GRASSPATH_LU => Rect::new(176,48,size,size),
            FloorTile::GRASSPATH_LD => Rect::new(192,48,size,size),
            FloorTile::GRASSPATH_UR => Rect::new(192,64,size,size),
            FloorTile::GRASSPATH_DL => Rect::new(144,80,size,size),
            FloorTile::GRASSPATH_H  => Rect::new(192,80,size,size),
        };

        let walltile_rects = enum_map! {
            WallTile::NONE => Rect::new(0,0, size, size),
            WallTile::WOODL => Rect::new(128,0,size,size),
            WallTile::WOODR => Rect::new(160,0,size,size),
            WallTile::WOOD => Rect::new(134,0,size,size),
            WallTile::WINDOW => Rect::new(176,0,size,size),
            WallTile::FENCE_L => Rect::new(96,16,size,size),
            WallTile::FENCE_M => Rect::new(112,16,size,size),
            WallTile::FENCE_R => Rect::new(128,16,size,size),
            WallTile::FENCE_S => Rect::new(144,16,size,size),
            WallTile::FENCE_HL => Rect::new(160,16,size,size),
            WallTile::FENCE_HR => Rect::new(176,16,size,size),
            WallTile::FENCE_TR => Rect::new(192,16,size,size),
            WallTile::FENCE_TL => Rect::new(208,16,size,size),
            WallTile::FENCE_DL => Rect::new(224,16,size,size),
            WallTile::FENCE_BL => Rect::new(240,16,size,size),
            WallTile::FENCE_DR => Rect::new(256,16,size,size),
            WallTile::FENCE_BR => Rect::new(272,16,size,size),
            WallTile::TGRASS_1 => Rect::new(0,112,size,size),
            WallTile::TGRASS_2 => Rect::new(16,112,size,size),
            WallTile::TGRASS_3 => Rect::new(32,112,size,size),
            WallTile::TGRASS_4 => Rect::new(48,112,size,size),
            WallTile::TREE_BOTTOM => Rect::new(80,112,size,size),
            WallTile::TREE_TOP => Rect::new(96,112,size,size),
            WallTile::TREES => Rect::new(112,112,size,size),
            WallTile::ROOF_1 => Rect::new(304,0,size,size),
            WallTile::ROOF_2 => Rect::new(320,0,size,size),
            WallTile::ROOF_3 => Rect::new(336,0,size,size),
            WallTile::ROOF_4 => Rect::new(352,0,size,size),
            WallTile::ROOF_5 => Rect::new(288,16,size,size),
            WallTile::ROOF_6 => Rect::new(304,16,size,size),
            WallTile::ROOF_7 => Rect::new(320,16,size,size),
            WallTile::ROOF_8 => Rect::new(336,16,size,size),
            WallTile::ROOF_9 => Rect::new(352,16,size,size),
            WallTile::ROOF_10 => Rect::new(368,16,size,size),
            WallTile::ROOF_11 => Rect::new(288,32,size,size),
            WallTile::ROOF_12 => Rect::new(304,32,size,size),
            WallTile::ROOF_13 => Rect::new(320,32,size,size),
            WallTile::ROOF_14 => Rect::new(336,32,size,size),
            WallTile::ROOF_15 => Rect::new(352,32,size,size),
            WallTile::ROOF_16 => Rect::new(368,32,size,size),
            WallTile::DOOR => Rect::new(96,0,size,size),
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
            floortile_rects,
            walltile_rects,
            trans: Transition::Fade,
        }
    }

    pub fn render_transition(
        &mut self,
        texture_manager: &mut TextureManager<WindowContext>,
        delta_time: f32,
        trans: Transition,
    ) -> Result<bool, String> {
        match trans {
            Transition::Fade => {
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
                        let screen_quad = Rect::new(0, 0, PIXELS_X, PIXELS_Y);
                        let fade_slice = Rect::new(240 * curr_fade_frame, 0, 240, 160);
                        self.canvas.copy(&fade_texture, fade_slice, screen_quad)?;
                        if (FADE_FRAMES as f64 * (1.0 - (self.fade_anim_time / FADE_TIME) as f64)).round()
                            as i32
                            > FADE_FRAMES / 2
                            && !self.did_trans
                        {
                            /*match map.id {
                                0 => {
                                    *map = TileMap::load(1);
                                    obj_man.load_objects(Path::new("maps/map1/"));
                                }
                                1 => {
                                    *map = TileMap::load(0);
                                    obj_man.load_objects(Path::new("maps/map0"));
                                }
                                _ => panic!("Trying to load map that doesn't exist"),
                            }*/
                            return Ok(true);
                        }
                    }
                }
                Ok(false)
            },
            Transition::Win => { //render win screen for set amount of time
                if self.is_fading {
                    self.fade_anim_time -= delta_time;
                    if self.fade_anim_time <= 0.0 {
                        self.is_fading = false; //end transition
                    } else {
                        let texture = texture_manager.load("assets/backgrounds/winscreen.png")?; //load texture for win screen
                        let screen_quad = Rect::new(0, 0, PIXELS_X, PIXELS_Y); //rectangle containing entire screen
                        self.canvas.copy(&texture, None, screen_quad)?; //render win screen texture over whole screen
                    }
                    return Ok(true); //return Ok(true) to indicate success
                }
                Ok(false) //return Ok(false) to indicate mishap in the code
            },
            Transition::Loss => {  //render loss screen for set amount of time
                if self.is_fading {
                    if self.fade_anim_time <= 0.0 {
                        self.is_fading = false; //end transition
                    } else {
                        let texture = texture_manager.load("assets/backgrounds/lossscreen.png")?; //load texture for loss screen
                        let screen_quad = Rect::new(0, 0, PIXELS_X, PIXELS_Y); //rectangle containing entire screen
                        self.canvas.copy(&texture, None, screen_quad)?; //render loss screen texture over whole screen
                    }
                    return Ok(true); //return Ok(true) to indicate success
                }
                Ok(false) //return Ok(false) to indicate mishap in the code
            },
        }
    }

    pub fn render_menus(
        &mut self,
        world: &World,
        texture_manager: &mut TextureManager<WindowContext>,
        font_man: &FontManager,
        menu_man: &menu::MenuManager,
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
                    self.render_bag_menu(menu, world, texture_manager, font_man)?
                }
                menu::Menu::MovesMenu(menu) => {
                    self.render_moves_menu(menu, texture_manager, font_man)?
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
