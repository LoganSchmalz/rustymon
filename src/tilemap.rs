use enum_map::Enum;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::{fs, path::Path};

#[derive(FromPrimitive, ToPrimitive, Debug, Enum, Clone)]
pub enum Tile {
    NONE,   //0
    GRASS1, //1
    GRASS2, //2
    WATER1, //3
    WGTL,   //4
    WGT,    //5
    WGTR,   //6
    WGR,    //7
    WGBR,   //8
    WGB,    //9
    WGBL,   //10
    WGL,    //11
    GWTL,   //12
    GWTR,   //13
    GWBR,   //14
    GWBL,   //15
    FB1,    //16
    WOODL,  //17
    WOODR,  //18
}

#[derive(FromPrimitive, ToPrimitive)]
pub enum CollisionTile {
    NONE,
    WALL,
    DOOR,
}

pub struct TileMap {
    pub size_x: usize,
    pub size_y: usize,
    pub floor: Vec<Tile>,
    pub walls: Vec<Tile>,
    pub collision: Vec<CollisionTile>,
    pub map_id: i32,
}

impl TileMap {
    pub fn load(mapfolder: &Path, id: i32) -> TileMap {
        let map_id = id;
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

        let floor: Vec<Tile> = fs::read_to_string(mapfolder.join("floor.txt"))
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

        let walls: Vec<Tile> = fs::read_to_string(mapfolder.join("walls.txt"))
            .expect(&format!(
                "{}walls.txt not found",
                mapfolder.to_str().unwrap()
            ))
            .split_whitespace()
            .map(|x| {
                FromPrimitive::from_u32(x.parse::<u32>().expect("Not an integer!"))
                    .expect("Invalid wall tile")
            })
            .collect();

        if floor.len() != (size_x * size_y) as usize {
            panic!(
                "{}walls.txt does not contain {} values",
                mapfolder.to_str().unwrap(),
                size_x * size_y,
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
            walls,
            collision,
            map_id,
        }
    }
}
