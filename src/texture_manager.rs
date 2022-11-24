use crate::tilemap::{FloorTile, TILE_COUNT};
use crate::objects;

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
    fade_texture: Texture<'a>,
    //Characters
    player: Texture<'a>,
    dad: Texture<'a>,
    jodo: Texture<'a>,
    sika: Texture<'a>,
    //Text Box
    pub text_box: Texture<'a>,
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
        let player = creator.load_texture("assets/newcharsprite.png").unwrap();
        let dad = creator.load_texture("assets/dadcharsprite.png").unwrap();
        let jodo = creator.load_texture("assets/jodocharsprite.png").unwrap();
        let sika = creator.load_texture("assets/sikacharsprite.png").unwrap();
        let text_box = creator.load_texture("assets/text_box.png").unwrap();

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
        }
    }
}

pub struct TextureManager {
    textures: Texture,
    tile_rects: Vec<Rect>,
    object_rects: Vec<Rect>
}

impl TextureManager {
    pub fn new(textures: Texture) -> TextureManager {
        let tile_rects = Vec<Rect>::with_capacity(tilemap::TILE_COUNT);
        tile_rects[to_usize(FloorTile::GRASS1)] = Rect::new(32, 0, 16, 16);
        tile_rects[to_usize(FloorTile::GRASS2)] = Rect::new(48, 0, 16, 16);
        tile_rects[to_usize(FloorTile::WATER1)] = Rect::new(16, 64, 16, 16);
        tile_rects[to_usize(FloorTile::WGTL)] = Rect::new(0, 48, 16, 16);
        tile_rects[to_usize(FloorTile::WGT)] = Rect::new(16, 48, 16, 16);
        tile_rects[to_usize(FloorTile::WGTR)] = Rect::new(32, 48, 16, 16);
        tile_rects[to_usize(FloorTile::WGL)] = Rect::new(0, 64, 16, 16);
        tile_rects[to_usize(FloorTile::WGR)] = Rect::new(32, 64, 16, 16);
        tile_rects[to_usize(FloorTile::WGBL)] = Rect::new(0, 80, 16, 16);
        tile_rects[to_usize(FloorTile::WGB)] = Rect::new(16, 80, 16, 16);
        tile_rects[to_usize(FloorTile::WGBR)] = Rect::new(32, 80, 16, 16);
        tile_rects[to_usize(FloorTile::GWTL)] = Rect::new(48, 48, 16, 16);
        tile_rects[to_usize(FloorTile::GWTR)] = Rect::new(80, 48, 16, 16);
        tile_rects[to_usize(FloorTile::GWBL)] = Rect::new(48, 80, 16, 16);
        tile_rects[to_usize(FloorTile::GWBR)] = Rect::new(80, 80, 16, 16);
        tile_rects[to_usize(FloorTile::FB1)] = Rect::new(112, 0, 16, 16);

        let object_rects = Vec<Rect>::with_capacity(objects::OBJECT_COUNT);
        tile_rects[to_usize(ObjectTile::BERRY)] = Rect::new(0, 0, 16, 16);
        tile_rects[to_usize(ObjectTile::DOOR)] = Rect::new(96, 0, 16, 16);
        tile_rects[to_usize(ObjectTile::WOODL)] = Rect::new(128, 0, 16, 16);
        tile_rects[to_usize(ObjectTile::WOODR)] = Rect::new(160, 0, 16, 16);
        
        TextureManager {
            textures, tile_rects, object_rects
        }
    }

    pub fn get_tile(&self, tile: &FloorTile) -> Rect {
        tile_rects[to_usize(tile)]
    }

    pub fn get_object(&self, object: &ObjectTile) -> (&Texture, Rect) {
        if object == ObjectTile::BERRY {
            return (&self.textures.objectsprites, obj_rects[to_usize(ObjectTile::BERRY)]);
        } else {
            match object {
                ObjectTile::DOOR => return (&self.textures.tilesprites, obj_rects[to_usize(ObjectTile::DOOR)]),
                ObjectTile::WOODL => return (&self.textures.tilesprites, obj_rects[to_usize(ObjectTile::WOODL)]),
                ObjectTile::WOODR => return (&self.textures.tilesprites, obj_rects[to_usize(ObjectTile::WOODR)]),
                _ => panic!("Bad object request to texture_manager")
            }
        }
    }
}

/*match map.floor.get(i + j * map.size_x) {
    Some(tilemap::FloorTile::GRASS1) => canvas
        .copy(&textures.tilesprites, tile_rects.g1, render_quad)
        .unwrap(),
    Some(tilemap::FloorTile::GRASS2) => canvas
        .copy(&textures.tilesprites, tile_rects.g2, render_quad)
        .unwrap(),
    Some(tilemap::FloorTile::WATER1) => canvas
        .copy(&textures.tilesprites, tile_rects.w1, render_quad)
        .unwrap(),
    Some(tilemap::FloorTile::WGTL) => canvas
        .copy(&textures.tilesprites, tile_rects.wg_tl, render_quad)
        .unwrap(),
    Some(tilemap::FloorTile::WGT) => canvas
        .copy(&textures.tilesprites, tile_rects.wg_t, render_quad)
        .unwrap(),
    Some(tilemap::FloorTile::WGTR) => canvas
        .copy(&textures.tilesprites, tile_rects.wg_tr, render_quad)
        .unwrap(),
    Some(tilemap::FloorTile::WGR) => canvas
        .copy(&textures.tilesprites, tile_rects.wg_r, render_quad)
        .unwrap(),
    Some(tilemap::FloorTile::WGBR) => canvas
        .copy(&textures.tilesprites, tile_rects.wg_br, render_quad)
        .unwrap(),
    Some(tilemap::FloorTile::WGB) => canvas
        .copy(&textures.tilesprites, tile_rects.wg_b, render_quad)
        .unwrap(),
    Some(tilemap::FloorTile::WGBL) => canvas
        .copy(&textures.tilesprites, tile_rects.wg_bl, render_quad)
        .unwrap(),
    Some(tilemap::FloorTile::WGL) => canvas
        .copy(&textures.tilesprites, tile_rects.wg_l, render_quad)
        .unwrap(),
    Some(tilemap::FloorTile::GWTL) => canvas
        .copy(&textures.tilesprites, tile_rects.gw_tl, render_quad)
        .unwrap(),
    Some(tilemap::FloorTile::GWTR) => canvas
        .copy(&textures.tilesprites, tile_rects.gw_tr, render_quad)
        .unwrap(),
    Some(tilemap::FloorTile::GWBR) => canvas
        .copy(&textures.tilesprites, tile_rects.gw_br, render_quad)
        .unwrap(),
    Some(tilemap::FloorTile::GWBL) => canvas
        .copy(&textures.tilesprites, tile_rects.gw_bl, render_quad)
        .unwrap(),
    Some(tilemap::FloorTile::FB1) => canvas
        .copy(&textures.tilesprites, tile_rects.fb1, render_quad)
        .unwrap(),
    None => {}
}; */
