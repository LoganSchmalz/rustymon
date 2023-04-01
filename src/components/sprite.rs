use sdl2::rect::Rect;

pub struct Sprite {
    pub texture: String,
    pub src: Rect,
    pub shift_x: i32,
    pub shift_y: i32,
}

impl Sprite {
    pub fn character(str: String) -> Self {
        Self {
            texture: str,
            src: Rect::new(0, 0, 16, 20),
            shift_x: 0,
            shift_y: -8,
        }
    }

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
    fn default() -> Self {
        Self {
            texture: String::from("assets/char-sprites/augosprite.png"),
            src: Rect::new(0, 0, 16, 20),
            shift_x: 0,
            shift_y: 0,
        }
    }
}
