use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

use crate::object::npc::Character;
use crate::tilemap::Tile;
use crate::object::{self, Object};

use enum_map::{enum_map, EnumMap};
pub struct Sprite<'a> {
    pub texture: &'a Texture<'a>,
    pub src: Rect
}

pub struct Textures<'a> {
    //Sprite sheets
    tilesprites: Texture<'a>,
    objectsprites: Texture<'a>,
    //Main menu assets
    pub main_menu: Texture<'a>,
    pub start_button: Texture<'a>,
    pub load_button: Texture<'a>,
    pub settings_button: Texture<'a>,
    //Transitions
    pub fade_texture: Texture<'a>,
    //Characters
    pub player: Texture<'a>,
    pub dad: Texture<'a>,
    pub jodo: Texture<'a>,
    sika: Texture<'a>,
    //Text Box
    pub text_box: Texture<'a>,
    //Pause Menu
    pub pause_menu: Texture<'a>,
    //Bag Menu
    pub bag_menu: Texture<'a>,
}

impl<'a> Textures<'a> {
    pub fn load(creator: &'a TextureCreator<WindowContext>) -> Self {
        let tilesprites = creator.load_texture("assets/tilesprites.png").unwrap();
        let objectsprites = creator.load_texture("assets/objectsprites.png").unwrap();
        let main_menu = creator.load_texture("assets/titlescreen.png").unwrap();
        let start_button = creator.load_texture("assets/STARTbutton.png").unwrap();
        let load_button = creator.load_texture("assets/SAVELOADbutton.png").unwrap();
        let settings_button = creator.load_texture("assets/SETTINGSbutton.png").unwrap();
        let fade_texture = creator.load_texture("assets/gooWipe.png").unwrap();
        let player = creator.load_texture("assets/nakedcharsprite.png").unwrap();
        let dad = creator.load_texture("assets/nakedcharsprite.png").unwrap();
        let jodo = creator.load_texture("assets/nakedcharsprite.png").unwrap();
        let sika = creator.load_texture("assets/nakedcharsprite.png").unwrap();
        let text_box = creator.load_texture("assets/text_box.png").unwrap();
        let pause_menu = creator.load_texture("assets/pause_menu.png").unwrap();
        let bag_menu = creator.load_texture("assets/bag_menu.png").unwrap();

        Textures {
            tilesprites,
            objectsprites,
            main_menu,
            start_button,
            load_button,
            settings_button,
            fade_texture,
            player,
            dad,
            jodo,
            sika,
            text_box,
            pause_menu,
            bag_menu,
        }
    }
}

pub struct TextureManager<'a> {
    pub textures: Textures<'a>,
    tile_rects: EnumMap<Tile, Rect>,
}

impl TextureManager<'_> {
    pub fn new(textures: Textures) -> TextureManager {
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

        TextureManager {
            textures, tile_rects
        }
    }

    pub fn get_tile(&self, tile: Tile) -> Sprite {
        Sprite { texture: &self.textures.tilesprites, src: self.tile_rects[tile] }
    }

    pub fn get_object(&self, object: &Object) -> Sprite {
        match object {
            Object::Berry(_) => return Sprite { texture: &self.textures.objectsprites, src: Rect::new(0, 0, 16, 16) },
            Object::Door(_) => return Sprite { texture: &self.textures.tilesprites, src: Rect::new(96, 0, 16, 16) },
            Object::NPC(npc) => return self.get_npc(npc),
            //_ => panic!("Bad object request to texture_manager")
        }
    }

    pub fn get_npc(&self, npc: &object::npc::NPC) -> Sprite {
        match npc.character {
            Character::Dad => Sprite { texture: &self.textures.dad, src: npc.get_texture() },
            Character::Jodo => Sprite { texture: &self.textures.jodo, src: npc.get_texture() },
            Character::Sika => Sprite { texture: &self.textures.sika, src: npc.get_texture() },
            //_ => return Sprite { texture: &self.textures.tilesprites, src: Rect::new(96, 0, 16, 16) }
        }
    }
}
