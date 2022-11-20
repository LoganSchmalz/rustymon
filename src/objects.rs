use crate::menu;
use crate::menu::textbox::Textbox;
use crate::tilemap;
use crate::tilemap::ObjectTile;
use crate::tilemap::CollisionTile;
use crate::render;

pub fn object_interact(pos: usize, map: &mut tilemap::TileMap, renderer: &mut render::Renderer, menu_man: &mut menu::MenuManager) {
    match map.objects.get(pos) {
        Some(tilemap::ObjectTile::BERRY) => {
            map.objects[pos] = ObjectTile::NONE;
            map.collision[pos] = CollisionTile::NONE;
            menu_man.open_menu(Box::new(Textbox::new("Don't eat me!".to_string())));
        }

        Some(tilemap::ObjectTile::DOOR) => {
            renderer.play_fade();
        }

        _ => { }
    }
}