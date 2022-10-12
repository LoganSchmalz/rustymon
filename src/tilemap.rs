use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::{fs, path::Path};

#[derive(FromPrimitive, ToPrimitive, Debug)]
pub enum FloorTile {
    GRASS1,
    GRASS2,
	WATER1
}

#[derive(FromPrimitive, ToPrimitive)]
pub enum ObjectTile {
    NONE,
}

#[derive(FromPrimitive, ToPrimitive)]
pub enum CollisionTile {
    NONE,
    WALL,
}

pub struct TileMap {
    pub size_x: usize,
    pub size_y: usize,
    pub floor: Vec<FloorTile>,
    pub objects: Vec<ObjectTile>,
    pub collision: Vec<CollisionTile>,
}

pub fn load_tilemap(mapfolder: &Path) -> TileMap {
    let dim: Vec<usize> = fs::read_to_string(mapfolder.join("dim.txt"))
        .expect(&format!("{}dim.txt not found", mapfolder.to_str().unwrap()))
        .split_whitespace()
        .map(|x| x.parse::<usize>().expect("Not an integer!"))
        .collect();

    if dim.len() != 2 {
        panic!(
            "{}dim.txt does not contain exactly 2 values",
            mapfolder.to_str().unwrap()
        )
    }
    let size_x = dim.get(0).unwrap();
    let size_y = dim.get(1).unwrap();

    let floor: Vec<FloorTile> = fs::read_to_string(mapfolder.join("floor.txt"))
        .expect(&format!(
            "{}floor.txt not found",
            mapfolder.to_str().unwrap()
        ))
        .split_whitespace()
        .map(|x| {
            FromPrimitive::from_u32(x.parse::<u32>().expect("Not an integer!"))
                .expect("Invalid floor tile")
        })
        .collect();

    if floor.len() != (size_x * size_y) as usize {
        panic!(
            "{}floor.txt does not contain {} values",
            mapfolder.to_str().unwrap(),
            size_x * size_y,
        )
    }

    let objects: Vec<ObjectTile> = fs::read_to_string(mapfolder.join("objects.txt"))
        .expect(&format!(
            "{}objects.txt not found",
            mapfolder.to_str().unwrap()
        ))
        .split_whitespace()
        .map(|x| {
            FromPrimitive::from_u32(x.parse::<u32>().expect("Not an integer!"))
                .expect("Invalid object tile")
        })
        .collect();

    if objects.len() != (size_x * size_y) as usize {
        panic!(
            "{}objects.txt does not contain {} values",
            mapfolder.to_str().unwrap(),
            size_x * size_y
        )
    }

    let collision: Vec<CollisionTile> = fs::read_to_string(mapfolder.join("collision.txt"))
        .expect(&format!(
            "{}collision.txt not found",
            mapfolder.to_str().unwrap()
        ))
        .split_whitespace()
        .map(|x| {
            FromPrimitive::from_u32(x.parse::<u32>().expect("Not an integer!"))
                .expect("Invalid collision tile")
        })
        .collect();
    if collision.len() != (size_x * size_y) as usize {
        panic!(
            "{}collision.txt does not contain {} values",
            mapfolder.to_str().unwrap(),
            size_x * size_y
        )
    }

	TileMap {
		size_x: *size_x,
		size_y: *size_y,
		floor,
		objects,
		collision
	}
}
