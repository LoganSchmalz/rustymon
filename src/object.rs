use crate::menu::textbox::Textbox;
use crate::menu::{self, MenuManager};
use crate::render;
use crate::tilemap;
use crate::tilemap::CollisionTile;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::ops::Deref;
use std::{fs, path::Path};

mod berry;
mod door;
mod npc;

use self::berry::Berry;
use self::door::Door;
use self::npc::{Character, NPC};

pub const OBJECT_COUNT: usize = 5;

#[enum_delegate::register]
pub trait TObject {
    fn pos(&self) -> (f64, f64);
    fn interact(&self, renderer: &mut render::Renderer, menu_man: &mut MenuManager) -> bool; //returns if obj should be removed from map
    fn update(&self);
}

//#[derive(FromPrimitive, ToPrimitive)]
#[enum_delegate::implement(TObject)]
pub enum Object {
    //None, //0
    Berry(Berry), //1
    Door(Door), //2
    NPC(NPC)
    /*
    Dad(NPC), //3
    Jodo(NPC), //4
    Sika(NPC), //5
    */
}

pub struct ObjectManager {
    pub objects: Vec<Object>,
}

impl ObjectManager {
    pub fn new() -> ObjectManager {
        ObjectManager { objects: vec![] }
    }

    pub fn load_objects(&mut self, mapfolder: &Path) {
        self.objects.clear();

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

        let objects: Vec<u32> = fs::read_to_string(mapfolder.join("objects.txt"))
            .expect(&format!(
                "{}objects.txt not found",
                mapfolder.to_str().unwrap()
            ))
            .split_whitespace()
            .map(|x| {
                x.parse::<u32>().expect("Not an integer!")
            })
            .collect();
        
        //println!("{:?}", objects);

        //todo: improve object loading, stop loading from map by coordinates and instead load from list
        for (idx, obj) in objects.iter().enumerate() {
            let pos = ((idx % size_x) as f64, (idx / size_x) as f64);

            match obj {
                1 => {
                    self.objects.push(Object::Berry(Berry::new(pos)));
                }
                2 => {
                    self.objects.push(Object::Door(Door::new(pos, (0,2.0,1.0))));
                }
                3 => { self.objects.push(Object::NPC(NPC::new(pos, Character::Dad))); }
                4 => { self.objects.push(Object::NPC(NPC::new(pos, Character::Jodo))); }
                5 => { self.objects.push(Object::NPC(NPC::new(pos, Character::Sika))); }
                _ => {}
            }
        }
    }

    pub fn interact(
        &mut self,
        pos: (f64, f64),
        renderer: &mut render::Renderer,
        menu_man: &mut menu::MenuManager,
    ) {
        for (idx, obj) in self.objects.iter().enumerate() {
            if pos == obj.pos() {
                if obj.interact(renderer, menu_man) {
                    self.objects.remove(idx);
                }
                break;
            }
        }
    }
}
