use crate::tilemap;
use crate::tilemap::ObjectTile;
use crate::tilemap::CollisionTile;

pub fn object_interact(pos: usize, map: &mut tilemap::TileMap) {
    match map.objects.get(pos) {
        Some(tilemap::ObjectTile::BERRY) => {
            map.objects[pos] = ObjectTile::NONE;
            map.collision[pos] = CollisionTile::NONE;
        }

        Some(tilemap::ObjectTile::DOOR) => {

        }

        _ => { }
    }
}