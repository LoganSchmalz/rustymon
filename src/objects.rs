use crate::tilemap;
use crate::tilemap::ObjectTile;
use crate::tilemap::CollisionTile;
use crate::render;

pub fn object_interact(pos: usize, map: &mut tilemap::TileMap, renderer: &mut render::Renderer) {
    match map.objects.get(pos) {
        Some(tilemap::ObjectTile::BERRY) => {
            renderer.play_text();
            map.objects[pos] = ObjectTile::NONE;
            map.collision[pos] = CollisionTile::NONE;
        }

        Some(tilemap::ObjectTile::DOOR) => {
            renderer.play_fade();
        }

        _ => { }
    }
}