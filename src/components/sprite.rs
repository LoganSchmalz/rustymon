/****************************************************/
// Created by: Logan Schmalz
// Description: Sprite definitions for characters and items
/****************************************************/

use sdl2::rect::Rect;

pub struct Sprite {
    pub texture: String,
    pub src: Rect,
    pub shift_x: i32,
    pub shift_y: i32,
}

impl Sprite {
    //the default for character sprites is to shift them up a bit so they look like they are centered on the tile
    //initializes a character Sprite
    //takes in a string that is used to find the texture of the character (the file location)
    pub fn character(str: String) -> Self {
        Self {
            texture: str,
            src: Rect::new(0, 0, 16, 20),
            shift_x: 0,
            shift_y: -8,
        }
    }

    //initializes a berry Sprite
    pub fn berry() -> Self {
        Self {
            texture: String::from("assets/tiles/objectsprites.png"),
            src: Rect::new(32, 0, 16, 16),
            shift_x: 0,
            shift_y: 0,
        }
    }
}

impl Default for Sprite {
    //creates a default Sprite
    fn default() -> Self {
        Self {
            texture: String::from("assets/char-sprites/augosprite.png"),
            src: Rect::new(0, 0, 16, 20),
            shift_x: 0,
            shift_y: 0,
        }
    }
}
