use enum_map::Enum;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::{fs, path::Path};

use crate::vec2::{Vec2, self};

#[derive(FromPrimitive, ToPrimitive, Debug, Enum, Clone, Copy)]
pub enum FloorTile {
    NONE,
    GRASS1,
    GRASS2,
    WATER1,
    WGTL,
    WGT,
    WGTR,
    WGL,
    WGR,
    WGBL,
    WGB,
    WGBR,
    GWTL,
    GWTR,
    GWBL,
    GWBR,
    SGTL,
    SGTM,
    SGTR,
    SGML,
    SGC,
    SGMR,
    SGBL,
    SGBM,
    SGBR,
    FB1,
    GRASSPATH_V,
    GRASSPATH_AB,
    GRASSPATH_NB,
    GRASSPATH_LB,
    GRASSPATH_TB,
    GRASSPATH_BB,
    GRASSPATH_RB,
    GRASSPATH_LU,
    GRASSPATH_LD,
    GRASSPATH_UR,
    GRASSPATH_DL,
    GRASSPATH_H,
}

#[derive(FromPrimitive, ToPrimitive, PartialEq, Debug, Enum, Clone, Copy)]
pub enum WallTile {
    NONE,
    WOODL,
    WOODR,
    WOOD,
    WINDOW,
    FENCE_L,
    FENCE_M,
    FENCE_R,
    FENCE_S,
    FENCE_HL,
    FENCE_HR,
    FENCE_TR,
    FENCE_TL,
    FENCE_DL,
    FENCE_BL,
    FENCE_DR,
    FENCE_BR,
    TGRASS_1,
    TGRASS_2,
    TGRASS_3,
    TGRASS_4,
    TREE_BOTTOM,
    TREE_TOP,
    TREES,
    ROOF_1,
    ROOF_2,
    ROOF_3,
    ROOF_4,
    ROOF_5,
    ROOF_6,
    ROOF_7,
    ROOF_8,
    ROOF_9,
    ROOF_10,
    ROOF_11,
    ROOF_12,
    ROOF_13,
    ROOF_14,
    ROOF_15,
    ROOF_16,
    DOOR,
}

#[derive(FromPrimitive, ToPrimitive, PartialEq, Debug)]
pub enum CollisionTile {
    None,
    Wall,
    Door,
}

pub struct TileMap {
    pub size_x: usize,
    pub size_y: usize,
    pub floor: Vec<FloorTile>,
    pub walls: Vec<WallTile>,
    pub front_filter: Vec<WallTile>,
    pub collision: Vec<CollisionTile>,
    pub id: i32,
}

impl TileMap {
    pub fn load(id: i32) -> TileMap {
        let str = String::from("maps/map") + &id.to_string() + "/";
        let mapfolder = Path::new(str.as_str());
        println!("{}", mapfolder.to_str().unwrap());

        let dim: Vec<usize> = fs::read_to_string(mapfolder.join("dim.txt"))
            .unwrap_or_else(|_| panic!("{}dim.txt not found", mapfolder.to_str().unwrap()))
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
            .unwrap_or_else(|_| panic!("{}floor.txt not found", mapfolder.to_str().unwrap()))
            .split_whitespace()
            .map(|x| {
                FromPrimitive::from_u32(x.parse::<u32>().expect("Not an integer!"))
                    .expect("Invalid floor tile")
            })
            .collect();

        if floor.len() != (size_x * size_y) {
            panic!(
                "{}floor.txt does not contain {} values",
                mapfolder.to_str().unwrap(),
                size_x * size_y,
            )
        }

        let walls: Vec<WallTile> = fs::read_to_string(mapfolder.join("walls.txt"))
            .unwrap_or_else(|_| panic!("{}walls.txt not found", mapfolder.to_str().unwrap()))
            .split_whitespace()
            .map(|x| {
                FromPrimitive::from_u32(x.parse::<u32>().expect("Not an integer!"))
                    .expect("Invalid wall tile")
            })
            .collect();

        if walls.len() != (size_x * size_y) {
            panic!(
                "{}walls.txt does not contain {} values",
                mapfolder.to_str().unwrap(),
                size_x * size_y,
            )
        }

        // filter out wall tiles that you cannot walk behind (only front wall tiles)
        let front_filter = vec![
            WallTile::ROOF_1,
            WallTile::ROOF_2,
            WallTile::ROOF_3,
            WallTile::ROOF_4,
            WallTile::ROOF_5,
            WallTile::ROOF_10,
            WallTile::ROOF_11,
            WallTile::ROOF_16,
            WallTile::TREE_TOP
            ];

        let collision: Vec<CollisionTile> = fs::read_to_string(mapfolder.join("collision.txt"))
            .unwrap_or_else(|_| panic!("{}collision.txt not found", mapfolder.to_str().unwrap()))
            .split_whitespace()
            .map(|x| {
                FromPrimitive::from_u32(x.parse::<u32>().expect("Not an integer!"))
                    .expect("Invalid collision tile")
            })
            .collect();
        if collision.len() != (size_x * size_y) {
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
            front_filter,
            collision,
            id,
        }
    }

    pub fn check_collision(&self, pos: Vec2) -> bool {
        !matches!(
            self.collision.get(pos.to_usize(self.size_x)),
            Some(CollisionTile::None)
        ) || pos.0 < 0.0
            || pos.1 < 0.0
            || pos.0 >= self.size_x as f32
            || pos.1 >= self.size_y as f32
    }

    pub fn check_encounter(&self, pos: Vec2) -> bool {
        //if check position is encounter tile
        if let Some(tile) = self.walls.get(pos.to_usize(self.size_x)) {
            matches!(
                tile,
                WallTile::TGRASS_1
                    | WallTile::TGRASS_2
                    | WallTile::TGRASS_3
                    | WallTile::TGRASS_4
            )
        } else {
            false
        }
        //true
    }
}
